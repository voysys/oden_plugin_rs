//! Host side of the message bus.
//!
//! The host publishes a *factory record* — the address of a [`BusHostVTable`]
//! living in the host executable — under [`HOST_FACTORY_KEY`] in the plugin
//! data-sharing store, before any plugin runs.
//! [`BusOwner::open`](super::BusOwner::open) allocates every bus through
//! that factory, so both the bus vtable code and the `BusState` reside in the
//! host binary and stay valid across any plugin DLL/SO unload.
//!
//! Oden does this by linking this crate into the host (via `oden_rs`) and calling
//! [`host_factory_record`] there; the record's function pointers therefore point
//! into the host's own copy of `imp`, not into any plugin.

use std::ffi::c_void;

use super::{
    abi::{BusHandle, BusHostVTable},
    imp,
};
use crate::SettingsApi;

/// Data-sharing key under which the host publishes its bus factory record.
pub const HOST_FACTORY_KEY: &str = "oden_message_bus.host";

/// Magic prefix on the factory record. Bump if the layout of [`BusHostVTable`],
/// the record, or the [`BusVTable`](super::abi::BusVTable) behind the handles it
/// creates ever needs an incompatible change post-merge.
const HOST_FACTORY_MAGIC: [u8; 8] = *b"ODENBUSF";

/// Size in bytes of the on-wire factory record. 8 (magic) + 1 (pointer width) +
/// 7 (reserved) + 8 (vtable) + 8 (reserved) = 32, mirroring the bus discovery record.
pub const HOST_FACTORY_RECORD_SIZE: usize = 32;

unsafe extern "C" fn create_bus(out_handle: *mut BusHandle) -> bool {
    if out_handle.is_null() {
        return false;
    }
    unsafe { *out_handle = imp::create_bus_handle() };
    true
}

static HOST_VTABLE: BusHostVTable = BusHostVTable { create_bus };

/// Serialize the factory record for this build's [`BusHostVTable`]. The record
/// embeds pointers into whatever binary this crate is linked into, so it must
/// only be published by a module that is never unloaded — in practice the host
/// executable.
pub fn host_factory_record() -> [u8; HOST_FACTORY_RECORD_SIZE] {
    let mut out = [0u8; HOST_FACTORY_RECORD_SIZE];
    out[0..8].copy_from_slice(&HOST_FACTORY_MAGIC);
    out[8] = std::mem::size_of::<usize>() as u8;
    let vtable_addr = &HOST_VTABLE as *const BusHostVTable as usize as u64;
    out[16..24].copy_from_slice(&vtable_addr.to_le_bytes());
    out
}

fn read_host_factory_record(bytes: &[u8]) -> Option<*const BusHostVTable> {
    if bytes.len() != HOST_FACTORY_RECORD_SIZE {
        return None;
    }
    if bytes[0..8] != HOST_FACTORY_MAGIC {
        return None;
    }
    if bytes[8] as usize != std::mem::size_of::<usize>() {
        return None;
    }
    let vtable_addr = u64::from_le_bytes(bytes[16..24].try_into().ok()?);
    if vtable_addr == 0 {
        return None;
    }
    Some(vtable_addr as usize as *const BusHostVTable)
}

/// Allocate a fresh bus through the host's factory. Returns `None` if this host
/// publishes no factory record (or an invalid one).
pub(crate) fn create_bus_via_host(api: &dyn SettingsApi) -> Option<BusHandle> {
    let (data, _) = api
        .query_data(HOST_FACTORY_KEY, HOST_FACTORY_RECORD_SIZE as i32)
        .ok()?;
    let vtable = read_host_factory_record(&data)?;
    let mut handle = BusHandle {
        vtable: std::ptr::null(),
        this: std::ptr::null_mut::<c_void>(),
    };
    // SAFETY: same trust model as bus discovery records — this key is only ever
    // written by the host running this module's `host_factory_record`, and the
    // vtable it points at lives in the host executable for the process lifetime.
    if !unsafe { ((*vtable).create_bus)(&mut handle as *mut BusHandle) } {
        return None;
    }
    if handle.vtable.is_null() || handle.this.is_null() {
        return None;
    }
    Some(handle)
}
