#[cfg(feature = "serialize")]
use crate::impl_settings_ext_api;
use crate::{
    impl_graphics_api, impl_init_and_graphics_api, impl_scene_api, impl_scene_api_ext,
    impl_settings_api, math::Matrix4, DrawApi,
};
use std::sync::mpsc::Receiver;

/// # DrawParams
/// The function OdenPlugin::draw allows for using raw OpenGL commands.
/// > IMPORTANT NOTE: This is very complex and is not covered in the example plugin.
///
/// > NOTE: The hello_world.rs plugin does not implement the draw function by default since it is an very complex feature.
/// > If desired the function below can be added to the `OdenPlugin` implementation for `State`.
///
/// DrawParams is a struct that exposes the Oden Plugin API functions associated with
/// the OdenPlugin::draw function.
/// The API is passed as a parameter to `draw(&mut self, api: &DrawParams)` and is accessed as shown in the example below:
///
/// # Examples
/// ```ignore
/// fn draw(&mut self, api: &DrawParams) {
///     api.gl_loader();
/// }
/// ```
pub struct DrawParams<'a> {
    #[doc(hidden)]
    pub inner: *mut crate::plugin_h::OdenPluginEntityDrawParams,
    #[doc(hidden)]
    pub recording_playback_channel: &'a Receiver<Vec<u8>>,
}

impl_settings_api!(DrawParams);
#[cfg(feature = "serialize")]
impl_settings_ext_api!(DrawParams);
impl_scene_api!(DrawParams);
impl_scene_api_ext!(DrawParams);
impl_graphics_api!(DrawParams);
impl_init_and_graphics_api!(DrawParams);

#[inherent::inherent]
impl DrawApi for DrawParams<'_> {
    /// Returns the plugin entity world transformation matrix.
    ///
    /// Drawing using e.g. ImDraw is made with an identity world matrix.
    /// If it is desired to draw using the position, rotation, etc of the plugin entity
    /// it is necessary to push this matrix first.
    pub fn world_matrix(&self) -> Matrix4 {
        unsafe { (*self.inner).worldMatrix }
    }

    /// Returns the plugin entity projection matrix.
    pub fn proj_matrix(&self) -> Matrix4 {
        unsafe { (*self.inner).projMatrix }
    }

    pub fn viewport(&self) -> (i32, i32, i32, i32) {
        unsafe {
            (
                (*self.inner).viewportX,
                (*self.inner).viewportY,
                (*self.inner).viewportWidth,
                (*self.inner).viewportHeight,
            )
        }
    }

    pub fn target_size(&self) -> (i32, i32) {
        unsafe { ((*self.inner).targetWidth, (*self.inner).targetHeight) }
    }
}
