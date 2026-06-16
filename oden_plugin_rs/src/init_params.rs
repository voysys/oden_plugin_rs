#[cfg(feature = "serialize")]
use crate::impl_settings_ext_api;
use crate::{impl_init_and_graphics_api, impl_init_and_update_api, impl_settings_api};
use std::marker::PhantomData;

/// # InitParams
/// [`OdenPlugin::init`](crate::OdenPlugin::init) is called when the plugin is initialized.
/// This is an appropriate place to create data for storing state and/or initializing threads.
///
/// InitParams is a struct that exposes the Oden Plugin API functions associated with
/// the [`OdenPlugin::init`](crate::OdenPlugin::init) function.
/// The API is passed as a parameter to `init(api: &InitParams)` and can be accessed as shown in the example below:
///
/// # Examples
/// ```ignore
/// fn init(api: &InitParams) -> Self {
///     State { _dummy: false }
/// }
/// ```
pub struct InitParams<'a> {
    #[doc(hidden)]
    pub inner: *mut crate::plugin_h::OdenPluginEntityInitParams,
    #[doc(hidden)]
    pub phantom: PhantomData<&'a i32>,
}

impl_settings_api!(InitParams);
#[cfg(feature = "serialize")]
impl_settings_ext_api!(InitParams);
impl_init_and_update_api!(InitParams);
impl_init_and_graphics_api!(InitParams);
