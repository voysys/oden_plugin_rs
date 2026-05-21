//! Named shared data is a method for rust plugins compiled together with the same compiler to share data

use crate::{
    layout_id::layout_id, DrawParams, GuiParams, InitParams, SettingsApi, ShutdownParams,
    UpdateParams,
};
use std::{
    mem,
    sync::{self, Arc, Mutex},
};

#[derive(Default)]
struct SharedDataHolder<T>
where
    T: Default + Sync + 'static,
{
    inner: Mutex<sync::Weak<T>>,
}

fn get_or_create_named_shared_data<T>(api: &dyn SettingsApi, name: &str) -> Arc<T>
where
    T: Default + Sync + 'static,
{
    let name = format!("{name}_{}", layout_id::<T>().0);

    let holder =
        if let Ok((addr_data, _)) = api.query_data(&name, mem::size_of::<*const u8>() as i32) {
            let addr = usize::from_le_bytes(addr_data.try_into().unwrap());
            addr as *const SharedDataHolder<T>
        } else {
            let p = Box::new(SharedDataHolder::<T>::default());
            let p = Box::into_raw(p);

            let addr = p as usize;
            let addr = addr.to_le_bytes();
            api.publish_data(&name, &addr);

            p
        };
    let holder = unsafe { &*holder };

    let mut guard = holder.inner.lock().unwrap();

    if let Some(inner) = guard.upgrade() {
        inner
    } else {
        let new_inner = Arc::new(T::default());
        *guard = Arc::downgrade(&new_inner);
        new_inner
    }
}

/// Extension trait for named mpmc channel methods
pub trait NamedSharedDataExt {
    /// Create or fetch a named mpmc channel with the specified name
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T>;
}

impl NamedSharedDataExt for InitParams<'_> {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(self, name)
    }
}

impl NamedSharedDataExt for ShutdownParams<'_> {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(self, name)
    }
}

impl NamedSharedDataExt for UpdateParams<'_> {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(self, name)
    }
}

impl NamedSharedDataExt for DrawParams<'_> {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(self, name)
    }
}

impl NamedSharedDataExt for GuiParams<'_> {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(self, name)
    }
}

impl NamedSharedDataExt for &dyn SettingsApi {
    fn named_shared_data<T: Default + Sync + 'static>(&self, name: &str) -> Arc<T> {
        get_or_create_named_shared_data(*self, name)
    }
}
