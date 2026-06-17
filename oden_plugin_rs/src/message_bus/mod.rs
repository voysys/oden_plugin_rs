//! Generic 1↔N message bus shared between plugins.
//!
//! # Design
//!
//! One *owner* plugin creates a `BusOwner` under a well-known UUID. Any other
//! plugin in the same process opens a `BusClient` against that UUID. Clients can
//! `subscribe` to named messages broadcast by the owner (downstream) and `send`
//! messages back to the owner (upstream).
//!
//! # ABI stability across toolchains and allocators
//!
//! The shared state lives behind a vtable published alongside it. Every function
//! pointer in that vtable runs in the host executable's translation unit, so one `std`
//! and global allocator own the bus's internal `HashMap`, `Mutex`, `mpsc` channels, and
//! queued message `Vec<u8>`s. Plugins only touch `#[repr(C)]` POD across the boundary:
//! borrowed `BusStr`/`BusBytes` (input) and caller-allocated output buffers. No
//! `Vec`/`String`/`Arc` ever flows between sides, so a client built with a different
//! Rust toolchain or `#[global_allocator]` than the owner is fine — there is no
//! cross-boundary `free`.
//!
//! # Receive contract
//!
//! The ABI's receive functions take caller-allocated output buffers. If the buffers are
//! too small, the bus writes the required sizes and returns `BufferTooSmall` *without
//! popping the message*; the same message is delivered on retry once buffers are big
//! enough. The safe Rust facade in this module hides that retry loop, so users only
//! see `Option<OwnedMessage>`.
//!
//! # Late subscribers
//!
//! A broadcast that matches no live subscription is parked inside the bus (up to 128
//! messages) and handed, in order, to the first later subscription whose name filter
//! matches. Parked messages that don't fit in the claiming subscription's queue are
//! dropped — impossible with the default queue capacity (128, matching the parked
//! buffer), possible with a smaller explicit capacity. This mirrors the old
//! named-channel behavior where messages sent before
//! the consumer attached waited in the queue, and covers the window between a
//! webview page connecting and a client plugin's first subscribe (startup, project
//! load, bus reconnect). A message that matched at least one subscription at
//! broadcast time is never re-delivered to subscriptions created later.
//!
//! # Lifecycle: the bus outlives owners and clients
//!
//! The bus under a UUID is a process-lifetime mailbox living in the host
//! executable. The first endpoint to dial a UUID — owner or client — allocates it
//! through the host's factory record (see [`host`]) and publishes its address in
//! the discovery record; every later [`BusOwner::open`] or [`BusClient::open`]
//! for the same UUID adopts that same bus. It is never torn down: subscriptions,
//! queued messages, and pending receive futures all stay valid while the owner
//! plugin is unloaded, hot-reloaded, or restarted. Clients subscribe once and
//! simply see no messages while no owner is broadcasting; upstream sends wait in
//! the bounded upstream queue for the next owner to drain them. There is no
//! liveness state to poll.
//!
//! The `BusState` allocation is **never freed**. Its address is embedded in the
//! discovery record published through `SettingsApi`, which exposes no way to retract a
//! record, so freeing would leave a dangling pointer for a later `open` to
//! dereference. It is leaked for the process lifetime — one allocation per UUID —
//! mirroring `named_shared_data`'s leaked holder.
//!
//! Because the bus vtable and `BusState` live in the host executable, they survive
//! any plugin DLL/SO unload. A host that publishes no factory record cannot run
//! buses — `open` panics rather than allocating the bus inside the
//! plugin's own DLL, where an unload would unmap it under live users. That unload
//! hazard still exists for `named_mpmc_channel` and `named_shared_data`, whose
//! state lives in the owning plugin.
//!
//! One hazard remains for the bus: wake callbacks registered by
//! [`Subscription::recv_async`] / [`BusOwner::recv_upstream_async`] point into the
//! plugin that polled the future. Dropping the future unregisters them, so this only
//! bites a plugin that leaks a pending `RecvFuture` past its own unload.

pub mod abi;
pub mod host;
mod imp;
#[cfg(test)]
mod tests;

use crate::{math::Uuid, SettingsApi};
use std::{
    ffi::c_void,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    time::Duration,
};

use abi::{BusBytes, BusHandle, BusStatus, BusStr, BusSubHandle};

/// A message delivered from / received by the bus, owned by the caller's allocator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedMessage {
    /// Routing name. UTF-8.
    pub name: String,
    /// Opaque payload bytes (often JSON, but the bus does not interpret them).
    pub payload: Vec<u8>,
}

/// Error returned by [`BusOwner::broadcast`] and [`BusClient::send`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    /// A bounded queue was full. For `send` this means the upstream queue. For
    /// `broadcast` it means at least one subscriber's queue was full — the
    /// broadcast still delivered to subscribers whose queues had space.
    QueueFull,
}

// ---------------------------------------------------------------------------
// Inner shared handle. Refcounted via the bus vtable.
// ---------------------------------------------------------------------------

struct BusInner {
    handle: BusHandle,
}

impl BusInner {
    fn new(handle: BusHandle) -> Arc<Self> {
        Arc::new(Self { handle })
    }
}

impl Drop for BusInner {
    fn drop(&mut self) {
        unsafe { ((*self.handle.vtable).release)(self.handle.this) }
    }
}

// SAFETY: The vtable functions are thread-safe by construction (they lock internally).
unsafe impl Send for BusInner {}
unsafe impl Sync for BusInner {}

// ---------------------------------------------------------------------------
// Discovery — how the first endpoint publishes the bus and later ones adopt it.
// ---------------------------------------------------------------------------

/// Magic prefix on every bus discovery record. Bump this if the layout of the
/// `BusVTable` or the discovery record ever needs an incompatible change post-merge.
const DISCOVERY_MAGIC: [u8; 8] = *b"ODENBUSD";

/// Size in bytes of the on-wire discovery record. 8 (magic) + 1 (pointer width) +
/// 7 (reserved) + 8 (vtable) + 8 (this) = 32.
const DISCOVERY_RECORD_SIZE: usize = 32;

fn discovery_key(owner_uuid: &Uuid) -> String {
    format!("oden_message_bus.{owner_uuid}")
}

/// Serialize a `BusHandle` into a discovery record byte string. Fixed layout, all
/// integers little-endian, no padding.
fn write_discovery_record(handle: BusHandle) -> [u8; DISCOVERY_RECORD_SIZE] {
    let mut out = [0u8; DISCOVERY_RECORD_SIZE];
    out[0..8].copy_from_slice(&DISCOVERY_MAGIC);
    out[8] = std::mem::size_of::<usize>() as u8;
    // out[9..16] reserved, left zero.
    out[16..24].copy_from_slice(&(handle.vtable as usize as u64).to_le_bytes());
    out[24..32].copy_from_slice(&(handle.this as usize as u64).to_le_bytes());
    out
}

/// Parse a discovery record byte string. Returns `None` if the magic or pointer
/// width don't match this build.
fn read_discovery_record(bytes: &[u8]) -> Option<BusHandle> {
    if bytes.len() != DISCOVERY_RECORD_SIZE {
        return None;
    }
    if bytes[0..8] != DISCOVERY_MAGIC {
        return None;
    }
    if bytes[8] as usize != std::mem::size_of::<usize>() {
        return None;
    }
    let vtable_addr = u64::from_le_bytes(bytes[16..24].try_into().ok()?);
    let this_addr = u64::from_le_bytes(bytes[24..32].try_into().ok()?);
    if vtable_addr == 0 || this_addr == 0 {
        return None;
    }
    Some(BusHandle {
        vtable: vtable_addr as usize as *const abi::BusVTable,
        this: this_addr as usize as *mut c_void,
    })
}

/// Adopt the bus published under `owner_uuid`, or allocate it through the host's
/// factory and publish it if none exists yet (or only a corrupt record does).
/// Returns an owned reference either way.
///
/// # Panics
///
/// If the bus has to be created and the host publishes no valid factory record.
fn open_or_create_bus(api: &dyn SettingsApi, owner_uuid: &Uuid) -> BusHandle {
    let key = discovery_key(owner_uuid);

    if let Ok((data, _)) = api.query_data(&key, DISCOVERY_RECORD_SIZE as i32) {
        if let Some(handle) = read_discovery_record(&data) {
            // SAFETY: `read_discovery_record` only rejects corrupt or mismatched
            // records; it cannot prove the pointers are live. That rests on the
            // trust model: this key is only ever written by plugins in this
            // process running this function, and the `BusState` a record points
            // at is never freed (see `imp::release`).
            unsafe { ((*handle.vtable).retain)(handle.this) };
            return handle;
        }
    }

    // The handle returned by the factory starts with refcount 1, which becomes
    // this caller's reference.
    let handle = host::create_bus_via_host(api)
        .expect("host did not publish a valid message bus factory record");
    api.publish_data(&key, &write_discovery_record(handle));
    handle
}

// ---------------------------------------------------------------------------
// BusOwner
// ---------------------------------------------------------------------------

/// Owner-side handle. Created by the plugin that wants to be addressable.
///
/// Cloning is cheap. Dropping every owner leaves the bus fully intact —
/// subscriptions keep their queued messages, clients keep sending into the
/// bounded upstream queue, and the next `open` under the same UUID picks up
/// right where the previous owner left off.
#[derive(Clone)]
pub struct BusOwner {
    inner: Arc<BusInner>,
}

impl BusOwner {
    /// Adopt (or create, if no endpoint has dialed it yet) the bus published
    /// under `owner_uuid` — the well-known address clients pass to
    /// [`BusClient::open`]. Use [`Uuid::parse`] in a `const` so a malformed
    /// literal fails at compile time. The plugin's host-assigned
    /// entity/instance UUID is deliberately not used here: for global plugin
    /// entities the host generates a random per-installation entity UUID, so a
    /// key derived from it would never match the constant clients dial.
    ///
    /// The bus is allocated inside the host executable through its factory
    /// record (see [`host`]), so it stays valid even if this plugin's library
    /// is unloaded.
    ///
    /// # Panics
    ///
    /// If the host publishes no valid bus factory record.
    pub fn open(api: &dyn SettingsApi, owner_uuid: Uuid) -> Self {
        Self {
            inner: BusInner::new(open_or_create_bus(api, &owner_uuid)),
        }
    }

    /// Broadcast a message to every subscriber whose subscription includes `name`.
    /// If no subscription matches, the message is parked for the first later
    /// subscription that wants it (see the module docs on late subscribers).
    ///
    /// Returns `Err(QueueFull)` if at least one subscriber's queue was full — the
    /// broadcast still delivered to subscribers whose queues had space — or if the
    /// message matched no subscription and the parked buffer was full.
    pub fn broadcast(&self, name: &str, payload: &[u8]) -> Result<(), SendError> {
        let h = self.inner.handle;
        let n = BusStr {
            ptr: name.as_ptr(),
            len: name.len(),
        };
        let p = BusBytes {
            ptr: payload.as_ptr(),
            len: payload.len(),
        };
        match unsafe { ((*h.vtable).try_broadcast)(h.this, n, p) } {
            BusStatus::OK => Ok(()),
            _ => Err(SendError::QueueFull),
        }
    }

    /// Drain the upstream lane (messages sent by clients) without waiting for a
    /// message. Blocks while a concurrent `recv_upstream_timeout` on any clone
    /// of this owner is waiting.
    pub fn try_recv_upstream(&self) -> Option<OwnedMessage> {
        let h = self.inner.handle;
        recv_via(
            |name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len| unsafe {
                ((*h.vtable).try_recv_upstream)(
                    h.this, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                )
            },
        )
        .ok()
    }

    /// Blocking drain of the upstream lane. Returns `None` if no message arrived
    /// within `timeout`.
    pub fn recv_upstream_timeout(&self, timeout: Duration) -> Option<OwnedMessage> {
        let h = self.inner.handle;
        let ns = duration_to_ns(timeout);
        recv_via(
            |name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len| unsafe {
                ((*h.vtable).recv_upstream_timeout)(
                    h.this, ns, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                )
            },
        )
        .ok()
    }

    /// Async drain of the upstream lane. Resolves when a message arrives.
    /// Runtime-agnostic: works with any executor that respects `Waker`.
    pub fn recv_upstream_async(&self) -> RecvFuture {
        RecvFuture::new(self.inner.clone(), 0)
    }
}

// ---------------------------------------------------------------------------
// BusClient
// ---------------------------------------------------------------------------

/// Client-side handle for a bus owned by some other plugin.
///
/// Cloning is cheap. The client needs no liveness handling: it stays valid
/// across owner unloads and restarts (see the module docs on lifecycle).
#[derive(Clone)]
pub struct BusClient {
    inner: Arc<BusInner>,
}

impl BusClient {
    /// Open the bus published under `owner_uuid`, creating it if no endpoint has
    /// dialed that UUID yet — subscriptions made before the owner plugin starts
    /// receive its broadcasts once it does.
    ///
    /// # Panics
    ///
    /// If the bus has to be created and the host publishes no valid bus factory
    /// record.
    pub fn open(api: &dyn SettingsApi, owner_uuid: Uuid) -> Self {
        Self {
            inner: BusInner::new(open_or_create_bus(api, &owner_uuid)),
        }
    }

    /// Subscribe to one or more named messages. The returned `Subscription` drops
    /// the subscription on drop.
    pub fn subscribe(&self, names: &[&str]) -> Subscription {
        self.subscribe_with_capacity(names, 0)
    }

    /// Subscribe with an explicit queue capacity. `capacity == 0` uses the default (128).
    pub fn subscribe_with_capacity(&self, names: &[&str], capacity: usize) -> Subscription {
        let h = self.inner.handle;
        let bus_strs: Vec<BusStr> = names
            .iter()
            .map(|n| BusStr {
                ptr: n.as_ptr(),
                len: n.len(),
            })
            .collect();
        let mut handle: BusSubHandle = 0;
        let status = unsafe {
            ((*h.vtable).subscribe)(
                h.this,
                bus_strs.as_ptr(),
                bus_strs.len(),
                capacity,
                &mut handle as *mut BusSubHandle,
            )
        };
        assert!(status == BusStatus::OK, "bus subscribe failed: {status:?}");
        Subscription {
            bus: self.inner.clone(),
            handle,
        }
    }

    /// Send a message to the bus owner. Messages sent while no owner is draining
    /// wait in the bounded upstream queue for the next owner.
    ///
    /// Returns `Err(QueueFull)` if the upstream queue is full.
    pub fn send(&self, name: &str, payload: &[u8]) -> Result<(), SendError> {
        let h = self.inner.handle;
        let n = BusStr {
            ptr: name.as_ptr(),
            len: name.len(),
        };
        let p = BusBytes {
            ptr: payload.as_ptr(),
            len: payload.len(),
        };
        match unsafe { ((*h.vtable).send_upstream)(h.this, n, p) } {
            BusStatus::OK => Ok(()),
            _ => Err(SendError::QueueFull),
        }
    }
}

// ---------------------------------------------------------------------------
// Subscription
// ---------------------------------------------------------------------------

/// A live subscription. Drops the subscription on `Drop`.
pub struct Subscription {
    bus: Arc<BusInner>,
    handle: BusSubHandle,
}

impl Subscription {
    /// Receive without waiting for a message. Returns `None` if no message is
    /// queued. Blocks while a concurrent `recv_timeout` on this subscription is
    /// waiting.
    pub fn try_recv(&self) -> Option<OwnedMessage> {
        let h = self.bus.handle;
        let sub = self.handle;
        recv_via(
            |name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len| unsafe {
                ((*h.vtable).try_recv_downstream)(
                    h.this, sub, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                )
            },
        )
        .ok()
    }

    /// Blocking receive. Returns `None` if no message arrived within `timeout`.
    pub fn recv_timeout(&self, timeout: Duration) -> Option<OwnedMessage> {
        let h = self.bus.handle;
        let sub = self.handle;
        let ns = duration_to_ns(timeout);
        recv_via(
            |name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len| unsafe {
                ((*h.vtable).recv_downstream_timeout)(
                    h.this, sub, ns, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                )
            },
        )
        .ok()
    }

    /// Async receive. Resolves when a message arrives.
    /// Runtime-agnostic: works with any executor that respects `Waker`.
    ///
    /// The future is cancel-safe: dropping it before resolution unregisters the
    /// wake callback and leaves any queued message in the queue for the next
    /// receiver.
    pub fn recv_async(&self) -> RecvFuture {
        RecvFuture::new(self.bus.clone(), self.handle)
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let h = self.bus.handle;
        unsafe { ((*h.vtable).unsubscribe)(h.this, self.handle) };
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn duration_to_ns(d: Duration) -> u64 {
    u64::try_from(d.as_nanos()).unwrap_or(u64::MAX)
}

/// Why a receive produced no message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecvFail {
    /// Nothing queued (or the wait timed out). Receiving later can succeed.
    Empty,
    /// The lane can never produce a message again (the subscription was
    /// dropped, or the bus returned an unknown status code).
    Dead,
}

/// Hide the `BufferTooSmall` retry loop. Starts with reasonable scratch capacities and
/// grows them on demand. `call` invokes the underlying ABI recv with our buffers.
fn recv_via<F>(mut call: F) -> Result<OwnedMessage, RecvFail>
where
    F: FnMut(*mut u8, usize, *mut usize, *mut u8, usize, *mut usize) -> BusStatus,
{
    let mut name = Vec::<u8>::with_capacity(64);
    let mut payload = Vec::<u8>::with_capacity(256);

    loop {
        let mut name_len: usize = 0;
        let mut payload_len: usize = 0;
        let status = call(
            name.as_mut_ptr(),
            name.capacity(),
            &mut name_len as *mut usize,
            payload.as_mut_ptr(),
            payload.capacity(),
            &mut payload_len as *mut usize,
        );
        match status {
            BusStatus::OK => {
                unsafe {
                    name.set_len(name_len);
                    payload.set_len(payload_len);
                }
                let name_str = String::from_utf8(name)
                    .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());
                return Ok(OwnedMessage {
                    name: name_str,
                    payload,
                });
            }
            BusStatus::EMPTY => return Err(RecvFail::Empty),
            BusStatus::BUFFER_TOO_SMALL => {
                // len is 0 here, so `reserve_exact(n)` guarantees capacity >= n.
                if name.capacity() < name_len {
                    name.reserve_exact(name_len);
                }
                if payload.capacity() < payload_len {
                    payload.reserve_exact(payload_len);
                }
            }
            // INVALID_HANDLE, QUEUE_FULL (never returned by recvs), and any code
            // minted by a newer build. An unknown code must never reach the
            // retry arm above — retrying on unknown semantics could loop
            // forever.
            _ => return Err(RecvFail::Dead),
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-FFI async receive — runtime-agnostic.
// ---------------------------------------------------------------------------

/// Shared state between the future and the C wake callback. Held via `Arc`; the
/// bus owns one reference per live one-shot registration, released by exactly
/// one of `wake_trampoline` (fire) or `drop_ctx_trampoline` (unregister).
struct RecvWaker {
    state: Mutex<RecvWakerState>,
}

struct RecvWakerState {
    waker: Option<Waker>,
    /// Set by `wake_trampoline`. Tells the next `poll` that its registration
    /// was consumed and it must re-register before peeking again.
    fired: bool,
}

impl RecvWaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(RecvWakerState {
                waker: None,
                fired: false,
            }),
        })
    }
}

/// Drop one `RecvWaker` reference.
unsafe extern "C" fn drop_ctx_trampoline(ctx: *mut c_void) {
    drop(unsafe { Arc::from_raw(ctx as *const RecvWaker) });
}

/// Consume the registration's reference: mark it fired, take the stored
/// `Waker`, then wake outside the lock.
unsafe extern "C" fn wake_trampoline(ctx: *mut c_void) {
    let state = unsafe { Arc::from_raw(ctx as *const RecvWaker) };
    let waker_to_wake = match state.state.lock() {
        Ok(mut s) => {
            s.fired = true;
            s.waker.take()
        }
        Err(_) => None,
    };
    if let Some(w) = waker_to_wake {
        w.wake();
    }
}

/// Future returned by [`BusOwner::recv_upstream_async`] and [`Subscription::recv_async`].
///
/// Resolves with `Some(message)` when a message arrives, or `None` if the lane
/// can never produce one again (the subscription was dropped while the future
/// was pending).
///
/// Cancel-safe: dropping it before resolution removes its wake registration and
/// leaves any queued message in the queue for the next receiver. Multiple
/// concurrent futures on the same lane each hold their own registration; a
/// message wakes them all and the first to poll takes it.
pub struct RecvFuture {
    bus: Arc<BusInner>,
    /// `0` means the upstream lane on the owner side; non-zero is a client
    /// subscription handle.
    sub: BusSubHandle,
    state: Arc<RecvWaker>,
    /// Token of the live one-shot registration, if any. A fire consumes the
    /// registration (`fired` flips true), so `poll` re-registers before
    /// peeking again.
    token: Option<u64>,
}

impl RecvFuture {
    fn new(bus: Arc<BusInner>, sub: BusSubHandle) -> Self {
        Self {
            bus,
            sub,
            state: RecvWaker::new(),
            token: None,
        }
    }

    fn try_recv_once(&self) -> Result<OwnedMessage, RecvFail> {
        let h = self.bus.handle;
        let sub = self.sub;
        recv_via(
            |name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len| unsafe {
                if sub == 0 {
                    ((*h.vtable).try_recv_upstream)(
                        h.this, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                    )
                } else {
                    ((*h.vtable).try_recv_downstream)(
                        h.this, sub, name_buf, name_cap, name_len, pl_buf, pl_cap, pl_len,
                    )
                }
            },
        )
    }
}

impl Future for RecvFuture {
    type Output = Option<OwnedMessage>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let needs_register = {
            let mut state = match self.state.state.lock() {
                Ok(g) => g,
                Err(_) => return Poll::Ready(None),
            };
            state.waker = Some(cx.waker().clone());
            let fired = state.fired;
            state.fired = false;
            fired || self.token.is_none()
        };

        if needs_register {
            self.token = None;
            // Hand the bus one freshly-incremented reference for it to own. On
            // any non-Ok status the bus has not taken ownership and we reclaim
            // the reference ourselves.
            let ctx = Arc::into_raw(self.state.clone()) as *mut c_void;
            let h = self.bus.handle;
            let mut token: u64 = 0;
            let status = unsafe {
                ((*h.vtable).register_waker)(
                    h.this,
                    self.sub,
                    wake_trampoline,
                    drop_ctx_trampoline,
                    ctx,
                    &mut token as *mut u64,
                )
            };
            if status == BusStatus::OK {
                self.token = Some(token);
            } else {
                drop(unsafe { Arc::from_raw(ctx as *const RecvWaker) });
                // The subscription vanished. Drain anything still queued before
                // reporting the dead lane — no further wake will ever come, so
                // never return Pending from here.
                return Poll::Ready(self.try_recv_once().ok());
            }
        }

        // Race-safe peek: with the registration live, any message that arrives
        // after this point will fire `wake_trampoline`. Anything already in the
        // queue is delivered here.
        match self.try_recv_once() {
            Ok(msg) => Poll::Ready(Some(msg)),
            Err(RecvFail::Empty) => Poll::Pending,
            Err(RecvFail::Dead) => Poll::Ready(None),
        }
    }
}

impl Drop for RecvFuture {
    fn drop(&mut self) {
        // Release the bus-held reference if our registration is still live. If
        // it already fired, the token is gone and this is a no-op — the
        // reference was consumed by the wake.
        if let Some(token) = self.token {
            let h = self.bus.handle;
            unsafe {
                ((*h.vtable).unregister_waker)(h.this, self.sub, token);
            }
        }
    }
}
