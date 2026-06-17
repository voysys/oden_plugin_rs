//! The Update Api is a trait that has all the functions that are available in an update context in Oden.

#![allow(clippy::needless_lifetimes)]
#![allow(missing_docs)]

use crate::{
    math::{Matrix4, Ray, Vec2, Vec2i, Vec3},
    plugin_h::{OdenCameraState, OdenMonitorInfo},
    ViewportInfo,
};
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;

pub struct ScreenSpaceGuard<'a> {
    #[doc(hidden)]
    pub(crate) inner: *mut crate::plugin_h::OdenPluginEntityUpdateParams,
    #[doc(hidden)]
    pub(crate) phantom: PhantomData<&'a i32>,
}

impl Default for ScreenSpaceGuard<'_> {
    fn default() -> Self {
        Self {
            inner: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }
}

impl Drop for ScreenSpaceGuard<'_> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                if let Some(screen_space_pop) = (*self.inner).imDrawPopScreenSpace {
                    screen_space_pop()
                }
            }
        }
    }
}

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait UpdateApi {
    fn dt(&self) -> f64;
    fn world_matrix(&self) -> Matrix4;
    fn camera_forward_ray(&self) -> Ray;
    fn mouse_pos(&self) -> Vec2;
    fn mouse_pressed_since_last_swap(&self) -> bool;
    fn mouse_released_since_last_swap(&self) -> bool;
    fn download_virtual_camera_image<'a>(&self, entity: &str) -> Option<(Vec2i, &'a [u8])>;
    fn push_recording_bytes(&self, data: &[u8]);
    fn push_recording_data<T: ::zerocopy::AsBytes + 'static>(&self, data: &T);
    fn recording_playback_channel(&self) -> &Receiver<Vec<u8>>;
    fn reset_view(&self);
    fn set_fps_camera_angles_and_position(&self, yaw: f32, pitch: f32, pos: Vec3);
    fn set_fps_camera_fov(&self, fov: f32);
    fn set_entity_transform(&self, entity: &str, mat: Matrix4);
    fn mouse_ray(&self) -> Ray;
    fn viewport_info(&self) -> ViewportInfo;
    fn mouse_pos_px_with_offset(&self) -> Vec2;
    fn mouse_clicked_last_element(&self) -> bool;
    fn mouse_pressed_over_last_element(&self) -> bool;
    fn mouse_over_last_element(&self) -> bool;
    fn virtual_camera_pixel_to_ray(&self, id: &str, x: f32, y: f32) -> Option<Ray>;
    fn screen_space<'a>(&'a self) -> Option<ScreenSpaceGuard<'a>>;
    fn remove_last_element(&self);
    fn monitor_info(&self) -> Option<Vec<OdenMonitorInfo>>;
    fn add_neural_network(&self) -> Result<crate::math::Uuid, crate::NeuralNetworkError>;
    fn remove_neural_network(
        &self,
        uuid: &crate::math::Uuid,
    ) -> Result<(), crate::NeuralNetworkError>;
    fn set_neural_network_model(
        &self,
        uuid: &crate::math::Uuid,
        path: &str,
    ) -> Result<(), crate::NeuralNetworkError>;
    fn set_neural_network_input_video_stream(
        &self,
        uuid: &crate::math::Uuid,
        tensor_name: &str,
        entity_id: &str,
        stream_id: i32,
    ) -> Result<(), crate::NeuralNetworkError>;
    fn neural_network_tensor_data<'a>(
        &'a self,
        uuid: &crate::math::Uuid,
        tensor_name: &str,
    ) -> Result<crate::TensorData<'a>, crate::NeuralNetworkError>;
    fn trigger_neural_network_run(
        &self,
        uuid: &crate::math::Uuid,
    ) -> Result<i32, crate::NeuralNetworkError>;
    fn neural_network_info<'a>(
        &'a self,
        uuid: &crate::math::Uuid,
    ) -> Result<crate::NeuralNetworkInfo<'a>, crate::NeuralNetworkError>;
    fn inference_settings<'a>(
        &'a self,
    ) -> Result<crate::InferenceSettings<'a>, crate::NeuralNetworkError>;
    fn set_neural_network_dla_core(
        &self,
        uuid: &crate::math::Uuid,
        dla_core: i32,
    ) -> Result<(), crate::NeuralNetworkError>;
    fn fps_camera_state(&self) -> OdenCameraState;
    fn prevent_close(&self);
    fn close_was_denied(&self) -> bool;
}
