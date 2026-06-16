#[cfg(feature = "serialize")]
use crate::impl_settings_ext_api;
use crate::{impl_settings_api, plugin_h};
use std::marker::PhantomData;

/// # ShutdownParams
/// The [`OdenPlugin::shutdown`](crate::OdenPlugin::shutdown) call comes when the plugin is closing down.
/// Data allocated during OdenPlugin::shutdown must be freed here.
///
/// ShutdownParams is a struct that exposes the Oden Plugin API functions associated with
/// the [`OdenPlugin::shutdown`](crate::OdenPlugin::shutdown) function.
/// The api is passed as a parameter to `shutdown(self, api: &ShutdownParams)` and the api can be accessed as shown in the example below:
///
/// # Examples
/// ```ignore
/// fn shutdown(self, api: &ShutdownParams) {
///     api.im_draw_add_text("Hello World!",
///         Vec3::xyz(-1.0, 0.0, -2.0),
///         Vec3::xyz(0.0, 0.3, 0.0),
///         Vec3::xyz(1.0, 0.0, 0.0),
///         Vec4::rgba(0.8, 0.8, 0.8, 1.0),
///         0,
///     );
/// }
/// ```
pub struct ShutdownParams<'a> {
    #[doc(hidden)]
    pub inner: *mut plugin_h::OdenPluginEntityShutdownParams,
    #[doc(hidden)]
    pub phantom: PhantomData<&'a i32>,
}

impl_settings_api!(ShutdownParams);
#[cfg(feature = "serialize")]
impl_settings_ext_api!(ShutdownParams);
