use crate::{
    im_draw_api::ImDrawApi,
    math::{Matrix4, Ray, Uuid, Vec2, Vec2i, Vec3, Vec4},
    plugin_h::{
        OdenCameraState, OdenFlexboxLayout, OdenFlexboxRotation, OdenLinkStatsTransferredBytes,
        OdenMonitorInfo, OdenRay, OdenRayToVideoIntersect,
    },
    scene_api::SceneApi,
    scene_api::{JoystickState, LinkP2PStatus, PrintScreen, RigidBody, RigidBodyUuid},
    update_api::ScreenSpaceGuard,
    update_api::UpdateApi,
    CalibrationQuality, CameraCalibration, CameraMetadata, CameraStreamState, GamepadState,
    ImDrawTextAlignment, KeyModifiers, LinkError, LinkMode, LinkStatsInterfaceStatistics,
    MouseCursor, PlaybackTimes, RegulatorMode, SceneParamError, StreamStatistics, ViewportInfo,
    WindowEvent, WindowMode,
};

use std::{ops::RangeInclusive, sync::mpsc::Receiver};

mockall::mock! {

    pub UpdateSceneImdraw {}

    impl UpdateApi for UpdateSceneImdraw {
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

    impl SceneApi for UpdateSceneImdraw {
        fn inhibit_mouse_look(&self);
        fn horizon(&self, entity: &str) -> Option<Vec3>;
        fn set_horizon(&self, entity: &str, horizon: Vec3) -> bool;
        fn entity_name(&self, entity_id: &str) -> Option<String>;
        fn calibrated_floor_height(&self, entity: &str) -> Option<f32>;
        fn set_stitch_distance(&self, entity_id: &str, dist: f32) -> bool;
        fn camera_metadata(&self, entity: &str, stream: i32) -> Option<CameraMetadata>;
        fn remote_streamer_metadata(&self, remote_streamer_entity: &str) -> Option<CameraMetadata>;
        fn camera_temperature(&self, entity: &str, stream: i32) -> Option<f32>;
        fn camera_fps(&self, entity: &str, stream: i32) -> Option<f32>;
        fn camera_last_valid_timestamp(&self, entity: &str, stream: i32) -> Option<u64>;
        fn set_entity_enabled(&self, entity_id: &str, enable: bool);
        fn entity_scale(&self, entity: &str) -> Result<Vec3, SceneParamError>;
        fn set_entity_scale(&self, entity: &str, scale: &Vec3) -> Result<(), SceneParamError>;
        fn camera_current_timestamp(&self, entity: &str, stream: i32) -> Option<u64>;
        fn set_streamer_frame_metadata(&self, data: &[u8]) -> Result<(), String>;
        fn load_project(&self, path: &str);
        fn start_streamer(&self);
        fn stop_streamer(&self);
        fn camera_stream_state(&self, entity_id: &str, stream: i32) -> Option<CameraStreamState>;
        fn set_camera_stream_state(
            &self,
            entity_id: &str,
            stream: i32,
            stream_state: CameraStreamState,
        ) -> bool;
        fn all_scene_ids(&self) -> Vec<i32>;
        fn selected_scene_id(&self) -> Option<i32>;
        fn set_selected_scene(&self, scene_id: i32) -> bool;
        fn scene_name(&self, scene_id: i32) -> Option<String>;
        fn raw_record_start(&self);
        fn raw_record_stop(&self);
        fn set_raw_record_entity_ring_buffer_size(&self, entity_id: &str, size: i32) -> bool;
        fn raw_record_entity_ring_buffer_enable(&self, entity_id: &str) -> bool;
        fn raw_record_last_path(&self) -> Option<String>;
        fn entity_world_matrix(&self, entity: &str) -> Option<Matrix4>;
        fn is_joystick_present(&self, joystick_id: i32) -> bool;
        fn joystick_state(&self, joystick_id: i32) -> Option<JoystickState>;
        fn gamepad_state(&self, joystick_id: i32) -> Option<GamepadState>;
        fn joystick_state_from_settings_backed_name(&self, settings_key: &str) -> Option<JoystickState>;
        fn stream_statistics(
            &self,
            entity: &str,
            stream: i32,
            max_num_stats: usize,
        ) -> Vec<StreamStatistics>;
        fn stream_statistics_ex(&self, entity: &str) -> Option<Box<crate::StreamStatisticsEx>>;
        fn has_entity_with_name(&self, entity: &str) -> bool;
        fn streamer_bitrate_mbps(&self) -> Option<f32>;
        fn set_streamer_max_bandwidth(&self, bitrate: f32) -> bool;
        fn link_count<'a>(&self, entity: Option<&'a str>) -> Result<i32, LinkError>;
        fn set_link_mode<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            mode: LinkMode,
        ) -> Result<(), LinkError>;
        fn link_mode<'a>(&self, entity: Option<&'a str>, index: i32) -> Result<LinkMode, LinkError>;
        fn set_link_bind_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            ip: &str,
        ) -> Result<(), LinkError>;
        fn link_bind_ip<'a>(&self, entity: Option<&'a str>, index: i32) -> Result<String, LinkError>;
        fn set_link_receive_port<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            port: u16,
        ) -> Result<(), LinkError>;
        fn link_receive_port<'a>(&self, entity: Option<&'a str>, index: i32) -> Result<u16, LinkError>;
        fn set_link_destination_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            ip: &str,
        ) -> Result<(), LinkError>;
        fn link_destination_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn set_link_destination_port<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            port: u16,
        ) -> Result<(), LinkError>;
        fn link_destination_port<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<u16, LinkError>;
        fn set_link_p2p_enabled<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            enabled: bool,
        ) -> Result<(), LinkError>;
        fn link_p2p_status<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<LinkP2PStatus, LinkError>;
        fn link_p2p_enabled<'a>(&self, entity: Option<&'a str>, index: i32) -> Result<bool, LinkError>;
        fn set_link_p2p_stun_server<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            addr: &str,
        ) -> Result<(), LinkError>;
        fn link_p2p_stun_server<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn all_entity_uuids(&self) -> Option<Vec<Uuid>>;
        fn entity_type_name(&self, entity: &str) -> Option<String>;
        fn basic_statistics(&self, entity: &str) -> Option<crate::plugin_h::OdenBasicStatistics>;
        fn remote_streamer_statistics(
            &self,
            remote_streamer_entity: &str,
        ) -> Option<Vec<StreamStatistics>>;
        fn alignment_id(
            &self,
            clone_stream_entity: &str,
            clone_stream: i32,
        ) -> Option<crate::plugin_h::OdenAlignmentId>;
        fn video_placement(
            &self,
            alignment_id: &crate::plugin_h::OdenAlignmentId,
        ) -> Option<crate::plugin_h::OdenVideoPlacement>;
        fn set_link_encryption_private_key_path<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            key_path: &str,
        ) -> Result<(), LinkError>;
        fn link_encryption_private_key_path<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn add_link_encryption_allowed_public_key<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            key: &str,
        ) -> Result<(), LinkError>;
        fn remove_link_encryption_allowed_public_key<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            key: &str,
        ) -> Result<(), LinkError>;
        fn link_encryption_allowed_public_keys_count<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<u16, LinkError>;
        fn link_encryption_allowed_public_key<'a>(
            &self,
            entity: Option<&'a str>,
            link_index: i32,
            key_index: i32,
        ) -> Result<String, LinkError>;
        fn set_link_encryption_internal_source_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            ip: &str,
        ) -> Result<(), LinkError>;
        fn link_encryption_internal_source_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn set_link_encryption_internal_destination_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            ip: &str,
        ) -> Result<(), LinkError>;
        fn link_encryption_internal_destination_ip<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn add_link<'a>(&self, entity: Option<&'a str>) -> Result<(), LinkError>;
        fn remove_link<'a>(&self, entity: Option<&'a str>, index: i32) -> Result<(), LinkError>;
        fn show_hmd_controllers(&self, show: bool);
        fn hmd_transform(&self, include_post_transformation: bool) -> Option<Matrix4>;
        fn set_focus_region_center_pixel_position(&self, entity: &str, x: i32, y: i32) -> bool;
        fn entity_video_size(&self, entity: &str, stream: i32) -> Option<Vec2i>;
        fn set_regulator_mode(&self, mode: RegulatorMode);
        fn regulator_mode(&self) -> RegulatorMode;
        fn set_gui_visible(&self, enable: bool);
        fn is_key_down(&self, key: i32, modifiers: Option<KeyModifiers>) -> bool;
        fn is_key_pressed_since_last_swap(&self, key: i32, modifiers: Option<KeyModifiers>) -> bool;
        fn keys_pressed_since_last_swap(&self) -> Vec<(i32, KeyModifiers)>;
        fn window_events(&self) -> Vec<WindowEvent>;
        fn raw_record_is_running(&self) -> bool;
        fn playback_start(&self);
        fn playback_pause(&self);
        fn set_playback_time(&self, time: f64);
        fn playback_advance_time(&self, time: f64);
        fn set_playback_loop(&self, enable: bool);
        fn playback_is_playing(&self) -> bool;
        fn playback_times(&self) -> Option<PlaybackTimes>;
        fn com_channel_max_message_size(&self) -> i32;
        fn com_channel_send_message_raw(&self, message_id: &str, data: &[u8]);
        fn com_channel_message_count(&self, message_id: &str) -> i32;
        fn com_channel_var_size_message(&self, message_id: &str, index: i32) -> Option<Vec<u8>>;
        fn com_channel_last_var_size_msg(&self, message_id: &str) -> Option<Vec<u8>>;
        fn camera_calibration(&self, entity: &str, stream: i32) -> Option<CameraCalibration>;
        fn scene_containing_plugin_entity(&self) -> Option<i32>;
        fn save_project(&self);
        fn project_has_unsaved_changes(&self) -> bool;
        fn set_force_feedback_angle(&self, angle: f32);
        fn is_streamer_running(&self) -> bool;
        fn camera_last_received_frame_time(&self, entity: &str, stream: i32) -> Option<u64>;
        fn set_audio_receiving_enable<'a>(
            &self,
            entity: Option<&'a str>,
            enable: bool,
        ) -> Result<(), String>;
        fn set_audio_sending_enable<'a>(
            &self,
            entity: Option<&'a str>,
            enable: bool,
        ) -> Result<(), String>;
        fn load_scene(&self, path: &str) -> Option<RangeInclusive<i32>>;
        fn remove_scene(&self, scene_id: i32);
        fn link_stats_bytes_transferred<'a>(
            &self,
            remote_streamer_entity: Option<&'a str>,
            index: i32,
        ) -> Result<OdenLinkStatsTransferredBytes, LinkError>;
        fn output_alignment_position(&self, entity: &str) -> Result<Vec2i, SceneParamError>;
        fn set_output_alignment_position(
            &self,
            entity: &str,
            position: Vec2i,
        ) -> Result<(), SceneParamError>;
        fn output_alignment_scale(&self, entity: &str) -> Result<f32, SceneParamError>;
        fn set_output_alignment_scale(&self, entity: &str, scale: f32) -> Result<(), SceneParamError>;
        fn output_alignment_rotation(
            &self,
            entity: &str,
        ) -> Result<crate::plugin_h::OdenOutputAlignmentRotation, SceneParamError>;
        fn set_output_alignment_rotation(
            &self,
            entity: &str,
            rotation: crate::plugin_h::OdenOutputAlignmentRotation,
        ) -> Result<(), SceneParamError>;
        fn streamer_resolution(&self) -> Result<Vec2i, SceneParamError>;
        fn set_streamer_resolution(&self, resolution: Vec2i) -> Result<(), SceneParamError>;
        fn set_sync_camera(&self, entity_or_stream: &str, stream: i32) -> Result<(), SceneParamError>;
        fn decoder_stats(
            &self,
            entity: &str,
            stream: i32,
        ) -> Result<[crate::plugin_h::OdenDecodedFrame; 64usize], SceneParamError>;
        fn is_over_gui(&self) -> Result<bool, SceneParamError>;
        fn close_project(&self);
        fn request_print_screen(&self);
        fn print_screen<'a>(&'a self) -> Result<PrintScreen<'a>, SceneParamError>;
        fn window_mode(&self) -> Result<WindowMode, SceneParamError>;
        fn link_index<'a>(
            &self,
            entity: Option<&'a str>,
            uuid: &crate::plugin_h::OdenUuid,
        ) -> Result<i32, LinkError>;
        fn link_uuid<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<crate::plugin_h::OdenUuid, LinkError>;
        fn link_stats_interface<'a>(
            &self,
            remote_streamer_entity: Option<&'a str>,
            index: i32,
        ) -> Result<LinkStatsInterfaceStatistics, LinkError>;
        fn set_bandwidth_regulator_state(&self, enabled: bool) -> Result<(), SceneParamError>;
        fn bandwidth_regulator_state(&self) -> Result<bool, SceneParamError>;
        fn scene_to_string(
            &self,
            scene_id: Option<i32>,
            full_scene_config: bool,
        ) -> Result<String, SceneParamError>;
        fn load_scene_from_string(&self, scene_string: &str) -> Result<(), SceneParamError>;
        fn scene_entity_uuids(&self, scene_id: i32) -> Result<Vec<Uuid>, SceneParamError>;
        fn flexbox_layout(&self, entity_id: &str) -> Result<Vec<OdenFlexboxLayout>, SceneParamError>;
        fn set_flexbox_padding(
            &self,
            entity_id: &str,
            padding: &crate::math::Vec4,
        ) -> Result<(), SceneParamError>;
        fn target_bitrate(&self) -> Result<f32, SceneParamError>;
        fn entity_enabled(&self, entity_id: &str) -> Result<bool, SceneParamError>;
        fn output_alignment_id(&self, entity_id: &str) -> Result<i32, SceneParamError>;
        fn clone_source_uuid(&self, entity_id: &str, stream: i32) -> Result<Uuid, SceneParamError>;
        fn set_clone_stream_source(
            &self,
            clone_entity_id: &str,
            clone_stream: i32,
            source_entity_id: &str,
            source_stream: i32,
        ) -> bool;
        fn rigid_bodies(&self) -> Result<Vec<RigidBody>, SceneParamError>;
        fn rigid_bodies_uuid(&self) -> Result<Vec<RigidBodyUuid>, SceneParamError>;
        fn set_camera_calibration(
            &self,
            entity: &str,
            stream: i32,
            camera_calibration: CameraCalibration,
        ) -> bool;
        fn set_camera_calibration_string(&self, entity: &str, calibration: &str) -> bool;
        fn dewarping_params(
            &self,
            entity: &str,
        ) -> Result<(bool, f32, [f32; 5], [f32; 2], [f32; 3]), &'static str>;
        fn set_dewarping_params(
            &self,
            entity: &str,
            enabled: bool,
            fov_deg: Option<f32>,
            k: Option<[f32; 5]>,
            offset: Option<[f32; 2]>,
            rotation_euler_deg: Option<[f32; 3]>,
        ) -> bool;
        fn gui_visible(&self) -> bool;
        fn set_virtual_camera_position(&self, entity: &str, position: Vec3);
        fn set_virtual_camera_rotation(&self, entity: &str, rotation: Vec3);
        fn set_virtual_camera_fov(&self, entity: &str, fov: f32);
        fn set_virtual_camera_resolution(&self, entity: &str, resolution: Vec2i);
        fn virtual_camera_resolution(&self, entity: &str) -> Result<Vec2i, SceneParamError>;
        fn calibration_quality(&self, entity: &str) -> Result<CalibrationQuality, SceneParamError>;
        fn add_entity<'a>(&self, entity: &str, parent: Option<&'a str>) -> Option<Uuid>;
        fn remove_entity(&self, entity: &str);
        fn poll_remote_calibration(
            &self,
            remote_streamer: crate::plugin_h::OdenUuid,
            remote_entity: crate::plugin_h::OdenUuid,
        ) -> Option<crate::scene_api::RemoteCalibrationResult>;
        fn camera_crop(&self, entity: &str, stream: i32) -> Option<crate::CameraCropData>;
        fn set_camera_crop(&self, entity: &str, stream: i32, crop: &crate::CameraCropData) -> Result<(), SceneParamError>;
        fn camera_hard_crop(&self, entity: &str, stream: i32) -> Result<Vec4, SceneParamError>;
        fn set_camera_hard_crop(&self, entity: &str, stream: i32, crop: &Vec4) -> Result<(), SceneParamError>;
        fn drop_detector_timeout(&self, entity: &str, stream: i32) -> Result<f32, SceneParamError>;
        fn set_drop_detector_timeout(&self, entity: &str, stream: i32, timeout_ms: f32) -> Result<(), SceneParamError>;
        fn raw_record_folder(&self) -> Result<String, crate::SceneParamError>;
        fn set_raw_record_folder(&self, folder_path: &str) -> Result<(), crate::SceneParamError>;
        fn virtual_camera_render_only_uuids(
            &self,
            entity_id: &str,
        ) -> Result<Vec<Uuid>, SceneParamError>;
        fn set_external_stitch_geometry(
            &self,
            entity: &str,
            vertices: &[f32],
        ) -> Result<(), SceneParamError>;
        fn ray_to_video_intersect(
            &self,
            entity_id: &str,
            ray: OdenRay,
        ) -> Result<Vec<OdenRayToVideoIntersect>, SceneParamError>;
        fn parent_uuid(&self, entity_id: &str) -> Option<Uuid>;
        fn ancestor_uuid(&self, entity_id: &str, nth: i32) -> Option<Uuid>;
        fn entity_uuid(&self, entity_id: &str) -> Option<Uuid>;
        fn flexbox_rotation(&self, entity_id: &str) -> Result<Vec<OdenFlexboxRotation>, crate::SceneParamError>;
        fn performance_metrics<'a>(&'a self) -> Result<crate::PerformanceMetrics<'a>, SceneParamError>;
        fn set_link_encryption_enabled<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            enable: bool,
        ) -> Result<(), LinkError>;
        fn clear_link_encryption_allowed_public_keys<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<(), LinkError>;
        fn set_link_to_relay_link<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            enable: bool,
        ) -> Result<(), LinkError>;
        fn is_link_relay_link<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<bool, LinkError>;
        fn set_link_send_to_relay<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            enable: bool,
        ) -> Result<(), LinkError>;
        fn is_link_sending_to_relay<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<bool, LinkError>;
        fn set_link_drop_outgoing_packets<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            ignore: bool,
        ) -> Result<(), LinkError>;
        fn remote_streamer_video_info(
            &self,
            remote_streamer_entity: &str,
        ) -> Result<Vec<crate::plugin_h::OdenRemoteStreamerVideo>, SceneParamError>;
        fn stream_name(&self, entity_id: &str, stream: i32) -> Result<String, SceneParamError>;
        fn set_packer_enabled(&self, enable: bool) -> Result<(), SceneParamError>;
        fn set_packer_auto_crop(&self, enabled: bool) -> Result<(), SceneParamError>;
        fn is_packer_enabled(&self) -> Result<bool, SceneParamError>;
        fn is_packer_auto_crop_enabled(&self) -> Result<bool, SceneParamError>;
        fn set_background_color(&self, color: &Vec3) -> Result<(), SceneParamError>;
        fn background_color(&self) -> Result<Vec3, SceneParamError>;
        fn clear_background_color(&self) -> Result<(), SceneParamError>;
        fn window_focused(&self) -> bool;
        fn focus_window(&self);
        fn audio_output_volume<'a>(
            &self,
            remote_streamer: Option<&'a str>,
        ) -> Result<f32, SceneParamError>;
        fn set_audio_output_volume<'a>(
            &self,
            remote_streamer: Option<&'a str>,
            volume: f32,
        ) -> Result<(), SceneParamError>;
        fn link_bind_device<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<String, LinkError>;
        fn set_link_bind_device<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            bind_device: &str
        ) -> Result<(), LinkError>;
        fn inhibit_hotkeys(&self);
        fn set_mouse_cursor(&self, cursor: MouseCursor);
        fn add_link_p2p_excluded_subnet<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
            subnet: &str,
        ) -> Result<(), LinkError>;
        fn clear_link_p2p_excluded_subnets<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<(), LinkError>;
        fn link_time_since_last_received_packet_ns<'a>(
            &self,
            entity: Option<&'a str>,
            index: i32,
        ) -> Result<i64, LinkError>;
    }

    impl ImDrawApi for UpdateSceneImdraw {
        fn im_draw_set_use_z_buffer(&self, enable: bool);
        fn im_draw_set_write_z_buffer(&self, enable: bool);
        fn im_draw_push_matrix(&self, mat: Matrix4);
        fn im_draw_pop_matrix(&self);
        fn im_draw_push_view_space(&self);
        fn im_draw_pop_view_space(&self);
        fn im_draw_set_view_space_z(&self, z: i32);
        fn im_draw_set_tint(&self, tint: Vec4);
        fn im_draw_push_shader(&self, shader_id: i32);
        fn im_draw_pop_shader(&self);
        fn im_draw_latch_backbuffer(&self, blur_passes: i32);
        fn im_draw_add_line(&self, from: Vec3, to: Vec3, normal: Vec3, width: f32, color: Vec4);
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_circle_segment(
            &self,
            pos: Vec3,
            radius: f32,
            from_angle_rad: f32,
            to_angle_rad: f32,
            width: f32,
            normal: Vec3,
            color: Vec4,
            circle_segments: i32,
        );
        fn im_draw_add_circle(
            &self,
            pos: Vec3,
            radius: f32,
            width: f32,
            normal: Vec3,
            color: Vec4,
            circle_segments: i32,
        );
        fn im_draw_add_square(&self, pos: Vec3, size: Vec2, width: f32, color: Vec4);
        fn im_draw_add_rectangle(&self, pos: Vec3, size: Vec2, color: Vec4);
        fn im_draw_add_rectangle_rounded(&self, pos: Vec3, size: Vec2, rounding: f32, color: Vec4);
        fn im_draw_add_image(&self, pos: Vec3, size: Vec2, image_id: i32);
        fn im_draw_add_video(&self, pos: Vec3, size: Vec2, entity: &str, stream_id: i32);
        fn im_draw_add_video_with_uvs(&self, pos: Vec3, size: Vec2, entity: &str, stream_id: i32, op_left_uv: Vec2, bottom_right_uv: Vec2);
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_text(
            &self,
            text: &str,
            pos: Vec3,
            up: Vec3,
            right: Vec3,
            color: Vec4,
            font_id: i32,
        );
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_text_normal(
            &self,
            text: &str,
            pos: Vec3,
            normal: Vec3,
            size: f32,
            color: Vec4,
            font_id: i32,
        );
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_text_aligned(
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
        );
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_text_aligned_normal(
            &self,
            text: &str,
            pos: Vec3,
            bounding_box: Vec2,
            text_height: f32,
            normal: Vec3,
            alignment: ImDrawTextAlignment,
            color: Vec4,
            font_id: i32,
        );
        fn im_draw_add_torus(
            &self,
            pos: Vec3,
            normal: Vec3,
            circle_radius: f32,
            tube_radius: f32,
            color: Vec4,
        );
        fn im_draw_add_lines(&self, point: &[Vec3], normal: Vec3, width: f32, color: Vec4);
        fn im_draw_add_rectangle_border(&self, pos: Vec3, size: Vec2, thickness: f32, color: Vec4);
        fn im_draw_add_rectangle_border_rounded(
            &self,
            pos: Vec3,
            size: Vec2,
            thickness: f32,
            inner_corner_radius: f32,
            inner_corner_segments: i32,
            color: Vec4,
        );
        fn im_draw_add_cuboid(&self, pos: Vec3, size: Vec3, color: Vec4);
        fn im_draw_add_triangles(&self, vertex_positions: &mut [Vec3], vertex_colors: &mut [Vec4]);
        fn im_draw_add_rectangle_aligned(
            &self,
            pos: Vec3,
            up: Vec3,
            right: Vec3,
            size: Vec2,
            color: Vec4,
        );
        fn im_draw_add_rectangle_rounded_aligned(
            &self,
            pos: Vec3,
            up: Vec3,
            right: Vec3,
            size: Vec2,
            rounding: f32,
            color: Vec4,
        );
        fn im_draw_add_rectangle_border_aligned(
            &self,
            pos: Vec3,
            up: Vec3,
            right: Vec3,
            size: Vec2,
            thickness: f32,
            color: Vec4,
        );
        #[allow(clippy::too_many_arguments)]
        fn im_draw_add_rectangle_border_rounded_aligned(
            &self,
            pos: Vec3,
            up: Vec3,
            right: Vec3,
            size: Vec2,
            thickness: f32,
            inner_corner_radius: f32,
            inner_corner_segments: i32,
            color: Vec4,
        );
        fn im_draw_add_triangles_with_uvs(
            &self,
            vertex_positions: &[Vec3],
            vertex_uvs: &[Vec2],
            vertex_colors: &[Vec4],
        );
        fn im_draw_calc_text_size(
            &self,
            text: &str,
            text_height: f32,
            max_width: f32,
            font_id: i32,
        ) -> Vec2;
    }
}
