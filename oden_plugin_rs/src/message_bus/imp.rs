//! The bus implementation. Compiled into every module that links this crate, but
//! instantiated only by the host executable through the factory in `host`, so
//! every allocation and `mpsc` operation here runs against the host's `std` and
//! global allocator.
//!
//! Owners and clients see only the `BusVTable` and `BusHandle`; they never touch any
//! of the types in this module directly. That's what keeps the ABI safe across
//! mismatched toolchains and allocators.
//!
//! Locks are acquired with `.unwrap()`: a panic inside any of these
//! `extern "C"` entry points aborts the process before it can unwind across the
//! FFI boundary, so a poisoned mutex is unreachable.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    ffi::c_void,
    ptr,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        mpsc, Arc, Mutex,
    },
    time::Duration,
};

use super::abi::{BusBytes, BusHandle, BusStatus, BusStr, BusSubHandle, BusVTable};

struct OwnedBusMessage {
    name: String,
    payload: Vec<u8>,
}

/// Per-subscriber receive state. `pending` holds the head message across
/// `BufferTooSmall` retries so a message is never popped until the caller's
/// buffer is big enough to copy it.
struct SubscriberRx {
    rx: mpsc::Receiver<OwnedBusMessage>,
    pending: Option<OwnedBusMessage>,
}

struct Subscriber {
    wanted: HashSet<String>,
    tx: mpsc::SyncSender<OwnedBusMessage>,
    rx: Arc<Mutex<SubscriberRx>>,
    wakers: Vec<WakerEntry>,
}

struct UpstreamRx {
    rx: mpsc::Receiver<OwnedBusMessage>,
    pending: Option<OwnedBusMessage>,
}

/// One registered one-shot cross-FFI wake callback. The bus owns one reference
/// to `wake_ctx` from registration until exactly one of `fire` (wake_fn consumes
/// the ctx) or `discard` (drop_ctx releases it) runs. Always invoked after every
/// bus lock has been released.
struct WakerEntry {
    token: u64,
    wake_fn: unsafe extern "C" fn(*mut c_void),
    drop_ctx: unsafe extern "C" fn(*mut c_void),
    wake_ctx: *mut c_void,
}

// SAFETY: `wake_fn`, `drop_ctx`, and `wake_ctx` are part of the documented ABI
// contract — the caller of `register_waker` certifies they are safe to invoke from
// any thread.
unsafe impl Send for WakerEntry {}

impl WakerEntry {
    fn fire(self) {
        unsafe { (self.wake_fn)(self.wake_ctx) };
    }

    fn discard(self) {
        unsafe { (self.drop_ctx)(self.wake_ctx) };
    }
}

pub(crate) struct BusState {
    subscribers: Mutex<HashMap<BusSubHandle, Subscriber>>,
    /// Broadcasts that matched no live subscription, retained for the first
    /// later subscription whose name filter matches (claim-once). Mirrors the
    /// old named-channel behavior where messages sent before the consumer
    /// attached waited in the queue. Locked after `subscribers`, never before.
    parked: Mutex<VecDeque<OwnedBusMessage>>,
    upstream_tx: mpsc::SyncSender<OwnedBusMessage>,
    upstream_rx: Arc<Mutex<UpstreamRx>>,
    upstream_wakers: Mutex<Vec<WakerEntry>>,
    /// Total reference count (every owner, client, subscription, recv future).
    /// Advisory only: the `BusState` is leaked rather than freed (see `release`),
    /// so this never drives deallocation. Kept for a future host that refuses to
    /// unload an owner DLL while this is non-zero.
    refcount: AtomicUsize,
    next_handle: AtomicU64,
    next_waker_token: AtomicU64,
}

const DEFAULT_QUEUE_CAP: usize = 128;
const PARKED_CAP: usize = 128;

impl BusState {
    fn new() -> Box<Self> {
        let (upstream_tx, upstream_rx) = mpsc::sync_channel::<OwnedBusMessage>(DEFAULT_QUEUE_CAP);
        Box::new(Self {
            subscribers: Mutex::new(HashMap::new()),
            parked: Mutex::new(VecDeque::new()),
            upstream_tx,
            upstream_rx: Arc::new(Mutex::new(UpstreamRx {
                rx: upstream_rx,
                pending: None,
            })),
            upstream_wakers: Mutex::new(Vec::new()),
            refcount: AtomicUsize::new(1),
            next_handle: AtomicU64::new(1),
            next_waker_token: AtomicU64::new(1),
        })
    }
}

/// Build a fresh `BusHandle` pointing at a newly allocated `BusState`. Refcount starts at 1.
pub(crate) fn create_bus_handle() -> BusHandle {
    let state = Box::into_raw(BusState::new());
    BusHandle {
        vtable: &VTABLE,
        this: state as *mut c_void,
    }
}

unsafe fn state<'a>(this: *mut c_void) -> &'a BusState {
    unsafe { &*(this as *const BusState) }
}

// ---------------------------------------------------------------------------
// vtable entry points
// ---------------------------------------------------------------------------

unsafe extern "C" fn retain(this: *mut c_void) {
    let st = unsafe { state(this) };
    st.refcount.fetch_add(1, Ordering::AcqRel);
}

// Drop one reference. The `BusState` is intentionally never freed: its address is
// embedded in the discovery record published via `SettingsApi`, which has no retract
// primitive, so freeing would leave a dangling pointer for a later `open`/`create` to
// dereference. The allocation is leaked for the process lifetime, mirroring
// `named_shared_data`'s leaked holder.
unsafe extern "C" fn release(this: *mut c_void) {
    let st = unsafe { state(this) };
    st.refcount.fetch_sub(1, Ordering::AcqRel);
}

unsafe fn read_bus_str(s: BusStr) -> String {
    if s.ptr.is_null() || s.len == 0 {
        return String::new();
    }
    let bytes = unsafe { std::slice::from_raw_parts(s.ptr, s.len) };
    String::from_utf8_lossy(bytes).into_owned()
}

unsafe fn read_bus_bytes(b: BusBytes) -> Vec<u8> {
    if b.ptr.is_null() || b.len == 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(b.ptr, b.len).to_vec() }
}

unsafe extern "C" fn send_upstream(
    this: *mut c_void,
    name: BusStr,
    payload: BusBytes,
) -> BusStatus {
    let st = unsafe { state(this) };

    let msg = OwnedBusMessage {
        name: unsafe { read_bus_str(name) },
        payload: unsafe { read_bus_bytes(payload) },
    };

    // The receiver half lives in the leaked `BusState`, so `try_send` can only
    // fail with `Full`.
    if st.upstream_tx.try_send(msg).is_err() {
        return BusStatus::QUEUE_FULL;
    }

    let to_wake: Vec<WakerEntry> = std::mem::take(&mut *st.upstream_wakers.lock().unwrap());
    for entry in to_wake {
        entry.fire();
    }

    BusStatus::OK
}

unsafe extern "C" fn try_broadcast(
    this: *mut c_void,
    name: BusStr,
    payload: BusBytes,
) -> BusStatus {
    let st = unsafe { state(this) };

    let name_owned = unsafe { read_bus_str(name) };
    let payload_owned = unsafe { read_bus_bytes(payload) };

    let mut any_queue_full = false;
    let to_wake: Vec<WakerEntry> = {
        let mut subs = st.subscribers.lock().unwrap();
        let mut to_wake = Vec::new();
        let mut matched = false;
        for sub in subs.values_mut() {
            if !sub.wanted.contains(&name_owned) {
                continue;
            }
            matched = true;
            let msg = OwnedBusMessage {
                name: name_owned.clone(),
                payload: payload_owned.clone(),
            };
            // The receiver half is alive while the subscriber is in the map,
            // so `try_send` can only fail with `Full`.
            match sub.tx.try_send(msg) {
                Ok(()) => to_wake.append(&mut sub.wakers),
                Err(_) => any_queue_full = true,
            }
        }
        if !matched {
            let mut parked = st.parked.lock().unwrap();
            if parked.len() < PARKED_CAP {
                parked.push_back(OwnedBusMessage {
                    name: name_owned,
                    payload: payload_owned,
                });
            } else {
                any_queue_full = true;
            }
        }
        to_wake
    };

    for entry in to_wake {
        entry.fire();
    }

    if any_queue_full {
        BusStatus::QUEUE_FULL
    } else {
        BusStatus::OK
    }
}

unsafe extern "C" fn subscribe(
    this: *mut c_void,
    names: *const BusStr,
    count: usize,
    queue_cap: usize,
    out_handle: *mut BusSubHandle,
) -> BusStatus {
    let st = unsafe { state(this) };
    if out_handle.is_null() {
        return BusStatus::INVALID_HANDLE;
    }

    let cap = if queue_cap == 0 {
        DEFAULT_QUEUE_CAP
    } else {
        queue_cap
    };

    let mut wanted = HashSet::with_capacity(count);
    if !names.is_null() {
        for i in 0..count {
            let s = unsafe { *names.add(i) };
            wanted.insert(unsafe { read_bus_str(s) });
        }
    }

    let (tx, rx) = mpsc::sync_channel::<OwnedBusMessage>(cap);
    let handle = st.next_handle.fetch_add(1, Ordering::AcqRel);

    {
        let mut subs = st.subscribers.lock().unwrap();
        {
            let mut parked = st.parked.lock().unwrap();
            let mut i = 0;
            while i < parked.len() {
                if wanted.contains(&parked[i].name) {
                    if let Some(msg) = parked.remove(i) {
                        tx.try_send(msg).ok();
                    }
                } else {
                    i += 1;
                }
            }
        }
        subs.insert(
            handle,
            Subscriber {
                wanted,
                tx,
                rx: Arc::new(Mutex::new(SubscriberRx { rx, pending: None })),
                wakers: Vec::new(),
            },
        );
    }

    unsafe { *out_handle = handle };
    BusStatus::OK
}

unsafe extern "C" fn unsubscribe(this: *mut c_void, handle: BusSubHandle) -> BusStatus {
    let st = unsafe { state(this) };
    let removed = st.subscribers.lock().unwrap().remove(&handle);
    match removed {
        Some(sub) => {
            // Fire (not discard) so a still-pending RecvFuture wakes, re-polls,
            // and observes InvalidHandle instead of waiting forever.
            for entry in sub.wakers {
                entry.fire();
            }
            BusStatus::OK
        }
        None => BusStatus::INVALID_HANDLE,
    }
}

/// Copy a queued message into caller-provided buffers if both fit. Otherwise leave the
/// message in `*pending` and return `BufferTooSmall` after writing the required sizes.
unsafe fn deliver_message(
    pending: &mut Option<OwnedBusMessage>,
    name_buf: *mut u8,
    name_cap: usize,
    name_len_out: *mut usize,
    payload_buf: *mut u8,
    payload_cap: usize,
    payload_len_out: *mut usize,
) -> BusStatus {
    let msg = match pending.as_ref() {
        Some(m) => m,
        None => return BusStatus::EMPTY,
    };

    let name_bytes = msg.name.as_bytes();
    let need_name = name_bytes.len();
    let need_payload = msg.payload.len();

    if !name_len_out.is_null() {
        unsafe { *name_len_out = need_name };
    }
    if !payload_len_out.is_null() {
        unsafe { *payload_len_out = need_payload };
    }

    if name_cap < need_name || payload_cap < need_payload {
        return BusStatus::BUFFER_TOO_SMALL;
    }

    if need_name > 0 && !name_buf.is_null() {
        unsafe { ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_buf, need_name) };
    }
    if need_payload > 0 && !payload_buf.is_null() {
        unsafe { ptr::copy_nonoverlapping(msg.payload.as_ptr(), payload_buf, need_payload) };
    }

    *pending = None;
    BusStatus::OK
}

/// Look up a subscriber's `rx` and clone the `Arc` out so the global subscribers map
/// lock can be released before we block on the per-subscriber receiver.
fn take_subscriber_rx(st: &BusState, handle: BusSubHandle) -> Option<Arc<Mutex<SubscriberRx>>> {
    let subs = st.subscribers.lock().unwrap();
    subs.get(&handle).map(|s| s.rx.clone())
}

unsafe extern "C" fn try_recv_downstream(
    this: *mut c_void,
    sub: BusSubHandle,
    name_buf: *mut u8,
    name_cap: usize,
    name_len_out: *mut usize,
    payload_buf: *mut u8,
    payload_cap: usize,
    payload_len_out: *mut usize,
) -> BusStatus {
    let st = unsafe { state(this) };
    let rx_arc = match take_subscriber_rx(st, sub) {
        Some(a) => a,
        None => return BusStatus::INVALID_HANDLE,
    };
    let mut rx = rx_arc.lock().unwrap();

    if rx.pending.is_none() {
        match rx.rx.try_recv() {
            Ok(m) => rx.pending = Some(m),
            Err(mpsc::TryRecvError::Empty) => return BusStatus::EMPTY,
            // The send half was dropped: a concurrent `unsubscribe` removed
            // this subscription while we were waiting.
            Err(mpsc::TryRecvError::Disconnected) => return BusStatus::INVALID_HANDLE,
        }
    }

    unsafe {
        deliver_message(
            &mut rx.pending,
            name_buf,
            name_cap,
            name_len_out,
            payload_buf,
            payload_cap,
            payload_len_out,
        )
    }
}

unsafe extern "C" fn recv_downstream_timeout(
    this: *mut c_void,
    sub: BusSubHandle,
    timeout_ns: u64,
    name_buf: *mut u8,
    name_cap: usize,
    name_len_out: *mut usize,
    payload_buf: *mut u8,
    payload_cap: usize,
    payload_len_out: *mut usize,
) -> BusStatus {
    let st = unsafe { state(this) };
    let rx_arc = match take_subscriber_rx(st, sub) {
        Some(a) => a,
        None => return BusStatus::INVALID_HANDLE,
    };
    let mut rx = rx_arc.lock().unwrap();

    if rx.pending.is_none() {
        let res: Result<OwnedBusMessage, mpsc::RecvTimeoutError> = if timeout_ns == u64::MAX {
            rx.rx
                .recv()
                .map_err(|_| mpsc::RecvTimeoutError::Disconnected)
        } else if timeout_ns == 0 {
            rx.rx.try_recv().map_err(|e| match e {
                mpsc::TryRecvError::Empty => mpsc::RecvTimeoutError::Timeout,
                mpsc::TryRecvError::Disconnected => mpsc::RecvTimeoutError::Disconnected,
            })
        } else {
            rx.rx.recv_timeout(Duration::from_nanos(timeout_ns))
        };
        match res {
            Ok(m) => rx.pending = Some(m),
            Err(mpsc::RecvTimeoutError::Timeout) => return BusStatus::EMPTY,
            Err(mpsc::RecvTimeoutError::Disconnected) => return BusStatus::INVALID_HANDLE,
        }
    }

    unsafe {
        deliver_message(
            &mut rx.pending,
            name_buf,
            name_cap,
            name_len_out,
            payload_buf,
            payload_cap,
            payload_len_out,
        )
    }
}

unsafe extern "C" fn try_recv_upstream(
    this: *mut c_void,
    name_buf: *mut u8,
    name_cap: usize,
    name_len_out: *mut usize,
    payload_buf: *mut u8,
    payload_cap: usize,
    payload_len_out: *mut usize,
) -> BusStatus {
    let st = unsafe { state(this) };
    let up_arc = st.upstream_rx.clone();
    let mut up = up_arc.lock().unwrap();

    if up.pending.is_none() {
        match up.rx.try_recv() {
            Ok(m) => up.pending = Some(m),
            Err(_) => return BusStatus::EMPTY,
        }
    }

    unsafe {
        deliver_message(
            &mut up.pending,
            name_buf,
            name_cap,
            name_len_out,
            payload_buf,
            payload_cap,
            payload_len_out,
        )
    }
}

unsafe extern "C" fn recv_upstream_timeout(
    this: *mut c_void,
    timeout_ns: u64,
    name_buf: *mut u8,
    name_cap: usize,
    name_len_out: *mut usize,
    payload_buf: *mut u8,
    payload_cap: usize,
    payload_len_out: *mut usize,
) -> BusStatus {
    let st = unsafe { state(this) };
    let up_arc = st.upstream_rx.clone();
    let mut up = up_arc.lock().unwrap();

    if up.pending.is_none() {
        let res: Result<OwnedBusMessage, mpsc::RecvTimeoutError> = if timeout_ns == u64::MAX {
            up.rx
                .recv()
                .map_err(|_| mpsc::RecvTimeoutError::Disconnected)
        } else if timeout_ns == 0 {
            up.rx.try_recv().map_err(|e| match e {
                mpsc::TryRecvError::Empty => mpsc::RecvTimeoutError::Timeout,
                mpsc::TryRecvError::Disconnected => mpsc::RecvTimeoutError::Disconnected,
            })
        } else {
            up.rx.recv_timeout(Duration::from_nanos(timeout_ns))
        };
        match res {
            Ok(m) => up.pending = Some(m),
            Err(_) => return BusStatus::EMPTY,
        }
    }

    unsafe {
        deliver_message(
            &mut up.pending,
            name_buf,
            name_cap,
            name_len_out,
            payload_buf,
            payload_cap,
            payload_len_out,
        )
    }
}

// On any non-Ok return the bus has NOT taken ownership of `wake_ctx` — the
// caller keeps its reference. Constructing-then-dropping a `WakerEntry` without
// pushing it releases nothing (`WakerEntry` has no `Drop`).
unsafe extern "C" fn register_waker(
    this: *mut c_void,
    sub: BusSubHandle,
    wake_fn: unsafe extern "C" fn(*mut c_void),
    drop_ctx: unsafe extern "C" fn(*mut c_void),
    wake_ctx: *mut c_void,
    out_token: *mut u64,
) -> BusStatus {
    let st = unsafe { state(this) };
    if out_token.is_null() {
        return BusStatus::INVALID_HANDLE;
    }

    let token = st.next_waker_token.fetch_add(1, Ordering::AcqRel);
    let entry = WakerEntry {
        token,
        wake_fn,
        drop_ctx,
        wake_ctx,
    };

    if sub == 0 {
        st.upstream_wakers.lock().unwrap().push(entry);
    } else {
        let mut subs = st.subscribers.lock().unwrap();
        match subs.get_mut(&sub) {
            Some(s) => s.wakers.push(entry),
            None => return BusStatus::INVALID_HANDLE,
        }
    }

    unsafe { *out_token = token };
    BusStatus::OK
}

unsafe extern "C" fn unregister_waker(
    this: *mut c_void,
    sub: BusSubHandle,
    token: u64,
) -> BusStatus {
    let st = unsafe { state(this) };

    let removed: Option<WakerEntry> = if sub == 0 {
        let mut wakers = st.upstream_wakers.lock().unwrap();
        wakers
            .iter()
            .position(|e| e.token == token)
            .map(|i| wakers.swap_remove(i))
    } else {
        let mut subs = st.subscribers.lock().unwrap();
        subs.get_mut(&sub).and_then(|s| {
            s.wakers
                .iter()
                .position(|e| e.token == token)
                .map(|i| s.wakers.swap_remove(i))
        })
    };

    match removed {
        Some(entry) => {
            entry.discard();
            BusStatus::OK
        }
        None => BusStatus::INVALID_HANDLE,
    }
}

/// The single vtable published by every bus owner in this build of the bus crate.
pub(crate) static VTABLE: BusVTable = BusVTable {
    retain,
    release,
    send_upstream,
    try_broadcast,
    subscribe,
    unsubscribe,
    try_recv_downstream,
    recv_downstream_timeout,
    try_recv_upstream,
    recv_upstream_timeout,
    register_waker,
    unregister_waker,
};
