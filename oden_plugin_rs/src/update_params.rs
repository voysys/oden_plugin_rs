#![allow(clippy::needless_lifetimes)]

#[cfg(feature = "serialize")]
use crate::impl_settings_ext_api;
use crate::{
    impl_graphics_api, impl_init_and_graphics_api, impl_init_and_update_api, impl_scene_api,
    impl_scene_api_ext, impl_settings_api,
    math::{Matrix4, Ray, Vec2, Vec2i, Vec3, Vec4},
    plugin_h::{OdenCameraState, OdenMonitorInfo},
    update_api::ScreenSpaceGuard,
    ImDrawApi, ImDrawTextAlignment, UpdateApi, ViewportInfo,
};
use std::{ffi::CString, marker::PhantomData, slice, sync::mpsc::Receiver};

/// # UpdateParams
/// [`OdenPlugin::update`](crate::OdenPlugin::update) is called once per Oden iteration for all non-hidden entities.
/// This is the proper place to do drawing or light recurring compute tasks.
///
/// UpdateParams is a struct that exposes the Oden Plugin API functions associated with
/// the [`OdenPlugin::update`](crate::OdenPlugin::update) function.
/// The api is passed as a parameter to `update(&mut self, api: &UpdateParams)` and the api can be accessed as shown in the example below:
///
/// # Examples
/// ```ignore
/// fn update(&mut self, api: &InitParams) {
///     api.im_draw_add_text("Hello World!",
///         Vec3::xyz(-1.0, 0.0, -2.0),
///         Vec3::xyz(0.0, 0.3, 0.0),
///         Vec3::xyz(1.0, 0.0, 0.0),
///         Vec4::rgba(0.8, 0.8, 0.8, 1.0),
///         0,
///     );
/// }
/// ```
pub struct UpdateParams<'a> {
    #[doc(hidden)]
    pub inner: *mut crate::plugin_h::OdenPluginEntityUpdateParams,
    #[doc(hidden)]
    pub recording_playback_channel: &'a Receiver<Vec<u8>>,
}

impl_settings_api!(UpdateParams);
#[cfg(feature = "serialize")]
impl_settings_ext_api!(UpdateParams);
impl_scene_api!(UpdateParams);
impl_scene_api_ext!(UpdateParams);
impl_init_and_update_api!(UpdateParams);
impl_graphics_api!(UpdateParams);
impl_init_and_graphics_api!(UpdateParams);

#[inherent::inherent]
impl UpdateApi for UpdateParams<'_> {
    /// Returns the delta time in seconds between last and current frame.
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// let dt = api.dt();
    /// # }
    /// ```
    pub fn dt(&self) -> f64 {
        unsafe { *self.inner }.dt
    }

    /// Returns the plugin entity world transformation matrix.
    pub fn world_matrix(&self) -> Matrix4 {
        unsafe { (*self.inner).worldMatrix }
    }

    /// Returns the viewport camera direction as a [`Ray`] into the world.
    pub fn camera_forward_ray(&self) -> Ray {
        if let Some(get_camera_forward_ray) = unsafe { (*self.inner).getCameraForwardRay } {
            let mut origin = Vec3::default();
            let mut direction = Vec3::default();
            unsafe {
                get_camera_forward_ray(&mut origin as *mut _, &mut direction as *mut _);
            }
            return Ray { origin, direction };
        }

        panic!("This version of Oden is too old to have the camera_forward_ray function");
    }

    /// Returns the mouse position in coordinates relative to the viewport in the range `[0,1]` for X and Y.
    pub fn mouse_pos(&self) -> Vec2 {
        if let Some(get_viewport_mouse_pos) = unsafe { (*self.inner).getViewportMousePos } {
            let mut res = Vec2::default();
            unsafe {
                get_viewport_mouse_pos(&mut res as *mut _);
            }
            return res;
        }
        panic!("This version of Oden is too old to have the mouse_pos function");
    }

    /// Returns `true` if the mouse was pressed the last frame.
    pub fn mouse_pressed_since_last_swap(&self) -> bool {
        if let Some(is_mouse_pressed_since_last_swap) =
            unsafe { (*self.inner).isMousePressedSinceLastSwap }
        {
            unsafe { is_mouse_pressed_since_last_swap() }
        } else {
            panic!(
                "This version of Oden is too old to have the mouse_pressed_since_last_swap function"
            );
        }
    }

    /// Returns `true` if the mouse was released the last frame.
    pub fn mouse_released_since_last_swap(&self) -> bool {
        if let Some(is_mouse_released_since_last_swap) =
            unsafe { (*self.inner).isMouseReleasedSinceLastSwap }
        {
            unsafe { is_mouse_released_since_last_swap() }
        } else {
            panic!(
                "This version of Oden is too old to have the mouse_pressed_since_last_swap function"
            );
        }
    }

    /// Downloads an image from a Virtual Camera, Virtual Cubemap Camera or 2D Video(from version 2.24).
    ///
    /// **The returned data is owned by Oden and is only valid until the end of update.**
    ///
    /// If the data is needed outside of update a copy must be made.
    ///
    /// Returns `true` if downloaded data is available.
    ///
    /// (It may take 4 or more frames before data is available, this function must be called in each of
    /// those frames).
    ///
    /// The download is asynchronous and pipelined: each frame will be available 4 frames
    /// after it is shown on the screen.
    pub fn download_virtual_camera_image<'a>(&self, entity: &str) -> Option<(Vec2i, &'a [u8])> {
        if let Some(download_virtual_camera_image) =
            unsafe { (*self.inner).downloadVirtualCameraImage }
        {
            let entity = CString::new(entity.trim_end_matches('\0')).unwrap();
            let mut width = 0;
            let mut height = 0;

            unsafe {
                let mut raw_rgb_data: *const u8 = std::ptr::null();

                if download_virtual_camera_image(
                    entity.as_ptr(),
                    &mut raw_rgb_data,
                    &mut width as *mut _,
                    &mut height as *mut _,
                ) {
                    Some((
                        Vec2i::xy(width, height),
                        slice::from_raw_parts(raw_rgb_data, (3 * width * height) as usize),
                    ))
                } else {
                    None
                }
            }
        } else {
            panic!("This version of Oden is too old to have the download_virtual_camera_image function");
        }
    }

    /// Push any `data` to the current raw recording.
    ///
    /// This data can then be played back later on, as if it was happening live.
    pub fn push_recording_bytes(&self, data: &[u8]) {
        if let Some(push_recording_data) = unsafe { (*self.inner).pushRecordingData } {
            unsafe {
                push_recording_data(data.as_ptr(), data.len() as i32);
            }
        } else {
            panic!("This version of Oden is too old to have the push_recording_data function");
        }
    }

    /// Push any `data` to the current raw recording.
    ///
    /// This data can then be played back later on, as if it was happening live.
    pub fn push_recording_data<T: ::zerocopy::AsBytes + 'static>(&self, data: &T) {
        if let Some(push_recording_data) = unsafe { (*self.inner).pushRecordingData } {
            let data_bytes = data.as_bytes();
            unsafe {
                push_recording_data(data_bytes.as_ptr(), data_bytes.len() as i32);
            }
        } else {
            panic!("This version of Oden is too old to have the push_recording_data function");
        }
    }

    pub fn recording_playback_channel(&self) -> &Receiver<Vec<u8>> {
        self.recording_playback_channel
    }

    /// Resets the viewport camera to default values.
    /// This is the same as pressing Ctrl+Q inside the application.
    pub fn reset_view(&self) {
        if let Some(reset_view) = unsafe { (*self.inner).resetView } {
            unsafe {
                reset_view();
            }
        } else {
            panic!("This version of Oden is too old to have the reset_view function");
        }
    }

    /// Sets the `yaw`, `pitch` and `pos` of the camera (same as using mouse when looking around in the application).
    ///
    /// `yaw` and `pitch` should be in radians.
    pub fn set_fps_camera_angles_and_position(&self, yaw: f32, pitch: f32, pos: Vec3) {
        if let Some(set_fps_camera_angles_and_position) =
            unsafe { (*self.inner).setFpsCameraAnglesAndPosition }
        {
            unsafe {
                set_fps_camera_angles_and_position(yaw, pitch, pos);
            }
        } else {
            panic!("This version of Oden is too old to have the set_fps_camera_angles_and_position function");
        }
    }

    /// Sets viewport camera field of view (degrees).
    pub fn set_fps_camera_fov(&self, fov: f32) {
        if let Some(set_fps_camera_fov) = unsafe { (*self.inner).setFpsCameraFieldOfView } {
            unsafe {
                set_fps_camera_fov(fov);
            }
        } else {
            panic!("This version of Oden is too old to have the set_fps_camera_fov function");
        }
    }

    /// Sets the `transform` of a given `entity`.
    ///
    /// By controlling the transform of the entity it is possible to precisely control position, scale,
    /// rotation, etc.
    /// This transform is applied after the regular transform specified in the GUI for each entity.
    pub fn set_entity_transform(&self, entity: &str, transform: Matrix4) {
        let entity = CString::new(entity.trim_end_matches('\0')).unwrap();
        if let Some(set_entity_transform) = unsafe { (*self.inner).setEntityTransform } {
            unsafe {
                set_entity_transform(entity.as_ptr(), transform);
            }
        } else {
            panic!("This version of Oden is too old to have the set_entity_transform function");
        }
    }

    /// Returns the [`Ray`] in the direction of the mouse pointer.
    ///
    /// Can be used to get intersection between the mouse and world objects.
    pub fn mouse_ray(&self) -> Ray {
        unsafe {
            if let Some(get_mouse_ray) = (*self.inner).getMouseRay {
                let mut origin = Vec3::new();
                let mut direction = Vec3::new();

                get_mouse_ray(&mut origin as _, &mut direction as *mut _);

                return Ray { origin, direction };
            }
        }
        panic!("This version of Oden is too old to have the mouse_ray function");
    }

    /// Returns information on resolution of connected monitors (or window resolution, if not fullscreen).
    ///
    /// The monitor info returned here is in the "viewport" OpenGL coordinate system.
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::plugin_h::OdenMonitorInfo;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     let monitors: Vec<OdenMonitorInfo> = api.monitor_info().unwrap();
    /// # }
    /// ```
    pub fn monitor_info(&self) -> Option<Vec<OdenMonitorInfo>> {
        unsafe {
            if let Some(get_monitor_info) = (*self.inner).getMonitorInfo {
                let mut monitor_count: i32 = 0;
                get_monitor_info(std::ptr::null_mut(), &mut monitor_count as *mut i32);
                let mut monitor_info = vec![OdenMonitorInfo::default(); monitor_count as usize];
                if get_monitor_info(
                    monitor_info.as_mut_ptr() as *mut _,
                    &mut monitor_count as *mut i32,
                ) {
                    Some(monitor_info)
                } else {
                    None
                }
            } else {
                panic!("This version of Oden is too old to have the monitor_info function");
            }
        }
    }

    /// Returns the viewport info.
    ///
    /// Viewport is the rendered view area.
    /// - `width_px`: viewport width in pixels
    /// - `height_px`: viewport height in pixels
    /// - `origin_x_px`: top left corner of viewport in pixels
    /// - `origin_y_px`: top right corner of viewport in pixels
    /// - `content_scale`: gui "scale" (mainly used to handle high-dpi)
    pub fn viewport_info(&self) -> ViewportInfo {
        unsafe {
            if let Some(get_viewport_info) = (*self.inner).getViewportInfo {
                return get_viewport_info();
            }
        }
        panic!("This version of Oden is too old to have the viewport_info function");
    }

    /// Returns the mouse position in pixels with offset for view and origin wrapping.
    ///
    /// This position will add an offset when the mouse wraps, so it appears to drag outside the window.
    /// If this is not the desired behavior, then use [`mouse_pos`](crate::UpdateParams::mouse_pos) and multiply with [`viewport_info`](crate::UpdateParams::viewport_info).
    pub fn mouse_pos_px_with_offset(&self) -> Vec2 {
        unsafe {
            if let Some(get_mouse_pos_with_offset) = (*self.inner).getViewMousePosPxWithOffset {
                return get_mouse_pos_with_offset();
            }
        }
        panic!("This version of Oden is too old to have the mouse_pos_px_with_offset function");
    }

    /// Returns `true` if mouse clicked last element.
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// if api.mouse_clicked_last_element() {
    ///     // mouse was clicked over the last element
    /// }
    /// # }
    /// ```
    pub fn mouse_clicked_last_element(&self) -> bool {
        if let Some(is_mouse_clicked_last_element) =
            unsafe { (*self.inner).isMouseClickedLastElement }
        {
            unsafe { is_mouse_clicked_last_element() }
        } else {
            panic!(
                "This version of Oden is too old to have the mouse_clicked_last_element function"
            );
        }
    }

    /// Returns `true` if mouse pressed over last element.
    ///
    /// **Note:** this bool persist even if the element is removed after the pressing action
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// if api.mouse_pressed_over_last_element() {
    ///     // mouse was pressed over the last element
    /// }
    /// # }
    /// ```
    pub fn mouse_pressed_over_last_element(&self) -> bool {
        if let Some(is_mouse_pressed_over_last_element) =
            unsafe { (*self.inner).isMousePressedOverLastElement }
        {
            unsafe { is_mouse_pressed_over_last_element() }
        } else {
            panic!(
                "This version of Oden is too old to have the mouse_pressed_over_last_element function"
            );
        }
    }

    /// Returns `true` if mouse is over last element
    ///
    /// **Note:** this bool persist even if the element is removed after the pressing action
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// if api.mouse_over_last_element() {
    ///     // mouse was pressed over the last element
    /// }
    /// # }
    /// ```
    pub fn mouse_over_last_element(&self) -> bool {
        if let Some(is_mouse_over_last_element) =
            unsafe { (*self.inner).imDrawIsMouseOverLastElement }
        {
            unsafe { is_mouse_over_last_element() }
        } else {
            panic!("This version of Oden is too old to have the mouse_over_last_element function");
        }
    }

    /// Returns the [`Ray`] corresponding to a pixel coordinate in a virtual camera image.
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// if let Some(ray) = api.virtual_camera_pixel_to_ray("My Virtual Camera", 1280.0, 720.0) {}
    /// # }
    /// ```
    pub fn virtual_camera_pixel_to_ray(&self, id: &str, x: f32, y: f32) -> Option<Ray> {
        unsafe {
            let mut ray = Ray::default();
            let id = CString::new(id.trim_end_matches('\0')).unwrap();

            if let Some(virtual_camera_pixel_to_ray) = (*self.inner).virtualCameraPixelToRay {
                if virtual_camera_pixel_to_ray(id.as_ptr(), x, y, &mut ray as *mut _) {
                    Some(ray)
                } else {
                    None
                }
            } else {
                panic!("This version of Oden is too old to have the virtual_camera_pixel_to_ray function");
            }
        }
    }
    /// Push screen space enters the screen coordiante system.
    /// Enter the screen coordinate system.
    ///
    /// In screen space the imDraw objects are drawn using a coordinate system that is in pixels and is
    /// global to all monitors. Coordinates can be found using the monitor_info function.
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Some(guard) = api.screen_space() {
    ///         // Screen Space
    ///     }
    /// # }
    /// ```
    pub fn screen_space<'a>(&'a self) -> Option<ScreenSpaceGuard<'a>> {
        unsafe {
            if let Some(screen_space) = (*self.inner).imDrawPushScreenSpace {
                screen_space();
                Some(ScreenSpaceGuard {
                    inner: self.inner,
                    phantom: PhantomData,
                })
            } else {
                panic!("This version of Oden is too old to have the screen_space function");
            }
        }
    }

    /// Removes the last drawn element
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     api.remove_last_element();
    /// # }
    /// ```
    pub fn remove_last_element(&self) {
        unsafe {
            if let Some(remove_last_element) = (*self.inner).imDrawRemoveLastElement {
                remove_last_element();
            } else {
                panic!("This version of Oden is too old to have the remove_last_element function");
            }
        }
    }

    /// Adds a neural network
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     let uuid = api.add_neural_network();
    /// # }
    /// ```
    pub fn add_neural_network(&self) -> Result<crate::math::Uuid, crate::NeuralNetworkError> {
        if let Some(add_neural_network) = unsafe { (*self.inner).addNeuralNetwork } {
            let mut uuid = Default::default();
            let res = unsafe { add_neural_network(&mut uuid) };
            if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                Ok(uuid)
            } else {
                Err(res)
            }
        } else {
            panic!("This version of Oden is too old to have the add_neural_network function");
        }
    }

    /// Remove a neural network
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         api.remove_neural_network(&uuid);
    ///     }
    /// # }
    /// ```
    pub fn remove_neural_network(
        &self,
        uuid: &crate::math::Uuid,
    ) -> Result<(), crate::NeuralNetworkError> {
        if let Some(remove_neural_network) = unsafe { (*self.inner).removeNeuralNetwork } {
            let res = unsafe { remove_neural_network(uuid) };

            if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                Ok(())
            } else {
                Err(res)
            }
        } else {
            panic!("This version of Oden is too old to have the remove_neural_network function");
        }
    }

    /// Set the model on a neural network. Supported formats are trt and onnx
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         api.set_neural_network_model(&uuid, "path_to_model.onnx");
    ///     }
    /// # }
    /// ```
    pub fn set_neural_network_model(
        &self,
        uuid: &crate::math::Uuid,
        path: &str,
    ) -> Result<(), crate::NeuralNetworkError> {
        let path = std::ffi::CString::new(path.trim_end_matches('\0')).unwrap();

        if let Some(set_neural_network_model) = unsafe { (*self.inner).setNeuralNetworkModel } {
            let res = unsafe { set_neural_network_model(uuid, path.as_ptr()) };

            if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                Ok(())
            } else {
                Err(res)
            }
        } else {
            panic!("This version of Oden is too old to have the set_neural_network_model function");
        }
    }

    /// Set the video stream associated with a input tensor of the neural network
    ///
    /// * `uuid` is the id of the network
    /// * `tensor_name` is the name of the input that your neural network expects
    /// * `entity_id` is the name or the uuid of the entity in Oden where you want your input from
    /// * `stream_id` is needed if your input is a stitched video.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         api.set_neural_network_input_video_stream(&uuid, "input_tensor_name", "entity_name_or_uuid", 1);
    ///     }
    /// # }
    /// ```
    pub fn set_neural_network_input_video_stream(
        &self,
        uuid: &crate::math::Uuid,
        tensor_name: &str,
        entity_id: &str,
        stream_id: i32,
    ) -> Result<(), crate::NeuralNetworkError> {
        let tensor_name = std::ffi::CString::new(tensor_name.trim_end_matches('\0')).unwrap();
        let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

        if let Some(set_neural_network_input_video_stream) =
            unsafe { (*self.inner).setNeuralNetworkInputVideoStream }
        {
            let res = unsafe {
                set_neural_network_input_video_stream(
                    uuid,
                    tensor_name.as_ptr(),
                    entity_id.as_ptr(),
                    stream_id,
                )
            };

            if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                Ok(())
            } else {
                Err(res)
            }
        } else {
            panic!("This version of Oden is too old to have the set_neural_network_input_video_stream function");
        }
    }

    /// Fetches a output tensor of the network. The returned data is valid until update returns.
    ///
    /// * `uuid` is the id of the network
    /// * `tensor_name` is the name of the output that your neural network provides
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         if let Ok(tensor_data) = api.neural_network_tensor_data(&uuid, "output_tensor_name") {
    ///             // Do something with the network output
    ///         }
    ///     }
    /// # }
    /// ```
    pub fn neural_network_tensor_data<'a>(
        &'a self,
        uuid: &crate::math::Uuid,
        tensor_name: &str,
    ) -> Result<crate::TensorData<'a>, crate::NeuralNetworkError> {
        let tensor_name = std::ffi::CString::new(tensor_name.trim_end_matches('\0')).unwrap();

        if let Some(get_neural_network_tensor_data) =
            unsafe { (*self.inner).getNeuralNetworkTensorData }
        {
            unsafe {
                let mut tensor_data = Default::default();

                let res =
                    get_neural_network_tensor_data(uuid, tensor_name.as_ptr(), &mut tensor_data);

                if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                    crate::TensorData::try_from(tensor_data)
                } else {
                    Err(res)
                }
            }
        } else {
            panic!(
                "This version of Oden is too old to have the neural_network_tensor_data function"
            );
        }
    }

    /// Triggers a manual run of the neural network. This will cause the network to enter into triggered mode.
    /// So you will need to call this function for every frame you want the neural network to run.
    ///
    /// * `uuid` is the id of the network
    ///
    /// A run id is returned from the function. This can be used to identify when you got the result from this run
    /// by matching it with the `frame_id` in the `TensorData` returned from `get_neural_network_tensor_data`.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         if let Ok(frame_id) = api.trigger_neural_network_run(&uuid) {
    ///             // Use the frame id to figure out when your result is returned
    ///         }
    ///     }
    /// # }
    /// ```
    pub fn trigger_neural_network_run(
        &self,
        uuid: &crate::math::Uuid,
    ) -> Result<i32, crate::NeuralNetworkError> {
        if let Some(trigger_neural_network_run) = unsafe { (*self.inner).triggerNeuralNetworkRun } {
            unsafe {
                let mut frame_id = Default::default();

                let res = trigger_neural_network_run(uuid, &mut frame_id);

                if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                    Ok(frame_id)
                } else {
                    Err(res)
                }
            }
        } else {
            panic!(
                "This version of Oden is too old to have the trigger_neural_network_run function"
            );
        }
    }

    /// Returns information about a neural network with the specified `uuid`.
    ///
    /// * `uuid` is the id of the network
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         if let Ok(neural_network_info) = api.neural_network_info(&uuid) {
    ///
    ///         }
    ///     }
    /// # }
    /// ```
    pub fn neural_network_info<'a>(
        &'a self,
        uuid: &crate::math::Uuid,
    ) -> Result<crate::NeuralNetworkInfo<'a>, crate::NeuralNetworkError> {
        if let Some(get_neural_network_info) = unsafe { (*self.inner).getNeuralNetworkInfo } {
            unsafe {
                let mut neural_network_info = Default::default();

                let res = get_neural_network_info(uuid, &mut neural_network_info);

                if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                    crate::NeuralNetworkInfo::try_from(neural_network_info)
                } else {
                    Err(res)
                }
            }
        } else {
            panic!("This version of Oden is too old to have the neural_network_info function");
        }
    }

    /// Returns the inference settings from the currently loaded project, including neural network info.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(inference_settings) = api.inference_settings() {
    ///
    ///     }
    /// # }
    /// ```
    pub fn inference_settings<'a>(
        &'a self,
    ) -> Result<crate::InferenceSettings<'a>, crate::NeuralNetworkError> {
        if let Some(get_inference_settings) = unsafe { (*self.inner).getInferenceSettings } {
            unsafe {
                let mut inference_settings = Default::default();

                let res = get_inference_settings(&mut inference_settings);

                if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                    crate::InferenceSettings::try_from(inference_settings)
                } else {
                    Err(res)
                }
            }
        } else {
            panic!("This version of Oden is too old to have the inference_settings function");
        }
    }

    /// Set the Deep Learning Accelerator (DLA) core for a neural network.
    ///
    /// This needs to be set before setting the model path to avoid unnecessarily rebuilding the network.
    ///
    /// Only supported on NVIDIA Jetson.
    /// You are responsible for selecting the correct DLA core index. Please see the NVIDIA Jetson docs.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::math::Uuid;
    /// # use std::str::FromStr;
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if let Ok(uuid) = Uuid::from_str("e7ed2d01-ebd1-4986-b72e-9965b775ef53") {
    ///         api.set_neural_network_dla_core(&uuid, 1);
    ///     }
    /// # }
    /// ```
    pub fn set_neural_network_dla_core(
        &self,
        uuid: &crate::math::Uuid,
        dla_core: i32,
    ) -> Result<(), crate::NeuralNetworkError> {
        if let Some(set_neural_network_dla_core) = unsafe { (*self.inner).setNeuralNetworkDlaCore }
        {
            let res = unsafe { set_neural_network_dla_core(uuid, dla_core) };

            if res == crate::NeuralNetworkError::OdenNeuralNetworkErrorOk {
                Ok(())
            } else {
                Err(res)
            }
        } else {
            panic!(
                "This version of Oden is too old to have the set_neural_network_dla_core function"
            );
        }
    }

    /// Gets the viewport camera position, yaw, pitch, and field of view.
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     let _state = api.fps_camera_state();
    /// # }
    /// ```
    pub fn fps_camera_state(&self) -> OdenCameraState {
        unsafe {
            if let Some(fps_camera_state) = (*self.inner).getFpsCameraState {
                let mut state = OdenCameraState::default();

                fps_camera_state(&mut state);

                state
            } else {
                panic!("This version of Oden is too old to have the remove_last_element function");
            }
        }
    }

    /// Prevents the Oden close button from closing the application for the next frame.
    /// The plugin must call this every frame for it to stay in effect. If the plugin stops calling it,
    /// the next close-button press will close Oden normally.
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     api.prevent_close();
    /// # }
    /// ```
    pub fn prevent_close(&self) {
        if let Some(prevent_close) = unsafe { (*self.inner).preventClose } {
            unsafe {
                prevent_close();
            }
        } else {
            panic!("This version of Oden is too old to have the prevent_close function");
        }
    }

    /// Returns `true` if a close attempt was denied on the most recent frame
    /// (because [`prevent_close`](Self::prevent_close) was active).
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    ///     if api.close_was_denied() {
    ///     }
    /// # }
    /// ```
    pub fn close_was_denied(&self) -> bool {
        if let Some(close_was_denied) = unsafe { (*self.inner).closeWasDenied } {
            unsafe { close_was_denied() }
        } else {
            panic!("This version of Oden is too old to have the close_was_denied function");
        }
    }
}

#[inherent::inherent]
impl ImDrawApi for UpdateParams<'_> {
    pub fn im_draw_set_use_z_buffer(&self, enable: bool) {
        if let Some(im_draw_set_use_z_buffer) = unsafe { (*self.inner).imDrawSetUseZBuffer } {
            unsafe {
                im_draw_set_use_z_buffer(enable);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_set_use_z_buffer function");
        }
    }

    pub fn im_draw_set_write_z_buffer(&self, enable: bool) {
        if let Some(im_set_write_z_buffer) = unsafe { (*self.inner).imDrawSetWriteZBuffer } {
            unsafe {
                im_set_write_z_buffer(enable);
            }
        } else {
            panic!(
                "This version of Oden is too old to have the im_draw_set_write_z_buffer function"
            );
        }
    }

    pub fn im_draw_push_matrix(&self, mat: Matrix4) {
        if let Some(im_draw_push_matrix) = unsafe { (*self.inner).imDrawPushMatrix } {
            unsafe {
                im_draw_push_matrix(mat);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_push_matrix function");
        }
    }

    pub fn im_draw_pop_matrix(&self) {
        if let Some(im_draw_pop_matrix) = unsafe { (*self.inner).imDrawPopMatrix } {
            unsafe {
                im_draw_pop_matrix();
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_pop_matrix function");
        }
    }

    pub fn im_draw_push_view_space(&self) {
        if let Some(im_draw_push_view_space) = unsafe { (*self.inner).imDrawPushViewSpace } {
            unsafe {
                im_draw_push_view_space();
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_push_view_space function");
        }
    }

    pub fn im_draw_pop_view_space(&self) {
        if let Some(im_draw_pop_view_space) = unsafe { (*self.inner).imDrawPopViewSpace } {
            unsafe {
                im_draw_pop_view_space();
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_pop_view_space function");
        }
    }

    /// Sets the rendering order of draw calls in view space coordinates.
    /// Higer z_order set will be rendered above lower z_order
    /// The first draw called will be rendered first if they have the same z_order
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::{math::{Vec2, Vec3, Vec4}};
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// api.im_draw_push_view_space();
    /// api.im_draw_set_view_space_z(1);
    ///
    /// let pos = Vec3::xyz(0.0, 0.0, 0.0);
    /// let size = Vec2::xy(1.0, 1.0);
    /// let color = Vec4::xyzw(0.0, 1.0, 0.0, 1.0);
    ///
    /// api.im_draw_add_rectangle(pos, size, color);
    ///
    /// api.im_draw_set_view_space_z(2); // this will render the next rectangle in front of the previous rectangle
    ///
    /// let pos = Vec3::xyz(-0.2, 0.2, 0.0);
    /// let size = Vec2::xy(1.0, 1.0);
    /// let color = Vec4::xyzw(1.0, 0.0, 0.0, 1.0);
    ///
    /// api.im_draw_add_rectangle(pos, size, color);
    ///
    /// api.im_draw_pop_view_space();
    /// # }
    /// ```
    pub fn im_draw_set_view_space_z(&self, z: i32) {
        if let Some(im_draw_set_view_space_z) = unsafe { (*self.inner).imDrawSetViewSpaceZ } {
            unsafe {
                im_draw_set_view_space_z(z);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_set_view_space_z function");
        }
    }

    /// Can be used to make textures transparant or tint the texture using the rgb color channels.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::{math::{Vec2, Vec3, Vec4}};
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// api.im_draw_add_rectangle(
    ///     Vec3::xyz(-0.1, 0.0, -1.0),
    ///     Vec2::xy(0.5, 0.5),
    ///     Vec4::xyzw(0.0, 0.0, 1.0, 1.0),
    /// );
    ///
    /// api.im_draw_set_tint(Vec4::xyzw(1.0, 1.0, 1.0, 0.5));
    /// api.im_draw_add_rectangle(
    ///     Vec3::xyz(0.05, 0.1, -0.99), // put this rectangle a bit infront of the other so transparancy will be visible
    ///     Vec2::xy(0.5, 0.5),
    ///     Vec4::xyzw(1.0, 0.0, 1.0, 1.0),
    /// );
    /// # }
    /// ```
    pub fn im_draw_set_tint(&self, tint: Vec4) {
        if let Some(im_draw_set_tint) = unsafe { (*self.inner).imDrawSetTint } {
            unsafe {
                im_draw_set_tint(tint);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_set_tint function");
        }
    }

    pub fn im_draw_push_shader(&self, shader_id: i32) {
        if let Some(im_draw_push_shader) = unsafe { (*self.inner).imDrawPushShader } {
            unsafe { im_draw_push_shader(shader_id) }
        } else {
            panic!("This version of Oden is too old to have the im_draw_push_shader function");
        }
    }

    pub fn im_draw_pop_shader(&self) {
        if let Some(im_draw_pop_shader) = unsafe { (*self.inner).imDrawPopShader } {
            unsafe { im_draw_pop_shader() }
        } else {
            panic!("This version of Oden is too old to have the im_draw_pop_shader function");
        }
    }

    pub fn im_draw_latch_backbuffer(&self, blur_passes: i32) {
        if let Some(im_draw_latch_backbuffer) = unsafe { (*self.inner).imDrawLatchBackbuffer } {
            unsafe { im_draw_latch_backbuffer(blur_passes) }
        } else {
            panic!("This version of Oden is too old to have the im_draw_latch_backbuffer function");
        }
    }

    pub fn im_draw_add_line(&self, from: Vec3, to: Vec3, normal: Vec3, width: f32, color: Vec4) {
        if let Some(im_draw_add_line) = unsafe { (*self.inner).imDrawAddLine } {
            unsafe {
                im_draw_add_line(from, to, normal, width, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_line function");
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn im_draw_add_circle_segment(
        &self,
        pos: Vec3,
        radius: f32,
        from_angle_rad: f32,
        to_angle_rad: f32,
        width: f32,
        normal: Vec3,
        color: Vec4,
        circle_segments: i32,
    ) {
        if let Some(im_draw_add_circle_segment) = unsafe { (*self.inner).imDrawAddCircleSegment } {
            unsafe {
                im_draw_add_circle_segment(
                    pos,
                    radius,
                    from_angle_rad,
                    to_angle_rad,
                    width,
                    normal,
                    color,
                    circle_segments,
                );
            }
        } else {
            panic!(
                "This version of Oden is too old to have the im_draw_add_circle_segment function"
            );
        }
    }

    pub fn im_draw_add_circle(
        &self,
        pos: Vec3,
        radius: f32,
        width: f32,
        normal: Vec3,
        color: Vec4,
        circle_segments: i32,
    ) {
        if let Some(im_draw_add_circle_) = unsafe { (*self.inner).imDrawAddCircle } {
            unsafe {
                im_draw_add_circle_(pos, radius, width, normal, color, circle_segments);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_circle function");
        }
    }

    pub fn im_draw_add_square(&self, pos: Vec3, size: Vec2, width: f32, color: Vec4) {
        if let Some(im_draw_add_square) = unsafe { (*self.inner).imDrawAddSquare } {
            unsafe {
                im_draw_add_square(pos, size, width, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_square function");
        }
    }

    pub fn im_draw_add_rectangle(&self, pos: Vec3, size: Vec2, color: Vec4) {
        if let Some(im_draw_add_rectangle) = unsafe { (*self.inner).imDrawAddRectangle } {
            unsafe {
                im_draw_add_rectangle(pos, size, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle function");
        }
    }

    pub fn im_draw_add_rectangle_rounded(&self, pos: Vec3, size: Vec2, rounding: f32, color: Vec4) {
        if let Some(im_draw_add_rectangle_rounded) =
            unsafe { (*self.inner).imDrawAddRectangleRounded }
        {
            unsafe {
                im_draw_add_rectangle_rounded(pos, size, rounding, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_rounded function");
        }
    }

    pub fn im_draw_add_image(&self, pos: Vec3, size: Vec2, image_id: i32) {
        if let Some(im_draw_add_image) = unsafe { (*self.inner).imDrawAddImage } {
            unsafe {
                im_draw_add_image(pos, size, image_id);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_image function");
        }
    }

    /// Add a video texture in the same way as an image would be added.
    pub fn im_draw_add_video(&self, pos: Vec3, size: Vec2, entity: &str, stream_id: i32) {
        let entity = CString::new(entity.trim_end_matches('\0')).unwrap();

        if let Some(im_draw_add_video) = unsafe { (*self.inner).imDrawAddVideo } {
            unsafe { im_draw_add_video(pos, size, entity.as_ptr(), stream_id) }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_video function");
        };
    }

    /// Add a video texture, this function also takes the uv coordinates used for sampling the video, top_left: (0.0, 0.0), bottom_right: (1.0, 1.0) is the entire image
    pub fn im_draw_add_video_with_uvs(
        &self,
        pos: Vec3,
        size: Vec2,
        entity: &str,
        stream_id: i32,
        top_left_uv: Vec2,
        bottom_right_uv: Vec2,
    ) {
        let entity = CString::new(entity.trim_end_matches('\0')).unwrap();

        if let Some(im_draw_add_video_with_uvs) = unsafe { (*self.inner).imDrawAddVideoWithUvs } {
            unsafe {
                im_draw_add_video_with_uvs(
                    pos,
                    size,
                    entity.as_ptr(),
                    stream_id,
                    top_left_uv,
                    bottom_right_uv,
                )
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_video function");
        };
    }

    pub fn im_draw_add_text(
        &self,
        text: &str,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        color: Vec4,
        font_id: i32,
    ) {
        if let Some(im_draw_add_text) = unsafe { (*self.inner).imDrawAddText } {
            let c_text = CString::new(text.trim_end_matches('\0')).unwrap();
            unsafe {
                im_draw_add_text(c_text.as_ptr(), pos, up, right, color, font_id);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_text function");
        }
    }

    pub fn im_draw_add_text_normal(
        &self,
        text: &str,
        pos: Vec3,
        normal: Vec3,
        size: f32,
        color: Vec4,
        font_id: i32,
    ) {
        if let Some(im_draw_add_text_normal) = unsafe { (*self.inner).imDrawAddTextNormal } {
            let c_text = CString::new(text.trim_end_matches('\0')).unwrap();
            unsafe {
                im_draw_add_text_normal(c_text.as_ptr(), pos, normal, size, color, font_id);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_text_normal function");
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn im_draw_add_text_aligned(
        &self,
        text: &str,
        pos: Vec3,
        bounding_box: Vec2,
        text_height: f32,
        up: Vec3,
        right: Vec3,
        alignment: ImDrawTextAlignment,
        color: Vec4,
        font_id: i32,
    ) {
        if let Some(im_draw_add_text_aligned) = unsafe { (*self.inner).imDrawAddTextAligned } {
            let c_text = CString::new(text.trim_end_matches('\0')).unwrap();
            unsafe {
                im_draw_add_text_aligned(
                    c_text.as_ptr(),
                    pos,
                    bounding_box,
                    text_height,
                    up,
                    right,
                    alignment,
                    color,
                    font_id,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_text_aligned function");
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn im_draw_add_text_aligned_normal(
        &self,
        text: &str,
        pos: Vec3,
        bounding_box: Vec2,
        text_height: f32,
        normal: Vec3,
        alignment: ImDrawTextAlignment,
        color: Vec4,
        font_id: i32,
    ) {
        if let Some(im_draw_add_text_aligned_normal) =
            unsafe { (*self.inner).imDrawAddTextAlignedNormal }
        {
            let c_text = CString::new(text.trim_end_matches('\0')).unwrap();
            unsafe {
                im_draw_add_text_aligned_normal(
                    c_text.as_ptr(),
                    pos,
                    bounding_box,
                    text_height,
                    normal,
                    alignment,
                    color,
                    font_id,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_text_aligned_normal function");
        }
    }

    pub fn im_draw_add_torus(
        &self,
        pos: Vec3,
        normal: Vec3,
        circle_radius: f32,
        tube_radius: f32,
        color: Vec4,
    ) {
        if let Some(im_draw_add_torus) = unsafe { (*self.inner).imDrawAddTorus } {
            unsafe {
                im_draw_add_torus(pos, normal, circle_radius, tube_radius, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_torus function");
        }
    }

    /// Draw a continuous line between the supplied points
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::{math::{Vec3, Vec4}};
    /// # fn example(api: &oden_plugin_rs::UpdateParams) {
    /// let mut points: Vec<Vec3> = Vec::new();
    /// for x in 0..200 {
    ///     let x = x as f32 / 100.0;
    ///     let y = x.sin();
    ///     points.push(Vec3::xyz(x, y, 0.0));
    /// }
    /// api.im_draw_add_lines(
    ///     &points,
    ///     Vec3::xyz(0.0, 0.0, 1.0),
    ///     0.01,
    ///     Vec4::xyzw(0.7, 0.5, 0.3, 1.0)
    /// );
    /// # }
    /// ```
    pub fn im_draw_add_lines(&self, points: &[Vec3], normal: Vec3, width: f32, color: Vec4) {
        if let Some(im_draw_add_lines) = unsafe { (*self.inner).imDrawAddLines } {
            unsafe {
                im_draw_add_lines(points.as_ptr(), points.len() as i32, normal, width, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_lines function");
        }
    }

    pub fn im_draw_add_rectangle_border(&self, pos: Vec3, size: Vec2, thickness: f32, color: Vec4) {
        if let Some(im_draw_add_rectangle_border) =
            unsafe { (*self.inner).imDrawAddRectangleBorder }
        {
            unsafe {
                im_draw_add_rectangle_border(pos, size, thickness, color);
            }
        } else {
            panic!(
                "This version of Oden is too old to have the im_draw_add_rectangle_border function"
            );
        }
    }

    pub fn im_draw_add_rectangle_border_rounded(
        &self,
        pos: Vec3,
        size: Vec2,
        thickness: f32,
        inner_corner_radius: f32,
        inner_corner_segments: i32,
        color: Vec4,
    ) {
        if let Some(im_draw_add_rectangle_border_rounded) =
            unsafe { (*self.inner).imDrawAddRectangleBorderRounded }
        {
            unsafe {
                im_draw_add_rectangle_border_rounded(
                    pos,
                    size,
                    thickness,
                    inner_corner_radius,
                    inner_corner_segments,
                    color,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_border_rounded function");
        }
    }

    pub fn im_draw_add_cuboid(&self, pos: Vec3, size: Vec3, color: Vec4) {
        if let Some(im_draw_add_cuboid) = unsafe { (*self.inner).imDrawAddCuboid } {
            unsafe {
                im_draw_add_cuboid(pos, size, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_cuboid function");
        }
    }

    pub fn im_draw_add_triangles(&self, vertex_positions: &mut [Vec3], vertex_colors: &mut [Vec4]) {
        if let Some(im_draw_add_triangles) = unsafe { (*self.inner).imDrawAddTriangles } {
            unsafe {
                im_draw_add_triangles(
                    vertex_positions.as_mut_ptr(),
                    vertex_colors.as_mut_ptr(),
                    std::cmp::min(vertex_positions.len(), vertex_colors.len()) as i32,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_triangles function");
        }
    }

    pub fn im_draw_add_rectangle_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        color: Vec4,
    ) {
        if let Some(im_draw_add_rectangle_aligned) =
            unsafe { (*self.inner).imDrawAddRectangleAligned }
        {
            unsafe {
                im_draw_add_rectangle_aligned(pos, up, right, size, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_aligned function");
        }
    }

    pub fn im_draw_add_rectangle_rounded_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        rounding: f32,
        color: Vec4,
    ) {
        if let Some(im_draw_add_rectangle_rounded_aligned) =
            unsafe { (*self.inner).imDrawAddRectangleRoundedAligned }
        {
            unsafe {
                im_draw_add_rectangle_rounded_aligned(pos, up, right, size, rounding, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_rounded_aligned function");
        }
    }

    pub fn im_draw_add_rectangle_border_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        thickness: f32,
        color: Vec4,
    ) {
        if let Some(im_draw_add_rectangle_border_aligned) =
            unsafe { (*self.inner).imDrawAddRectangleBorderAligned }
        {
            unsafe {
                im_draw_add_rectangle_border_aligned(pos, up, right, size, thickness, color);
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_border_aligned function");
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn im_draw_add_rectangle_border_rounded_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        thickness: f32,
        inner_corner_radius: f32,
        inner_corner_segments: i32,
        color: Vec4,
    ) {
        if let Some(im_draw_add_rectangle_border_rounded_aligned) =
            unsafe { (*self.inner).imDrawAddRectangleBorderRoundedAligned }
        {
            unsafe {
                im_draw_add_rectangle_border_rounded_aligned(
                    pos,
                    up,
                    right,
                    size,
                    thickness,
                    inner_corner_radius,
                    inner_corner_segments,
                    color,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_rectangle_border_rounded_aligned function");
        }
    }

    pub fn im_draw_add_triangles_with_uvs(
        &self,
        vertex_positions: &[Vec3],
        vertex_uvs: &[Vec2],
        vertex_colors: &[Vec4],
    ) {
        if let Some(im_draw_add_triangles_with_uvs) =
            unsafe { (*self.inner).imDrawAddTrianglesWithUvs }
        {
            unsafe {
                im_draw_add_triangles_with_uvs(
                    vertex_positions.as_ptr(),
                    vertex_uvs.as_ptr(),
                    vertex_colors.as_ptr(),
                    vertex_positions
                        .len()
                        .min(vertex_uvs.len())
                        .min(vertex_colors.len()) as i32,
                );
            }
        } else {
            panic!("This version of Oden is too old to have the im_draw_add_triangles_with_uvs function");
        }
    }

    pub fn im_draw_calc_text_size(
        &self,
        text: &str,
        text_height: f32,
        max_width: f32,
        font_id: i32,
    ) -> Vec2 {
        if let Some(im_draw_calc_text_size) = unsafe { (*self.inner).imDrawCalcTextSize } {
            let c_text = CString::new(text.trim_end_matches('\0')).unwrap();
            unsafe { im_draw_calc_text_size(c_text.as_ptr(), text_height, max_width, font_id) }
        } else {
            panic!("This version of Oden is too old to have the im_draw_calc_text_size function");
        }
    }
}
