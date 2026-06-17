//! ABI for the cross-plugin message bus.
//!
//! Everything in this module is `#[repr(C)]` and crosses the plugin boundary. It must
//! stay stable so plugins built with different toolchains can interoperate. See the
//! `message_bus` module-level documentation for the design rationale and buffer-ownership
//! rules.

#![allow(missing_docs)]

use std::ffi::c_void;

/// Borrowed view of a UTF-8 string. Input-only; the callee copies before returning.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BusStr {
    pub ptr: *const u8,
    pub len: usize,
}

/// Borrowed view of opaque bytes. Input-only; the callee copies before returning.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BusBytes {
    pub ptr: *const u8,
    pub len: usize,
}

/// Opaque subscription handle returned by `subscribe` and consumed by `unsubscribe`/recvs.
pub type BusSubHandle = u64;

/// Status code returned by every vtable function. A `#[repr(transparent)]`
/// wrapper over `u32` rather than a Rust enum so that a code minted by a build
/// with newer status values is an ordinary value to receive — not an invalid
/// enum discriminant, which would be UB at the call site. Callers must treat
/// unknown codes as errors.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BusStatus(pub u32);

impl BusStatus {
    pub const OK: Self = Self(0);
    pub const EMPTY: Self = Self(1);
    pub const INVALID_HANDLE: Self = Self(2);
    pub const BUFFER_TOO_SMALL: Self = Self(3);
    pub const QUEUE_FULL: Self = Self(4);
}

const _: () = assert!(std::mem::size_of::<BusStatus>() == 4);

impl std::fmt::Debug for BusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::OK => f.write_str("BusStatus::OK"),
            Self::EMPTY => f.write_str("BusStatus::EMPTY"),
            Self::INVALID_HANDLE => f.write_str("BusStatus::INVALID_HANDLE"),
            Self::BUFFER_TOO_SMALL => f.write_str("BusStatus::BUFFER_TOO_SMALL"),
            Self::QUEUE_FULL => f.write_str("BusStatus::QUEUE_FULL"),
            Self(other) => write!(f, "BusStatus({other})"),
        }
    }
}

/// The thin client-side handle. Pairs a `vtable` pointer with the owner-allocated `this`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BusHandle {
    pub vtable: *const BusVTable,
    pub this: *mut c_void,
}

unsafe impl Send for BusHandle {}
unsafe impl Sync for BusHandle {}

/// vtable behind the bus factory record published by the host (see the `host` module).
/// Its function pointers live in the host executable, which is never unloaded, so a
/// bus allocated through it survives any plugin DLL unload.
#[repr(C)]
pub struct BusHostVTable {
    /// Allocate a fresh bus in the publisher's memory. The returned handle has
    /// refcount 1.
    /// Returns `false` (leaving `*out_handle` untouched) on failure.
    pub create_bus: unsafe extern "C" fn(out_handle: *mut BusHandle) -> bool,
}

/// vtable for a live bus. Buses are only allocated through [`BusHostVTable`], so
/// these function pointers run in the host executable and its allocator owns
/// every internal allocation.
///
/// All plugins in a build share the same source for this struct, so the layout is
/// identical across them. There is no version field — any incompatible change
/// requires bumping the magic constant in the discovery record and rebuilding
/// every plugin. Adding a `BusStatus` code is not such a change: status is a
/// plain `u32` on the wire and callers treat unknown codes as errors.
#[repr(C)]
pub struct BusVTable {
    /// Increment the bus refcount. Called when a handle (`BusOwner`, `BusClient`)
    /// adopts a new reference to the bus.
    pub retain: unsafe extern "C" fn(this: *mut c_void),

    /// Decrement the bus refcount. The bus is never freed (the `BusState` is leaked,
    /// see the impl), so this only keeps the advisory count balanced.
    pub release: unsafe extern "C" fn(this: *mut c_void),

    /// Client → owner. Caller's `name`/`payload` are borrowed; callee copies before
    /// returning. The upstream queue is bounded; messages sent while no owner is
    /// draining wait in the queue for the next owner.
    pub send_upstream:
        unsafe extern "C" fn(this: *mut c_void, name: BusStr, payload: BusBytes) -> BusStatus,

    /// Owner → matching subscribers. Same borrow rules as `send_upstream`. A
    /// broadcast that matches no live subscription is parked (bounded) and
    /// delivered to the first later subscription whose name filter matches.
    pub try_broadcast:
        unsafe extern "C" fn(this: *mut c_void, name: BusStr, payload: BusBytes) -> BusStatus,

    /// Register a subscription. `names` is a borrowed array of `BusStr`; callee copies.
    /// `queue_cap` of 0 means "use the default".
    pub subscribe: unsafe extern "C" fn(
        this: *mut c_void,
        names: *const BusStr,
        count: usize,
        queue_cap: usize,
        out_handle: *mut BusSubHandle,
    ) -> BusStatus,

    /// Drop a subscription registered by `subscribe`.
    pub unsubscribe: unsafe extern "C" fn(this: *mut c_void, handle: BusSubHandle) -> BusStatus,

    /// Receive on a client subscription without waiting for a message. Not
    /// wait-free: blocks on a per-subscription lock held for the full wait of
    /// any concurrent `recv_downstream_timeout` on the same subscription.
    pub try_recv_downstream: unsafe extern "C" fn(
        this: *mut c_void,
        sub: BusSubHandle,
        name_buf: *mut u8,
        name_cap: usize,
        name_len_out: *mut usize,
        payload_buf: *mut u8,
        payload_cap: usize,
        payload_len_out: *mut usize,
    ) -> BusStatus,

    /// Blocking receive on a client subscription.
    /// `timeout_ns == 0` is non-blocking, `u64::MAX` blocks forever.
    pub recv_downstream_timeout: unsafe extern "C" fn(
        this: *mut c_void,
        sub: BusSubHandle,
        timeout_ns: u64,
        name_buf: *mut u8,
        name_cap: usize,
        name_len_out: *mut usize,
        payload_buf: *mut u8,
        payload_cap: usize,
        payload_len_out: *mut usize,
    ) -> BusStatus,

    /// Receive on the upstream lane (owner draining client sends) without
    /// waiting for a message. Not wait-free: blocks on the upstream lock held
    /// for the full wait of any concurrent `recv_upstream_timeout`.
    pub try_recv_upstream: unsafe extern "C" fn(
        this: *mut c_void,
        name_buf: *mut u8,
        name_cap: usize,
        name_len_out: *mut usize,
        payload_buf: *mut u8,
        payload_cap: usize,
        payload_len_out: *mut usize,
    ) -> BusStatus,

    /// Blocking receive on the upstream lane.
    pub recv_upstream_timeout: unsafe extern "C" fn(
        this: *mut c_void,
        timeout_ns: u64,
        name_buf: *mut u8,
        name_cap: usize,
        name_len_out: *mut usize,
        payload_buf: *mut u8,
        payload_cap: usize,
        payload_len_out: *mut usize,
    ) -> BusStatus,

    /// Register a one-shot wake callback for one lane. `sub == 0` targets the
    /// upstream lane; any non-zero value targets the matching client subscription.
    /// A lane holds any number of concurrent registrations; `*out_token` receives
    /// a token identifying this one for `unregister_waker`.
    ///
    /// Ownership of `wake_ctx` transfers to the bus only when this returns `Ok`.
    /// On any other status the bus has not touched `wake_ctx` and the caller
    /// still owns it. After `Ok`, exactly one of the following releases the
    /// bus's reference: a matching send or removal of the subscription fires
    /// `wake_fn(wake_ctx)` (which takes ownership), or `unregister_waker`
    /// removes the entry and calls `drop_ctx(wake_ctx)`.
    /// Each registration fires at most once;
    /// re-register after a fire to keep waiting.
    ///
    /// The bus releases its internal locks before invoking `wake_fn`, so
    /// `wake_fn` may safely call back into the bus — no deadlock and no UAF on
    /// the ctx pointer.
    pub register_waker: unsafe extern "C" fn(
        this: *mut c_void,
        sub: BusSubHandle,
        wake_fn: unsafe extern "C" fn(ctx: *mut c_void),
        drop_ctx: unsafe extern "C" fn(ctx: *mut c_void),
        wake_ctx: *mut c_void,
        out_token: *mut u64,
    ) -> BusStatus,

    /// Remove a waker registration by token, releasing the stored ctx via
    /// `drop_ctx`. Returns `InvalidHandle` if the token is unknown — it already
    /// fired, was already unregistered, or its subscription was dropped; in every
    /// such case the bus has already released the registration's ctx.
    pub unregister_waker:
        unsafe extern "C" fn(this: *mut c_void, sub: BusSubHandle, token: u64) -> BusStatus,
}
