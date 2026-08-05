#![allow(clippy::needless_lifetimes)]
#![allow(missing_docs)]

//! The Scene Api is a trait that has all the functions that are used to manipulate the scene in Oden.

use crate::math::{Matrix4, Quat, Uuid, Vec2i, Vec3, Vec4};
use crate::plugin_h::{OdenFlexboxRotation, OdenSceneParamPerformanceMetrics};
use crate::{
    plugin_h::{
        OdenFlexboxLayout, OdenLinkStatsTransferredBytes, OdenRay, OdenRayToVideoIntersect,
    },
    CameraCalibration, CameraCropData, CameraMetadata, CameraStreamState, GamepadState, LinkError,
    LinkMode, LinkP2PStatusWrapper, LinkStatsInterfaceStatistics, MouseCursor, RegulatorMode,
    SceneParamError, StreamStatistics, WindowEvent, WindowMode, PLUGIN_MOD_ALT, PLUGIN_MOD_CONTROL,
    PLUGIN_MOD_SHIFT, PLUGIN_MOD_SUPER,
};
use std::{error::Error, fmt::Display, ops::RangeInclusive, os::raw::c_char};

impl Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::OdenLinkErrorOk => write!(f, "Success"),
            LinkError::OdenLinkErrorUnknown => write!(f, "Unknown error occurred"),
            LinkError::OdenLinkErrorInvalidEntityId => {
                write!(f, "The provided Remote Streamer entity ID is invalid")
            }
            LinkError::OdenLinkErrorInvalidLinkIndex => write!(f, "Invalid link index"),
            LinkError::OdenLinkErrorNoLinksConfigured => {
                write!(f, "No links have been configured. Please add a link")
            }
            LinkError::OdenLinkErrorPortOutOfRange => {
                write!(f, "The provided port number is out of range")
            }
            LinkError::OdenLinkErrorInvalidIpAddress => write!(f, "The provided IP is invalid"),
            LinkError::OdenLinkErrorInsufficientBufferSize => write!(f, "Insufficient buffer size"),
            LinkError::OdenLinkErrorArgumentIsNull => write!(f, "One of the arguments is null"),
            LinkError::OdenLinkErrorInvalidLinkMode => write!(f, "Invalid link mode"),
            LinkError::OdenLinkErrorInvalidPublicKeyIndex => write!(f, "Invalid public key index"),
            LinkError::OdenLinkErrorInvalidPublicKey => write!(f, "Invalid public key"),
            LinkError::OdenLinkErrorInvalidType => write!(f, "Invalid type"),
            LinkError::OdenLinkErrorInvalidLinkUuid => write!(f, "Invalid link UUID"),
            LinkError::OdenLinkErrorNoTrafficOnLink => write!(f, "No traffic on link"),
            _ => write!(f, "Unknown error occurred"),
        }
    }
}

impl Error for LinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureStreamingError {
    OdenTextureStreamingErrorOk,
    OdenTextureStreamingErrorUnknown,
    OdenTextureStreamingErrorUnsupportedApplication,
    OdenTextureStreamingErrorArgumentIsNull,
    OdenTextureStreamingErrorUnsupportedCodec,
    OdenTextureStreamingErrorOutputPipelineRequired,
    OdenTextureStreamingErrorInvalidNumericField,
    OdenTextureStreamingErrorVideoCaptureNotFound,
    OdenTextureStreamingErrorNotConfigured,
    OdenTextureStreamingErrorArgumentContainsNul,
}

impl TextureStreamingError {
    #[doc(hidden)]
    pub fn from_raw(raw: crate::plugin_h::OdenTextureStreamingError) -> Self {
        match raw {
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorOk => {
                TextureStreamingError::OdenTextureStreamingErrorOk
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorUnsupportedApplication => {
                TextureStreamingError::OdenTextureStreamingErrorUnsupportedApplication
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorArgumentIsNull => {
                TextureStreamingError::OdenTextureStreamingErrorArgumentIsNull
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorUnsupportedCodec => {
                TextureStreamingError::OdenTextureStreamingErrorUnsupportedCodec
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorOutputPipelineRequired => {
                TextureStreamingError::OdenTextureStreamingErrorOutputPipelineRequired
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorInvalidNumericField => {
                TextureStreamingError::OdenTextureStreamingErrorInvalidNumericField
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorVideoCaptureNotFound => {
                TextureStreamingError::OdenTextureStreamingErrorVideoCaptureNotFound
            }
            crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorNotConfigured => {
                TextureStreamingError::OdenTextureStreamingErrorNotConfigured
            }
            _ => TextureStreamingError::OdenTextureStreamingErrorUnknown,
        }
    }
}

impl Display for TextureStreamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureStreamingError::OdenTextureStreamingErrorOk => write!(f, "Success"),
            TextureStreamingError::OdenTextureStreamingErrorUnknown => {
                write!(f, "Unknown error occurred")
            }
            TextureStreamingError::OdenTextureStreamingErrorUnsupportedApplication => {
                write!(
                    f,
                    "UI streaming is not available in this application type or build"
                )
            }
            TextureStreamingError::OdenTextureStreamingErrorArgumentIsNull => {
                write!(f, "One of the arguments is null")
            }
            TextureStreamingError::OdenTextureStreamingErrorUnsupportedCodec => {
                write!(f, "Unsupported codec (only H264 and H265 are accepted)")
            }
            TextureStreamingError::OdenTextureStreamingErrorOutputPipelineRequired => {
                write!(f, "output_pipeline is required")
            }
            TextureStreamingError::OdenTextureStreamingErrorInvalidNumericField => {
                write!(f, "Numeric fields must be non-negative")
            }
            TextureStreamingError::OdenTextureStreamingErrorVideoCaptureNotFound => {
                write!(f, "No video capture found for the given entity_id/stream")
            }
            TextureStreamingError::OdenTextureStreamingErrorNotConfigured => {
                write!(
                    f,
                    "No UI streaming configuration exists for the given entity_id/stream"
                )
            }
            TextureStreamingError::OdenTextureStreamingErrorArgumentContainsNul => {
                write!(f, "A string argument contains an interior NUL byte")
            }
        }
    }
}

impl Error for TextureStreamingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Default, Debug)]
pub struct RemoteCalibrationResult {
    pub floor_height: f32,
    pub calibrations: Vec<CameraCalibration>,
}

#[derive(Clone, Default, Debug)]
pub struct PlaybackTimes {
    pub min: f64,
    pub max: f64,
    pub current: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub mod_alt: bool,
    pub mod_ctrl: bool,
    pub mod_shift: bool,
    pub mod_super: bool,
}

impl KeyModifiers {
    pub fn from_mask(modifier: u32) -> Self {
        KeyModifiers {
            mod_alt: modifier & PLUGIN_MOD_ALT > 0,
            mod_ctrl: modifier & PLUGIN_MOD_CONTROL > 0,
            mod_shift: modifier & PLUGIN_MOD_SHIFT > 0,
            mod_super: modifier & PLUGIN_MOD_SUPER > 0,
        }
    }

    pub fn mask(self) -> i32 {
        let mut modifier: i32 = 0;
        if self.mod_alt {
            modifier |= PLUGIN_MOD_ALT as i32;
        }
        if self.mod_ctrl {
            modifier |= PLUGIN_MOD_CONTROL as i32;
        }
        if self.mod_shift {
            modifier |= PLUGIN_MOD_SHIFT as i32;
        }
        if self.mod_super {
            modifier |= PLUGIN_MOD_SUPER as i32;
        }
        modifier
    }
}

pub enum QueryError {
    DataIdNotFound,
    DataIsNotNotExpectedSize,
    InvalidParameters,
    InvalidData,
}

#[derive(Clone, Default, Debug)]
pub struct CalibrationQuality {
    pub quality: f32,
    pub confidence: f32,
}

impl Display for SceneParamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            SceneParamError::OdenSceneParamErrorOk => "OK",
            SceneParamError::OdenSceneParamErrorError => "Unknown error",
            SceneParamError::OdenSceneParamErrorInvalidType => "Invalid type",
            SceneParamError::OdenSceneParamErrorInvalidEntity => "Invalid entity",
            SceneParamError::OdenSceneParamErrorInvalidValue => "Invalid value",
            SceneParamError::OdenSceneParamErrorWrongApplication => "Invalid application",
            SceneParamError::OdenSceneParamErrorUnsupportedGet => "Unsupported get",
            SceneParamError::OdenSceneParamErrorUnsupportedSet => "Unsupported set",
            SceneParamError::OdenSceneParamErrorArgumentIsNullptr => "Argument is null",
            SceneParamError::OdenSceneParamErrorTypeAndStructMismatch => "Type and Struct Mismatch",
            SceneParamError::OdenSceneParamErrorVideoStreamNotAvailable => {
                "Video stream not available"
            }
            SceneParamError::OdenSceneParamErrorMaxEnum => "Max enum",
            SceneParamError::OdenSceneParamErrorUnsupportedForOS => {
                "Function is not supported for this OS."
            }
        };

        write!(f, "SceneParamError: {msg}")
    }
}

impl Error for SceneParamError {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub enum LinkP2PStatus {
    #[default]
    Inactive,
    QueryingStunServer(String),
    NegotiatingAddresses,
    HolePunching(std::net::SocketAddr, std::net::SocketAddr),
    VerifyingConnection(std::net::SocketAddr, std::net::SocketAddr),
    ConnectionEstablished(std::net::SocketAddr, std::net::SocketAddr),
}

pub struct PrintScreen<'a> {
    pub data: &'a [u8],
    pub height: i32,
    pub width: i32,
}

#[derive(Clone, Default, PartialEq)]
pub struct JoystickState {
    pub name: String,
    pub axes: Vec<f32>,
    pub buttons: Vec<bool>,
}

impl Display for JoystickState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JoystickState {{ axes: {:?}, buttons: {:?} }}",
            self.axes, self.buttons
        )
    }
}
impl JoystickState {
    pub fn from_c(c_state: crate::plugin_h::OdenJoystickState_s) -> Self {
        let name = unsafe { std::ffi::CStr::from_ptr(c_state.name.as_ptr()) }
            .to_string_lossy()
            .into();

        let axes = c_state.axes[..c_state.axesCount as usize].to_vec();
        let buttons = c_state.buttons[..c_state.buttonsCount as usize]
            .iter()
            .map(|&b| b != 0)
            .collect();

        Self {
            name,
            axes,
            buttons,
        }
    }

    pub fn button_pressed(&self, button_index: i32) -> bool {
        button_index >= 0
            && self.buttons.len() as i32 > button_index
            && self.buttons[button_index as usize]
    }
}

impl From<LinkP2PStatusWrapper> for LinkP2PStatus {
    fn from(wrapper: LinkP2PStatusWrapper) -> Self {
        match wrapper.status {
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusInactive => Self::Inactive,
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusQueryingStunServer => {
                let stun_server = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.destinationIp.as_ptr())
                        .to_string_lossy()
                        .to_string()
                };
                Self::QueryingStunServer(stun_server)
            }
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusNegotiatingAddresses => {
                Self::NegotiatingAddresses
            }
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusHolePunching => {
                let src_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.sourceIp.as_ptr()).to_string_lossy()
                };
                let dst_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.destinationIp.as_ptr()).to_string_lossy()
                };

                Self::HolePunching(src_addr.parse().unwrap(), dst_addr.parse().unwrap())
            }
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusVerifyingConnection => {
                let src_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.sourceIp.as_ptr()).to_string_lossy()
                };
                let dst_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.destinationIp.as_ptr()).to_string_lossy()
                };

                Self::VerifyingConnection(src_addr.parse().unwrap(), dst_addr.parse().unwrap())
            }
            crate::plugin_h::OdenLinkP2pStatus::OdenLinkP2pStatusConnectionEstablished => {
                let src_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.sourceIp.as_ptr()).to_string_lossy()
                };
                let dst_addr = unsafe {
                    std::ffi::CStr::from_ptr(wrapper.destinationIp.as_ptr()).to_string_lossy()
                };

                Self::ConnectionEstablished(src_addr.parse().unwrap(), dst_addr.parse().unwrap())
            }
            _ => Self::Inactive,
        }
    }
}

impl Display for LinkP2PStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inactive => write!(f, "Inactive"),
            Self::QueryingStunServer(stun_server) => {
                write!(f, "Querying {stun_server} for WAN IP/port")
            }
            Self::NegotiatingAddresses => write!(f, "Negotiating addresses with remote link"),
            Self::HolePunching(our, remote) => {
                let sep = if our.is_ipv6() || remote.is_ipv6() {
                    " <->\n"
                } else {
                    " <-> "
                };
                write!(f, "Hole punching:\n{our}{sep}{remote}")
            }
            Self::VerifyingConnection(our, remote) => {
                let sep = if our.is_ipv6() || remote.is_ipv6() {
                    " <->\n"
                } else {
                    " <-> "
                };
                write!(f, "Verifying connection:\n{our}{sep}{remote}")
            }
            Self::ConnectionEstablished(our, remote) => {
                let sep = if our.is_ipv6() || remote.is_ipv6() {
                    " <->\n"
                } else {
                    " <-> "
                };
                write!(f, "Established connection:\n{our}{sep}{remote}")
            }
        }
    }
}

#[derive(Debug)]

pub struct InferenceMetric<'a> {
    pub network_path: &'a str,
    pub uuid: Uuid,
    pub inference_time: std::time::Duration,
}

#[derive(Debug)]
pub struct PerformanceMetrics<'a> {
    pub total_frame_time: std::time::Duration,
    pub update_time: std::time::Duration,
    pub draw_time: std::time::Duration,
    pub gui_time: std::time::Duration,
    pub gpu_draw_time: std::time::Duration,
    pub encode_time: std::time::Duration,

    pub inference_metrics: Vec<InferenceMetric<'a>>,
}

impl<'a> From<OdenSceneParamPerformanceMetrics> for PerformanceMetrics<'a> {
    fn from(perf_metrics: OdenSceneParamPerformanceMetrics) -> Self {
        PerformanceMetrics {
            total_frame_time: std::time::Duration::from_secs_f64(perf_metrics.totalFrameTimeS),
            update_time: std::time::Duration::from_secs_f64(perf_metrics.updateTimeS),
            draw_time: std::time::Duration::from_secs_f64(perf_metrics.drawTimeS),
            gui_time: std::time::Duration::from_secs_f64(perf_metrics.guiTimeS),
            encode_time: std::time::Duration::from_secs_f64(perf_metrics.encodeTimeS),
            gpu_draw_time: std::time::Duration::from_secs_f64(perf_metrics.gpuDrawTimeS),
            inference_metrics: perf_metrics.inferenceMetrics
                [..perf_metrics.inferenceMetricsCount as usize]
                .iter()
                .map(|inference_metric| InferenceMetric {
                    network_path: unsafe {
                        std::ffi::CStr::from_ptr(inference_metric.networkPath)
                            .to_str()
                            .unwrap()
                    },
                    uuid: inference_metric.uuid,
                    inference_time: std::time::Duration::from_secs_f64(
                        inference_metric.inferenceTimeS,
                    ),
                })
                .collect(),
        }
    }
}

pub(crate) fn link_string<F>(
    mut function: F,
    entity: Option<&str>,
    index: i32,
) -> Result<String, LinkError>
where
    F: FnMut(*const c_char, i32, *mut c_char, *mut i32) -> LinkError,
{
    let mut buffer = vec![0; 1024];
    let ptr = buffer.as_mut_ptr();
    let mut size = buffer.len() as i32;

    let res = match entity {
        Some(entity) => {
            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

            function(
                entity.as_ptr(),
                index,
                ptr as *mut c_char,
                &mut size as *mut i32,
            )
        }
        None => function(
            std::ptr::null(),
            index,
            ptr as *mut c_char,
            &mut size as *mut i32,
        ),
    };

    match res {
        LinkError::OdenLinkErrorOk => {
            if size > 0 {
                // Resize to size + 1 because we need a ´\0´ for crate::utils::utf8_from_raw
                buffer.resize(size as usize + 1, 0);
            } else {
                buffer.clear()
            }
            match crate::utils::utf8_from_raw(&buffer) {
                Ok(buffer) => Ok(buffer),
                Err(_) => Err(LinkError::OdenLinkErrorUnknown),
            }
        }
        _ => Err(res),
    }
}

pub(crate) fn set_link_string<F>(
    mut set_function: F,
    entity: Option<&str>,
    index: i32,
    string_to_write: &str,
) -> Result<(), LinkError>
where
    F: FnMut(*const c_char, i32, *const c_char) -> LinkError,
{
    let string_to_write = std::ffi::CString::new(string_to_write.trim_end_matches('\0')).unwrap();

    let res = match entity {
        Some(entity) => {
            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

            set_function(entity.as_ptr(), index, string_to_write.as_ptr())
        }
        None => set_function(std::ptr::null(), index, string_to_write.as_ptr()),
    };

    match res {
        LinkError::OdenLinkErrorOk => Ok(()),
        _ => Err(res),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColliderType {
    Box,
    Cylinder,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub offset: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub density: f32,
    pub friction_coefficient: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigidBody {
    pub entity_id: String,
    pub dynamic: bool,
    pub colliders: Vec<Collider>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigidBodyUuid {
    pub entity_name: String,
    pub entity_uuid: Uuid,
    pub dynamic: bool,
    pub colliders: Vec<Collider>,
}

/// Gamepad axis indices corresponding to SDL3's `SDL_GamepadAxis` layout.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAxis {
    LeftX = 0,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

/// Gamepad button indices corresponding to SDL3's `SDL_GamepadButton` layout,
/// used to index [`GamepadState`](crate::GamepadState)'s `buttons` array.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton {
    South = 0,
    East,
    West,
    North,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    Misc1,
    RightPaddle1,
    LeftPaddle1,
    RightPaddle2,
    LeftPaddle2,
    Touchpad,
}

impl crate::plugin_h::OdenGamepadStateV2_s {
    /// Returns whether the specified gamepad button is pressed.
    #[inline]
    pub fn button_pressed(&self, button: GamepadButton) -> bool {
        let index = button as usize;
        self.buttons.get(index).map(|&b| b != 0).unwrap_or(false)
    }

    /// Returns the current value of the specified gamepad axis in the range `[-1.0, 1.0]`.
    #[inline]
    pub fn axis_value(&self, axis: GamepadAxis) -> f32 {
        let index = axis as usize;
        *self.axes.get(index).unwrap_or(&0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureStreamingCodec {
    #[default]
    H264,
    H265,
}

#[derive(Debug, Clone, Default)]
pub struct TextureStreamingConfig {
    pub width: i32,
    pub height: i32,
    pub frame_rate: i32,
    pub bitrate_kbps: i32,
    pub codec: TextureStreamingCodec,
    pub output_pipeline: String,
    pub entity_id: String,
    pub stream: i32,
}

#[allow(clippy::needless_lifetimes)]
#[cfg_attr(feature = "mock", mockall::automock)]
pub trait SceneApi {
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
    fn camera_current_timestamp(&self, entity: &str, stream: i32) -> Option<u64>;
    fn set_streamer_frame_metadata(&self, data: &[u8]) -> Result<(), String>;
    fn load_project(&self, path: &str);
    fn start_streamer(&self);
    fn stop_streamer(&self);
    fn configure_texture_streaming(
        &self,
        config: &TextureStreamingConfig,
    ) -> Result<(), TextureStreamingError>;
    fn start_texture_streaming(
        &self,
        entity_id: &str,
        stream: i32,
    ) -> Result<(), TextureStreamingError>;
    fn stop_texture_streaming(
        &self,
        entity_id: &str,
        stream: i32,
    ) -> Result<(), TextureStreamingError>;
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
    fn joystick_state_from_settings_backed_name(&self, settings_key: &str)
        -> Option<JoystickState>;
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
    ) -> Result<crate::plugin_h::OdenRotation, SceneParamError>;
    fn set_output_alignment_rotation(
        &self,
        entity: &str,
        rotation: crate::plugin_h::OdenRotation,
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
    #[allow(clippy::type_complexity)]
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
    ) -> Option<RemoteCalibrationResult>;
    fn camera_crop(&self, entity: &str, stream: i32) -> Option<CameraCropData>;
    fn set_camera_crop(
        &self,
        entity: &str,
        stream: i32,
        crop: &CameraCropData,
    ) -> Result<(), SceneParamError>;
    fn camera_hard_crop(
        &self,
        entity: &str,
        stream: i32,
    ) -> Result<crate::math::Vec4, SceneParamError>;
    fn set_camera_hard_crop(
        &self,
        entity: &str,
        stream: i32,
        crop: &Vec4,
    ) -> Result<(), SceneParamError>;
    fn drop_detector_timeout(&self, entity: &str, stream: i32) -> Result<f32, SceneParamError>;
    fn set_drop_detector_timeout(
        &self,
        entity: &str,
        stream: i32,
        timeout_ms: f32,
    ) -> Result<(), SceneParamError>;
    fn show_no_signal_screen(&self, entity: &str, stream: i32) -> Result<bool, SceneParamError>;
    fn set_show_no_signal_screen(
        &self,
        entity: &str,
        stream: i32,
        show: bool,
    ) -> Result<(), SceneParamError>;
    fn entity_scale(&self, entity: &str) -> Result<Vec3, SceneParamError>;
    fn set_entity_scale(&self, entity: &str, scale: &Vec3) -> Result<(), SceneParamError>;
    fn raw_record_folder(&self) -> Result<String, SceneParamError>;
    fn set_raw_record_folder(&self, folder_path: &str) -> Result<(), SceneParamError>;
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
    fn flexbox_rotation(
        &self,
        entity_id: &str,
    ) -> Result<Vec<OdenFlexboxRotation>, SceneParamError>;
    fn performance_metrics<'a>(&'a self) -> Result<PerformanceMetrics<'a>, SceneParamError>;
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
        bind_device: &str,
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

/// For all network link related functions the `entity` is an [`Option`].
/// If `entity` is a [`Some`] value, the function will get/set the network config
/// for the Remote Streamer entity with the name `entity`.
/// If `entity` is [`None`] and the plugin is running in Oden Streamer, the function
/// will get/set the Streamer output network config.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_scene_api {
    ($interface:ident) => {
        // For proper docs links because we can't use $interface directly in doc comments
        #[allow(unused_imports)]
        use $interface as scene_api_impl;

        #[inherent::inherent]
        impl $crate::scene_api::SceneApi for $interface<'_> {

            /// Prevents the user from moving the viewport camera using the mouse in the player window.
            /// Needs to be called every frame (it prevents mouse movement in the frame it is called).
            /// This is useful e.g. when creating GUIs that rely on the view being set up in a specific way.
            pub fn inhibit_mouse_look(&self) {
                unsafe {
                    if let Some(inhibit_mouse_look) = (*self.inner).inhibitMouseLook {
                        inhibit_mouse_look();
                    } else {
                        panic!(
                            "This version of Oden is too old to have the inhibit_mouse_look function"
                        );
                    }
                }
            }

            /// Returns the horizon (as Euler angles, radians, XYZ) for the named `entity`, or [`None`] if
            /// `entity` is not a `Stitched Video` entity.
            pub fn horizon(&self, entity: &str) -> Option<$crate::math::Vec3> {
                unsafe {
                    if let Some(horizon) = (*self.inner).getHorizon {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        let mut h = $crate::math::Vec3::default();
                        if horizon(entity.as_ptr(), &mut h as *mut _) {
                            Some(h)
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the horizon function");
                    }
                }
            }

            /// Set the horizon (as Eular angles, radians, xyz) for the named `entity`.
            ///
            /// Returns `true` if successful.
            pub fn set_horizon(&self, entity: &str, horizon: $crate::math::Vec3) -> bool {
                unsafe {
                    if let Some(set_horizon) = (*self.inner).setHorizon {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        set_horizon(entity.as_ptr(), horizon)
                    } else {
                        panic!("This version of Oden is too old to have the set_horizon function");
                    }
                }
            }

            /// Returns the height of the floor relative to the camera for a stitched video. Will return [`None`] if no stitched video with the specified name exists or if no horizon calibration exists.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Some(height) = api.calibrated_floor_height("My Stitched Video Name") {
            ///     // Do something with height
            /// }
            /// # }
            /// ```
            pub fn calibrated_floor_height(&self, entity: &str) -> Option<f32> {
                unsafe {
                    if let Some(get_calibrated_floor_height) = (*self.inner).getCalibratedFloorHeight {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                        let mut floor_height: f32 = 0.0;

                        if get_calibrated_floor_height(entity.as_ptr(),  &mut floor_height as *mut _) {
                            return Some(floor_height)
                        }
                    } else {
                        panic!("This version of Oden is too old to have the get_calibrated_floor_height function");
                    }
                }
                None
            }

            /// Returns the entity name from entity UUID or entity name
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let entity_name = api.entity_name("80db85c8-9971-43ce-0ee9-4cd112d90adf").unwrap();
            /// # }
            /// ```
            pub fn entity_name(&self, entity_id: &str) -> Option<String> {
                if let Some(entity_name) = unsafe { (*self.inner).getEntityName } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut size = unsafe {
                        let mut size = 0;
                        if !entity_name(entity_id.as_ptr(), std::ptr::null_mut(), &mut size as *mut _) {
                            return None;
                        }
                        size
                    };

                    if size == 0 {
                        return None;
                    }

                    let mut buf = vec![0_u8; size as usize];

                    unsafe {
                        if !entity_name(entity_id.as_ptr(), buf.as_mut_ptr() as *mut _, &mut size as *mut _) {
                            return None;
                        }
                    }

                    let s = std::ffi::CStr::from_bytes_with_nul(&buf).ok()?;

                    let s = s.to_str().ok()?;

                    Some(s.to_owned())
                } else {
                    panic!("This version of Oden is too old to have the entity_name function");
                }
            }

            /// Sets the stitching `distance` for a Stitched Video `entity`
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///       api.set_stitch_distance("entity_name", 20.0);
            /// # }
            /// ```
            pub fn set_stitch_distance(&self, entity: &str, distance: f32) -> bool {
                unsafe {
                    if let Some(set_stitch_distance) = (*self.inner).setStitchDistance {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        set_stitch_distance(entity.as_ptr(), distance)
                    } else {
                        panic!("This version of Oden is too old to have the set_stitch_distance function");
                    }
                }
            }

            /// Returns the metadata from the specified video `stream` in `entity`, or [`None`]
            /// if the arguments are invalid.
            ///
            /// See [`remote_streamer_metadata`](scene_api_impl::remote_streamer_metadata) to
            /// extract metadata from a Remote Streamer video stream.
            pub fn camera_metadata(&self, entity: &str, stream: i32) -> Option<$crate::CameraMetadata> {
                if let Some(camera_metadata) = unsafe { (*self.inner).getCameraMetadata } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    let mut res = $crate::CameraMetadata {
                        __bindgen_anon_1: $crate::plugin_h::OdenCameraMetadata_s__bindgen_ty_1 {
                            availableMetadata: 0,
                        },
                        metadata: [0; 200usize],
                        timestamp: 0,
                    };

                    if unsafe { camera_metadata(entity.as_ptr(), stream, &mut res as *mut _) } {
                        Some(res)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_temperature function");
                }
            }

            /// Returns the metadata from the specified Remote Streamer `entity`, or [`None`] if the
            /// arguments are invalid or there is no such entity.
            ///
            /// See [`set_streamer_frame_metadata`](scene_api_impl::set_streamer_frame_metadata) for how to embed data in the stream.
            pub fn remote_streamer_metadata(&self, remote_streamer_entity: &str) -> Option<$crate::CameraMetadata> {
                self.camera_metadata(remote_streamer_entity, 1)
            }

            /// Returns the camera temperature from certain industrial machine vision cameras, or [`None`]
            /// if the arguments are invalid.
            pub fn camera_temperature(&self, entity: &str, stream: i32) -> Option<f32> {
                if let Some(camera_temperature) = unsafe { (*self.inner).getCameraTemperature } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut temperature = 0.0;

                    if unsafe { camera_temperature(entity.as_ptr(), stream, &mut temperature as *mut _) } {
                        Some(temperature)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_temperature function");
                }
            }

            /// Returns the current camera frames per second or [`None`] if the arguments are invalid
            pub fn camera_fps(&self, entity: &str, stream: i32) -> Option<f32> {
                if let Some(camera_fps) = unsafe { (*self.inner).getCameraFps } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut fps = 0.0;

                    if unsafe { camera_fps(entity.as_ptr(), stream, &mut fps as *mut _) } {
                        Some(fps)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_fps function");
                }
            }

            /// Returns the last valid timestamp for a camera in nanoseconds, or [`None`]
            /// if no such `entity` and `stream` available.
            /// /// The Drop Detector MUST be enabled for this `stream`, otherwise [`None`] is returned
            pub fn camera_last_valid_timestamp(&self, entity: &str, stream: i32) -> Option<u64> {
                if let Some(camera_last_valid_timestamp) =
                    unsafe { (*self.inner).getCameraLastValidTimestamp }
                {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut last_valid_timestamp = 0;

                    if unsafe {
                        camera_last_valid_timestamp(
                            entity.as_ptr(),
                            stream,
                            &mut last_valid_timestamp as *mut _,
                        )
                    } {
                        Some(last_valid_timestamp)
                    } else {
                        None
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the camera_last_valid_timestamp function"
                    );
                }
            }

            /// Enables or disables an `entity` based on the supplied `enable` bool
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.set_entity_enabled("entity_name", true);
            /// # }
            /// ```
            pub fn set_entity_enabled(&self, entity: &str, enable: bool) {
                unsafe {
                    if let Some(set_entity_enabled) = (*self.inner).setEntityEnabled {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                        set_entity_enabled(entity.as_ptr(), enable);
                    } else {
                        panic!("This version of Oden is too old to have the set_entity_enabled function")
                    };
                };
            }

            /// Returns the current timestamp for a camera in nanoseconds, or [`None`]
            /// if no such `entity` and `stream` available.
            /// The Drop Detector MUST be enabled for this `stream`, otherwise [`None`] is returned
            pub fn camera_current_timestamp(&self, entity: &str, stream: i32) -> Option<u64> {
                if let Some(camera_current_timestamp) = unsafe { (*self.inner).getCameraCurrentTimestamp } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    let mut current_timestamp = 0;

                    if unsafe {
                        camera_current_timestamp(entity.as_ptr(), stream, &mut current_timestamp as *mut _)
                    } {
                        Some(current_timestamp)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_current_timestamp function");
                }
            }

            /// Sets the user H.264/H.265 SEI data into the encoded bitstream on Oden Streamer for
            /// the current frame.
            ///
            /// This can be used to send messages synchronized with the video stream from Streamer to
            /// a Remote Streamer entity.
            ///
            /// You can receive the message from a Remote Streamer with [`camera_metadata`](scene_api_impl::camera_metadata).
            ///
            /// You are limited to messages less or equal to 200 bytes and can only be called once per frame. If called more
            /// than once an error will be returned as well as logged in the console.
            pub fn set_streamer_frame_metadata(&self, data: &[u8]) -> Result<(), String> {
                if let Some(set_streamer_frame_metadata) = unsafe { (*self.inner).setStreamerFrameMetadata } {
                    let res = unsafe {
                        set_streamer_frame_metadata(
                            data.as_ptr() as *const std::os::raw::c_char,
                            data.len() as i32,
                        )
                    };
                    return if res {
                        Ok(())
                    } else {
                        Err("Error: Metadata has not been set, check error in console".to_string())
                    };
                } else {
                    panic!("This version of Oden is too old to have the set_streamer_frame_metadata function");
                }
            }

            /// Loads a project from the specified `path`.
            ///
            /// Absolute and project-root-relative paths are supported.
            /// The use of environment variables are supported as well. To use a environment variable set
            /// the name between two '%' characters, for example `%HOME%/Folder/myproject.vproj`
            ///
            /// **This will close the current project.**
            /// **Call this from a global plugin if you want to retain the plugin state.**
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.load_project("player.vproj");
            /// # }
            /// ```
            pub fn load_project(&self, path: &str) {
                if let Some(load_project) = unsafe { (*self.inner).loadProject } {
                    let path = std::ffi::CString::new(path.trim_end_matches('\0')).unwrap();
                    unsafe { load_project(path.as_ptr()) }
                } else {
                    panic!("This version of Oden is too old to have the load_project function");
                }
            }

            /// Starts the Streamer video streaming.
            pub fn start_streamer(&self) {
                if let Some(start_streamer) = unsafe { (*self.inner).startStreamer } {
                    unsafe { start_streamer() }
                } else {
                    panic!("This version of Oden is too old to have the start_streamer function");
                }
            }

            /// Stops the Streamer video streaming.
            pub fn stop_streamer(&self) {
                if let Some(stop_streamer) = unsafe { (*self.inner).stopStreamer } {
                    unsafe { stop_streamer() }
                } else {
                    panic!("This version of Oden is too old to have the stop_streamer function");
                }
            }

            /// Configures the OdenVR window streaming with the given `config`.
            ///
            /// A value of `0` in [`width`](crate::scene_api::TextureStreamingConfig::width) /
            /// [`height`](crate::scene_api::TextureStreamingConfig::height) means
            /// "use the source resolution", `0` in
            /// [`frame_rate`](crate::scene_api::TextureStreamingConfig::frame_rate) means
            /// "inherit the render rate", and `0` in
            /// [`bitrate_kbps`](crate::scene_api::TextureStreamingConfig::bitrate_kbps)
            /// selects a sane default. An empty
            /// [`entity_id`](crate::scene_api::TextureStreamingConfig::entity_id)
            /// captures the screen buffer; otherwise the
            /// ([`entity_id`](crate::scene_api::TextureStreamingConfig::entity_id),
            /// [`stream`](crate::scene_api::TextureStreamingConfig::stream)) pair targets
            /// a specific video capture stream.
            ///
            /// Use [`start_texture_streaming`](Self::start_texture_streaming) and
            /// [`stop_texture_streaming`](Self::stop_texture_streaming) to start/stop the texture streaming
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.configure_texture_streaming(&oden_plugin_rs::scene_api::TextureStreamingConfig {
            ///     width: 0,
            ///     height: 0,
            ///     frame_rate: 0,
            ///     bitrate_kbps: 0,
            ///     codec: oden_plugin_rs::scene_api::TextureStreamingCodec::H264,
            ///     output_pipeline: "h264parse ! rtph264pay ! udpsink host=127.0.0.1 port=5000".to_string(),
            ///     entity_id: String::new(),
            ///     stream: 0,
            /// }).unwrap();
            /// # }
            /// ```
            pub fn configure_texture_streaming(
                &self,
                config: &$crate::scene_api::TextureStreamingConfig,
            ) -> Result<(), $crate::TextureStreamingError> {
                let configure = match unsafe { (*self.inner).configureTextureStreaming } {
                    Some(configure) => configure,
                    None => panic!(
                        "This version of Oden is too old to have the configure_texture_streaming function"
                    ),
                };

                let pipeline = std::ffi::CString::new(
                    config.output_pipeline.trim_end_matches('\0'),
                )
                .map_err(|_| $crate::TextureStreamingError::OdenTextureStreamingErrorArgumentContainsNul)?;

                let entity_id = std::ffi::CString::new(
                    config.entity_id.trim_end_matches('\0'),
                )
                .map_err(|_| $crate::TextureStreamingError::OdenTextureStreamingErrorArgumentContainsNul)?;

                let codec = match config.codec {
                    $crate::scene_api::TextureStreamingCodec::H264 =>
                        $crate::plugin_h::OdenPluginCodec_e_OdenPluginCodecH264,
                    $crate::scene_api::TextureStreamingCodec::H265 =>
                        $crate::plugin_h::OdenPluginCodec_e_OdenPluginCodecH265,
                };

                let raw = $crate::plugin_h::OdenTextureStreamingConfig {
                    width: config.width,
                    height: config.height,
                    frame_rate: config.frame_rate,
                    bitrate_kbps: config.bitrate_kbps,
                    codec,
                    output_pipeline: pipeline.as_ptr(),
                    entity_id: entity_id.as_ptr(),
                    stream: config.stream,
                    reserved_i32: [0; 8],
                    reserved_ptr: [std::ptr::null_mut(); 4],
                };

                match unsafe { configure(&raw) } {
                    $crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorOk => Ok(()),
                    err => Err($crate::TextureStreamingError::from_raw(err)),
                }
            }

            /// Requests a texture share start for the given `(entity_id, stream)` source.
            ///
            /// An empty `entity_id` targets the screen-buffer share; otherwise the
            /// `(entity_id, stream)` pair resolves to a specific video capture
            /// stream. The target must have been configured with
            /// [`configure_texture_streaming`](Self::configure_texture_streaming) first.
            ///
            /// The start is asynchronous: `Ok(())` means the request was accepted,
            /// not that the stream is running. Use
            /// [`stop_texture_streaming`](Self::stop_texture_streaming) to stop.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.start_texture_streaming("Vehicle@cam", 0).unwrap();
            /// # }
            /// ```
            pub fn start_texture_streaming(
                &self,
                entity_id: &str,
                stream: i32,
            ) -> Result<(), $crate::TextureStreamingError> {
                let start = match unsafe { (*self.inner).startTextureStreaming } {
                    Some(start) => start,
                    None => panic!(
                        "This version of Oden is too old to have the start_texture_streaming function"
                    ),
                };

                let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0'))
                    .map_err(|_| $crate::TextureStreamingError::OdenTextureStreamingErrorArgumentContainsNul)?;

                match unsafe { start(entity_id.as_ptr(), stream) } {
                    $crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorOk => Ok(()),
                    err => Err($crate::TextureStreamingError::from_raw(err)),
                }
            }

            /// Requests a stop of a texture share previously started with
            /// [`start_texture_streaming`](Self::start_texture_streaming).
            ///
            /// The stop is asynchronous: `Ok(())` means the request was accepted.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.stop_texture_streaming("Vehicle@cam", 0).unwrap();
            /// # }
            /// ```
            pub fn stop_texture_streaming(
                &self,
                entity_id: &str,
                stream: i32,
            ) -> Result<(), $crate::TextureStreamingError> {
                let stop = match unsafe { (*self.inner).stopTextureStreaming } {
                    Some(stop) => stop,
                    None => panic!(
                        "This version of Oden is too old to have the stop_texture_streaming function"
                    ),
                };

                let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0'))
                    .map_err(|_| $crate::TextureStreamingError::OdenTextureStreamingErrorArgumentContainsNul)?;

                match unsafe { stop(entity_id.as_ptr(), stream) } {
                    $crate::plugin_h::OdenTextureStreamingError_e_OdenTextureStreamingErrorOk => Ok(()),
                    err => Err($crate::TextureStreamingError::from_raw(err)),
                }
            }

            /// Returns the state of a camera stream/input, or [`None`] if the `entity_id` and `stream` do not exist.
            ///
            /// State may be `CameraStreamStateUnknown` if the input does not support states.
            ///
            /// This function only works on some input types. All states may not be supported.
            pub fn camera_stream_state(
                &self,
                entity_id: &str,
                stream: i32,
            ) -> Option<$crate::CameraStreamState> {
                if let Some(camera_stream_state) = unsafe { (*self.inner).getCameraStreamState } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    let mut stream_state = $crate::CameraStreamStateUnknown;

                    if unsafe { camera_stream_state(entity_id.as_ptr(), stream, &mut stream_state) } {
                        Some(stream_state)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_stream_state function");
                }
            }

            /// Sets the state of a camera stream/input.
            ///
            /// Returns `true` if `stream` exists and it supports state changes.
            /// Returns true also if `stream` was already in the desired state.
            ///
            /// This function only works on some input types. All states may not be supported.
            pub fn set_camera_stream_state(
                &self,
                entity_id: &str,
                stream: i32,
                stream_state: $crate::CameraStreamState,
            ) -> bool {
                if let Some(set_camera_stream_state) = unsafe { (*self.inner).setCameraStreamState } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    unsafe { set_camera_stream_state(entity_id.as_ptr(), stream, stream_state) }
                } else {
                    panic!("This version of Oden is too old to have the set_camera_stream_state function");
                }
            }

            /// Returns the IDs for all available scenes.
            /// The length of the vector is the number of available scenes.
            pub fn all_scene_ids(&self) -> Vec<i32> {
                if let Some(all_scene_id) = unsafe { (*self.inner).getAllSceneIds } {
                    let mut scene_ids: Vec<i32> = Vec::new();
                    let mut count = 0;
                    if (unsafe { all_scene_id(std::ptr::null_mut(), &mut count as *mut _) }) {
                        scene_ids.resize(count as usize, 0);
                        unsafe { all_scene_id(scene_ids.as_mut_ptr(), &mut count as *mut _) };
                    }
                    scene_ids
                } else {
                    panic!("This version of Oden is too old to have the all_scene_id function");
                }
            }

            /// Returns the currently selected scene ID.
            pub fn selected_scene_id(&self) -> Option<i32> {
                if let Some(selected_scene_id) = unsafe { (*self.inner).getSelectedSceneId } {
                    let scene_id = unsafe { selected_scene_id() };

                    Some(scene_id)
                } else {
                    panic!("This version of Oden is too old to have the selected_scene_id function");
                }
            }

            /// Sets the active scene to the scene which correspond to the supplied `scene_id`.
            ///
            /// Returns `true` if the scene was set.
            pub fn set_selected_scene(&self, scene_id: i32) -> bool {
                if let Some(set_selected_scene) = unsafe { (*self.inner).setSelectedScene } {
                    unsafe { set_selected_scene(scene_id) }
                } else {
                    panic!("This version of Oden is too old to have the set_selected_scene function");
                }
            }

            /// Retrieves the name of the `scene_id`, or [`None`] if the `scene_id` does not exist.
            pub fn scene_name(&self, scene_id: i32) -> Option<String> {
                if let Some(scene_name) = unsafe { (*self.inner).getSceneName } {
                    let mut size = unsafe {
                        let mut size = 0;
                        if !scene_name(scene_id, std::ptr::null_mut(), &mut size as *mut _) {
                            return None;
                        }
                        size
                    };

                    if size == 0 {
                        return None;
                    }

                    let mut buf = vec![0_u8; size as usize];

                    unsafe {
                        if !scene_name(scene_id, buf.as_mut_ptr() as *mut _, &mut size as *mut _) {
                            return None;
                        }
                    }

                    let s = std::ffi::CStr::from_bytes_with_nul(&buf).ok()?;

                    let s = s.to_str().ok()?;

                    Some(s.to_owned())
                } else {
                    panic!("This version of Oden is too old to have the scene_name function");
                }
            }

            /// Starts a raw recording of the current scene. This saves data from entities that support it (may
            /// include video, audio, and plugin data) to disk in a raw format that can be played back later.
            ///
            /// See [`push_recording_bytes`](crate::UpdateParams::push_recording_bytes) and
            /// [`push_recording_data`](crate::UpdateParams::push_recording_data) for how to save data from
            /// a plugin.
            pub fn raw_record_start(&self) {
                if let Some(raw_record_start) = unsafe { (*self.inner).rawRecordStart } {
                    unsafe {
                        raw_record_start();
                    }
                } else {
                    panic!("This version of Oden is too old to have the raw_record_start function");
                }
            }

            /// Stops the raw recording.
            ///
            /// Make sure to properly stop the raw recording before closing the application to prevent
            /// data corruption.
            pub fn raw_record_stop(&self) {
                if let Some(raw_record_stop) = unsafe { (*self.inner).rawRecordStop } {
                    unsafe {
                        raw_record_stop();
                    }
                } else {
                    panic!("This version of Oden is too old to have the raw_record_stop function");
                }
            }

            /// Sets the raw recording ring buffer `size` of the specified `entity` in MB.
            ///
            /// See [`raw_record_entity_ring_buffer_enable`](scene_api_impl::raw_record_entity_ring_buffer_enable).
            pub fn set_raw_record_entity_ring_buffer_size(&self, entity: &str, size: i32) -> bool {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(set_raw_record_entity_ring_buffer_size) =
                    unsafe { (*self.inner).rawRecordSetEntityRingBufferSize }
                {
                    unsafe { set_raw_record_entity_ring_buffer_size(entity.as_ptr(), size) }
                } else {
                    panic!("This version of Oden is too old to have the set_raw_record_entity_ring_buffer_size function");
                }
            }

            /// Enable the ring buffer of the specified `entity`.
            /// This makes the raw recording loop when the specified size has been reached.
            ///
            /// Returns `true` if the specified `entity_id` has raw recording capabilities.
            ///
            pub fn raw_record_entity_ring_buffer_enable(&self, entity: &str) -> bool {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(raw_record_entity_ring_buffer_enable) =
                    unsafe { (*self.inner).rawRecordEntityRingBufferEnable }
                {
                    unsafe { raw_record_entity_ring_buffer_enable(entity.as_ptr()) }
                } else {
                    panic!("This version of Oden is too old to have the raw_record_entity_ring_buffer_enable function");
                }
            }

            /// Returns the path of the last raw recording
            ///
            /// # Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let path = api.raw_record_last_path();
            /// # }
            /// ```
            pub fn raw_record_last_path(&self) -> Option<String> {
                if let Some(raw_record_last_path) = unsafe { (*self.inner).rawRecordGetLastPath } {
                    let mut size = unsafe {
                        let mut size = 0;
                        if !raw_record_last_path(std::ptr::null_mut(), &mut size as *mut _) {
                            return None;
                        }
                        size
                    };

                    if size == 0 {
                        return None;
                    }

                    let mut path = vec![0_u8; size as usize];

                    unsafe {
                        if !raw_record_last_path(path.as_mut_ptr() as *mut _, &mut size as *mut _) {
                            return None;
                        }
                    }

                    let s = std::ffi::CStr::from_bytes_with_nul(&path).ok()?;

                    let s = s.to_str().ok()?;

                    Some(s.to_owned())
                } else {
                    panic!("This version of Oden is too old to have the raw_record_last_path function");
                }
            }

            /// Returns the world transformation matrix for the specified `entity`, or [`None`] if the
            /// `entity` does not exist.
            ///
            /// The matrix determines how the entity is positioned, rotated, scaled etc. in the world.
            pub fn entity_world_matrix(&self, entity: &str) -> Option<$crate::math::Matrix4> {
                if let Some(entity_world_matrix) = unsafe { (*self.inner).getEntityWorldMatrix } {
                    let mut mat = $crate::math::Matrix4::default();
                    let c_entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    if unsafe { entity_world_matrix(c_entity.as_ptr(), &mut mat as *mut _) } {
                        Some(mat)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the entity_world_matrix function");
                }
            }

            /// Returns `true` if the given `joystick_id` is available in the system.
            ///
            /// Note: On first run it may return `false` even if joystick is present.
            /// This can happen if Oden starts before the system has initialized the joystick.
            pub fn is_joystick_present(&self, joystick_id: i32) -> bool {
                unsafe {
                    if let Some(is_joystick_present) = (*self.inner).isJoystickPresent {
                        is_joystick_present(joystick_id)
                    } else {
                        panic!("This version of Oden is too old to have the is_joystick_present function");
                    }
                }
            }

            /// Returns the joystick state for the given `joystick_id`, or [`None`] if the joystick
            /// is not present.
            ///
            /// The axes are in the range `[-1, 1]`.
            /// The buttons are `true` when pressed and `false` when not pressed.
            pub fn joystick_state(&self, joystick_id: i32) -> Option<$crate::scene_api::JoystickState> {
                unsafe {
                    let mut state = $crate::plugin_h::OdenJoystickState_s::default();
                    if let Some(get_joystick_state) = (*self.inner).getJoystickState {
                        if get_joystick_state(joystick_id, &mut state as *mut _) {
                            Some($crate::scene_api::JoystickState::from_c(state))
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the joystick_state function");
                    }
                }
            }

            /// Returns the current state of the gamepad with the given `joystick_id`, or
            /// [`None`] if no gamepad is connected.
            ///
            /// The returned [`GamepadState`](crate::GamepadState) is indexed by
            /// [`GamepadButton`](crate::scene_api::GamepadButton) (SDL3 layout) and
            /// [`GamepadAxis`](crate::scene_api::GamepadAxis), and provides helper
            /// methods [`button_pressed`](crate::GamepadState::button_pressed) and
            /// [`axis_value`](crate::GamepadState::axis_value).
            ///
            /// # Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///       if let Some(state) = api.gamepad_state(0) {
            ///           if state.button_pressed(oden_plugin_rs::GamepadButton::South) {
            ///               println!("South pressed");
            ///           }
            ///           let lx = state.axis_value(oden_plugin_rs::GamepadAxis::LeftX);
            ///           let ly = state.axis_value(oden_plugin_rs::GamepadAxis::LeftY);
            ///           println!("Left stick: ({lx:.2}, {ly:.2})");
            ///       }
            /// # }
            /// ```
            ///
            /// Buttons return `true` when pressed; axes range from `-1.0` to `1.0`.
            pub fn gamepad_state(&self, joystick_id: i32) -> Option<$crate::GamepadState> {
                unsafe {
                    let mut state = $crate::GamepadState::default();
                    if let Some(get_gamepad_state_v2) = (*self.inner).getGamepadStateV2 {
                        if get_gamepad_state_v2(joystick_id, &mut state as *mut _) {
                            Some(state)
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the gamepad_state function");
                    }
                }
            }

            /// Retrieves joystick state based on a joystick name stored in settings.
            ///
            /// # Arguments
            ///
            /// * `settings_key` - The settings key where the selected joystick name is stored.
            ///
            /// # Returns
            ///
            /// * `Some(JoystickState)` if the joystick is present, `None` otherwise.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Some(state) = api.joystick_state_from_settings_backed_name("joystick_name") {
            ///     // Use the joystick state
            /// }
            /// # }
            /// ```
            pub fn joystick_state_from_settings_backed_name(&self, settings_key: &str) -> Option<$crate::scene_api::JoystickState> {
                if let Some(joystick) = self.read_string(settings_key) {
                    for i in 0..=($crate::JOYSTICK_LAST as i32) {
                        if self.is_joystick_present(i) {
                            if let Some(state) = self.joystick_state(i) {
                                if state.name == joystick {
                                    return Some(state);
                                }
                            }
                        }
                    }

                }

                None
            }

            /// Returns the statistics for a given video stream.
            ///
            /// Only works for `Remote Streamer` inputs.
            ///
            /// Make sure to specify `1` as the `stream`.
            pub fn stream_statistics(
                &self,
                entity: &str,
                stream: i32,
                max_num_stats: usize,
            ) -> Vec<$crate::StreamStatistics> {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                unsafe {
                    if let Some(stream_statistics) = (*self.inner).getStreamStatistics {
                        // Get number of available stats structs
                        let mut available_structs = 0;
                        if stream_statistics(
                            entity.as_ptr(),
                            stream,
                            std::ptr::null_mut(),
                            &mut available_structs as *mut _,
                        ) {
                            let mut limited_count = std::cmp::min(available_structs, max_num_stats as i32);
                            let mut stats = vec![$crate::StreamStatistics::default(); limited_count as usize];
                            let stats_slice = stats.as_mut_slice();
                            if stream_statistics(
                                entity.as_ptr(),
                                stream,
                                stats_slice.as_mut_ptr() as *mut _,
                                &mut limited_count as *mut _,
                            ) {
                                stats
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        }
                    } else {
                        panic!("This version of Oden is too old to have the stream_statistics function");
                    }
                }
            }

            /// Returns the extended statistics for a given video stream.
            ///
            /// Only works for `Remote Streamer` inputs.
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Some(stats) = api.stream_statistics_ex("Remote Streamer") {
            ///     // Do something with the statistics
            /// }
            /// # }
            /// ```
            pub fn stream_statistics_ex(
                &self,
                entity: &str,
            ) -> Option<Box<$crate::StreamStatisticsEx>> {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                unsafe {
                    if let Some(stream_statistics_ex) = (*self.inner).getStreamStatisticsEx {
                        let mut res = Box::new($crate::StreamStatisticsEx{
                            frameStatistics: [$crate::plugin_h::OdenReceiveFrameStatistics::default(); 64],
                            interfaceStatisticsCount:0,
                            interfaceStatistics:[$crate::plugin_h::OdenReceiveInterfaceStatistics{
                                ip: 0,
                                port: 0,
                                frameStatistics: [$crate::plugin_h::OdenReceiveFrameStatistics::default(); 64],
                            }; 64],
                        });

                        if stream_statistics_ex(
                            entity.as_ptr(),
                            1,
                            &mut *res as *mut _,
                        ) {
                            Some(res)
                        } else{
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the stream_statistics_ex function");
                    }
                }
            }

            /// Returns true if there is an entity with given name in the scene graph.
            pub fn has_entity_with_name(&self, entity: &str) -> bool {
                if let Some(has_entity_with_name) = unsafe { (*self.inner).hasEntityWithName } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    unsafe { has_entity_with_name(entity.as_ptr()) }
                } else {
                    panic!("This version of Oden is too old to have the has_entity_with_name function");
                }
            }

            /// Gets the current Streamer bitrate, or [`None`] if called in Player
            /// or if the Streamer is inactive.
            pub fn streamer_bitrate_mbps(&self) -> Option<f32> {
                if let Some(streamer_bitrate_mbps) = unsafe { (*self.inner).getStreamerBitrateMbps } {
                    let mut val: f32 = 0.0;
                    let res = unsafe { streamer_bitrate_mbps(&mut val as *mut f32) };
                    if res {
                        Some(val)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the streamer_bitrate_mbps function");
                }
            }

            /// Sets the maximum bandwidth the Streamer is allowed to output.
            /// `bitrate` MUST be in the range `(0.0, 200.0]`.
            ///
            /// Returns `true` if called in Oden Streamer and `bitrate` is within range.
            pub fn set_streamer_max_bandwidth(&self, bitrate: f32) -> bool {
                if let Some(set_streamer_max_bandwidth) = unsafe { (*self.inner).setStreamerMaxBandwidth } {
                    unsafe { set_streamer_max_bandwidth(f32::clamp(bitrate, 0.0, 200.0)) }
                } else {
                    panic!("This version of Oden is too old to have the set_streamer_max_bandwidth function");
                }
            }

            /// Returns the number of links for the specified `entity`.
            pub fn link_count(&self, entity: Option<&str>) -> Result<i32, $crate::LinkError> {
                if let Some(link_count) = unsafe { (*self.inner).getLinkCount } {
                    let mut count: i32 = 0;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_count(entity.as_ptr(), &mut count) }
                        }
                        None => unsafe { link_count(std::ptr::null(), &mut count) },
                    };
                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(count),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_count function");
                }
            }

            /// Sets the link mode for the given link `index`.
            ///
            /// [`LinkMode::OdenLinkModeSender`](crate::LinkMode::OdenLinkModeSender) initiates the connection.
            ///
            /// [`LinkMode::OdenLinkModeReceiver`](crate::LinkMode::OdenLinkModeReceiver) listens and replies to incoming traffic.
            pub fn set_link_mode(
                &self,
                entity: Option<&str>,
                index: i32,
                mode: $crate::LinkMode,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_mode) = unsafe { (*self.inner).setLinkMode } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_mode(entity.as_ptr(), index, mode) }
                        }
                        None => unsafe { set_link_mode(std::ptr::null(), index, mode) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_mode function");
                }
            }

            /// Returns the link mode for the given link `index`.
            /// The mode specifies if the link acts as sender or receiver.
            /// Sender links initiates connections.
            /// Receiver links listen and send traffic as replies.
            pub fn link_mode(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<$crate::LinkMode, $crate::LinkError> {
                if let Some(link_mode) = unsafe { (*self.inner).getLinkMode } {
                    let mut mode = $crate::LinkMode::OdenLinkModeReceiver;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_mode(entity.as_ptr(), index, &mut mode) }
                        }
                        None => unsafe { link_mode(std::ptr::null(), index, &mut mode) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(mode),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_mode function");
                }
            }

            /// Sets the bind IP for the given link `index`.
            /// `ip` MUST be a valid IPv4 or IPv6 address.
            pub fn set_link_bind_ip(
                &self,
                entity: Option<&str>,
                index: i32,
                ip: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_bind_ip) = unsafe { (*self.inner).setLinkBindIp } {
                    let ip = std::ffi::CString::new(ip.trim_end_matches('\0')).unwrap();

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_bind_ip(entity.as_ptr(), index, ip.as_ptr()) }
                        }
                        None => unsafe { set_link_bind_ip(std::ptr::null(), index, ip.as_ptr()) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_bind_ip function");
                }
            }

            /// Returns the bind IP address for the given link `index`.
            pub fn link_bind_ip(&self, entity: Option<&str>, index: i32) -> Result<String, $crate::LinkError> {
                if let Some(link_bind_ip) = unsafe { (*self.inner).getLinkBindIp } {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| { unsafe { link_bind_ip(entity, index, ptr, size) } },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_bind_ip function");
                }
            }

            /// Sets the receive `port` for the given link `index`.
            /// Accepted port range is `[1, 65535]`.
            ///
            /// **Note**: The operating system might not allow ports under <= `1000` if running as a
            /// regular user.
            pub fn set_link_receive_port(
                &self,
                entity: Option<&str>,
                index: i32,
                port: u16,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_receive_port) = unsafe { (*self.inner).setLinkReceivePort } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_receive_port(entity.as_ptr(), index, port as i32) }
                        }
                        None => unsafe { set_link_receive_port(std::ptr::null(), index, port as i32) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_receive_port function");
                }
            }

            /// Returns the receive port for the given link `index`.
            pub fn link_receive_port(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<u16, $crate::LinkError> {
                if let Some(link_receive_port) = unsafe { (*self.inner).getLinkReceivePort } {
                    let mut port: i32 = 0;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_receive_port(entity.as_ptr(), index, &mut port) }
                        }
                        None => unsafe { link_receive_port(std::ptr::null(), index, &mut port) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(port as u16),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_receive_port function");
                }
            }

            /// Sets the destination `ip` for the given link `index`.
            /// `ip` must be a valid IPv4 address.
            pub fn set_link_destination_ip(
                &self,
                entity: Option<&str>,
                index: i32,
                ip: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_destination_ip) = unsafe { (*self.inner).setLinkDestinationIp } {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ip: *const std::os::raw::c_char| {
                            unsafe { set_link_destination_ip(entity, index, ip) }
                        },
                        entity,
                        index,
                        ip,
                    )
                } else {
                    panic!("This version of Oden is too old to have the set_link_destination_ip function");
                }
            }

            /// Returns the destination IP for the given link `index`.
            pub fn link_destination_ip(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_destination_ip) = unsafe { (*self.inner).getLinkDestinationIp } {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| {
                            unsafe { link_destination_ip(entity, index, ptr, size) }
                        },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_destination_ip function");
                }
            }

            /// Sets the destination `port` for the given link `index`.
            /// Accepted port range is `[1, 65535]`.
            ///
            /// **Note**: The operating system might not allow ports under <= `1000` if running as a
            /// regular user.
            pub fn set_link_destination_port(
                &self,
                entity: Option<&str>,
                index: i32,
                port: u16,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_destination_port) = unsafe { (*self.inner).setLinkDestinationPort } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_destination_port(entity.as_ptr(), index, port as i32) }
                        }
                        None => unsafe { set_link_destination_port(std::ptr::null(), index, port as i32) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_destination_port function");
                }
            }

            /// Returns the destination port for the given link `index`.
            pub fn link_destination_port(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<u16, $crate::LinkError> {
                if let Some(link_destination_port) = unsafe { (*self.inner).getLinkDestinationPort } {
                    let mut port: i32 = 0;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_destination_port(entity.as_ptr(), index, &mut port) }
                        }
                        None => unsafe { link_destination_port(std::ptr::null(), index, &mut port) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(port as u16),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_destination_port function");
                }
            }

            /// Enable or disable P2P mode for the given link `index`
            pub fn set_link_p2p_enabled(&self, entity: Option<&str>, index: i32, enabled: bool) -> Result<(), $crate::LinkError> {
                if let Some(set_link_p2p_enabled) = unsafe { (*self.inner).setLinkP2pEnabled } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_p2p_enabled(entity.as_ptr(), index, enabled) }
                        }
                        None => unsafe { set_link_p2p_enabled(std::ptr::null(), index, enabled) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_p2p_enabled function");
                }
            }

            /// Returns `true` if the given link `index` has P2P mode enabled.
            pub fn link_p2p_enabled(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<bool, $crate::LinkError> {
                if let Some(link_p2p_enabled) = unsafe { (*self.inner).getLinkP2pEnabled } {
                    let mut enabled: bool = false;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_p2p_enabled(entity.as_ptr(), index, &mut enabled) }
                        }
                        None => unsafe { link_p2p_enabled(std::ptr::null(), index, &mut enabled) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(enabled),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_p2p_enabled function");
                }
            }

            /// Returns the P2P status for the given link `index`.
            pub fn link_p2p_status(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<$crate::scene_api::LinkP2PStatus, $crate::LinkError> {
                if let Some(link_p2p_status) = unsafe { (*self.inner).getLinkP2pStatus } {
                    let mut status = $crate::LinkP2PStatusWrapper::default();

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { link_p2p_status(entity.as_ptr(), index, &mut status) }
                        }
                        None => unsafe { link_p2p_status(std::ptr::null(), index, &mut status) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(status.into()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_p2p_status function");
                }
            }

            /// Set the P2P STUN server address for the given link `index`.
            /// `ip` must be a valid DNS hostname or IP address.
            pub fn set_link_p2p_stun_server(
                &self,
                entity: Option<&str>,
                index: i32,
                addr: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_p2p_stun_server) = unsafe { (*self.inner).setLinkP2pStunServer } {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        addr: *const std::os::raw::c_char| {
                            unsafe { set_link_p2p_stun_server(entity, index, addr) }
                        },
                        entity,
                        index,
                        addr,
                    )
                } else {
                    panic!("This version of Oden is too old to have the set_link_p2p_stun_server function");
                }
            }

            /// Returns the P2P STUN server for the given link `index`.
            pub fn link_p2p_stun_server(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_p2p_stun_server) = unsafe { (*self.inner).getLinkP2pStunServer } {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| {
                            unsafe { link_p2p_stun_server(entity, index, ptr, size) }
                        },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_p2p_stun_server function");
                }
            }

            /// Returns the UUIDs of all entities in the project.
            pub fn all_entity_uuids(&self) -> Option<Vec<$crate::math::Uuid>> {
                unsafe {
                    if let Some(all_entity_uuids) = (*self.inner).getAllEntityUuids {
                        let mut uuid_count: i32 = 0;
                        if all_entity_uuids(std::ptr::null_mut(), &mut uuid_count as *mut i32) {
                            let mut uuids = vec![$crate::math::Uuid::default(); uuid_count as usize];
                            let uuids_slice = uuids.as_mut_slice();
                            if all_entity_uuids(
                                uuids_slice.as_mut_ptr() as *mut _,
                                &mut uuid_count as *mut i32,
                            ) {
                                Some(uuids)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the all_entity_uuids function");
                    }
                }
            }

            /// Returns the type name for the given `entity`.
            pub fn entity_type_name(&self, entity: &str) -> Option<String> {
                unsafe {
                    if let Some(entity_type_name) = (*self.inner).getEntityTypeName {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        let c_str = entity_type_name(entity.as_ptr());
                        if c_str.is_null() {
                            None
                        } else {
                            let res = std::ffi::CStr::from_ptr(c_str).to_string_lossy().into();

                            Some(res)
                        }
                    } else {
                        panic!("This version of Oden is too old to have the entity_type_name function");
                    }
                }
            }

            /// Returns basic statistics from a `Remote Streamer` with name `entity`.
            pub fn basic_statistics(&self, entity: &str) -> Option<$crate::plugin_h::OdenBasicStatistics> {
                unsafe {
                    if let Some(basic_statistics) = (*self.inner).getBasicStatistics {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        let mut stats = $crate::plugin_h::OdenBasicStatistics::default();

                        if basic_statistics(
                            entity.as_ptr(),
                            &mut stats as *mut $crate::plugin_h::OdenBasicStatistics,
                        ) {
                            Some(stats)
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the basic_statistics function");
                    }
                }
            }

            /// Returns stream statistics from a `Remote Streamer` with name `entity`.
            /// Consecutive calls will get same and/or overlapping stats.
            /// Result is per-frame statistics.
            pub fn remote_streamer_statistics(&self, remote_streamer_entity: &str) -> Option<Vec<$crate::plugin_h::OdenStreamStatistics>> {
                unsafe {
                    if let Some(stream_statistics) = (*self.inner).getStreamStatistics {
                        let entity = std::ffi::CString::new(remote_streamer_entity.trim_end_matches('\0')).unwrap();
                        let mut res:Vec<$crate::plugin_h::OdenStreamStatistics> = Vec::new();
                        let mut count = 0;
                        if stream_statistics(entity.as_ptr(), 1, std::ptr::null_mut(), &mut count as *mut _) {
                            res.resize(count as usize, $crate::plugin_h::OdenStreamStatistics::default());
                            stream_statistics(entity.as_ptr(), 1, res.as_mut_ptr(), &mut count as *mut _);
                        } else {
                            return None;
                        }
                        return Some(res);

                    } else {
                        panic!("This version of Oden is too old to have the remote_streamer_statistics function");
                    }
                }
            }

            /// Gets the Output Alignment ID that the specified Clone Stream uses to clone from a Remote Streamer,
            /// and the corresponding Remote Streamer UUID.
            ///
            /// Returns `true` on success.
            pub fn alignment_id(
                &self,
                clone_stream_entity: &str,
                clone_stream: i32,
            ) -> Option<$crate::plugin_h::OdenAlignmentId> {
                unsafe {
                    if let Some(get_alignment_id) = (*self.inner).getAlignmentId {
                        let clone_stream_entity = std::ffi::CString::new(clone_stream_entity.trim_end_matches('\0')).unwrap();

                        let mut alignment_id = $crate::plugin_h::OdenAlignmentId::default();

                        if get_alignment_id(
                            clone_stream_entity.as_ptr(),
                            clone_stream,
                            &mut alignment_id as *mut _,
                        ) {
                            return Some(alignment_id);
                        } else {
                            return None;
                        }
                    } else {
                        panic!("This version of Oden is too old to have the alignment_id function");
                    }
                }
            }

            /// Gets video placement information for a specific alignment ID in a Remote Streamer.
            /// `alignment_id` should contain the alignment ID and Remote Streamer of interest.
            ///
            /// Returns `true` on success.
            pub fn video_placement(
                &self,
                alignment_id: &$crate::plugin_h::OdenAlignmentId,
            ) -> Option<$crate::plugin_h::OdenVideoPlacement> {
                unsafe {
                    if let Some(get_video_placement) = (*self.inner).getVideoPlacement {
                        let mut video_placement_data = $crate::plugin_h::OdenVideoPlacement::default();

                        if get_video_placement(
                            alignment_id,
                            &mut video_placement_data as *mut _,
                        ) {
                            return Some(video_placement_data);
                        } else {
                            return None;
                        }
                    } else {
                        panic!("This version of Oden is too old to have the video_placement function");
                    }
                }
            }

            /// Sets the encryption private key path for a given `entity` and link `index`.
            /// The path should contain a valid base64 encoded key pair in JSON.
            /// For example:
            /// ```text
            ///  {
            ///      "private_key": "lt9pRvWSJhfTR7Dy5ktg52ZmVIxpdgNkD5k21X6BNzA=",
            ///      "public_key": "bYA9y3izP96Asu4LA4sUooRvBDi0xD4i9VikcUNP+Sk="
            ///  }
            /// ```
            pub fn set_link_encryption_private_key_path(
                &self,
                entity: Option<&str>,
                index: i32,
                key_path: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_encryption_private_key_path) =
                    unsafe { (*self.inner).setLinkEncryptionPrivateKeyPath }
                {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        key_path: *const std::os::raw::c_char| {
                            unsafe { set_link_encryption_private_key_path(entity, index, key_path) }
                        },
                        entity,
                        index,
                        key_path,
                    )
                } else {
                    panic!("This version of Oden is too old to have the set_link_encryption_private_key_path function");
                }
            }

            /// Returns the encryption private key path for the given `entity` and link `index`.
            pub fn link_encryption_private_key_path(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_encryption_private_key_path) =
                    unsafe { (*self.inner).getLinkEncryptionPrivateKeyPath }
                {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| {
                            unsafe { link_encryption_private_key_path(entity, index, ptr, size) }
                        },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_encryption_private_key_path function");
                }
            }

            /// Add an allowed public key for the specified `entity` and link `index`.
            pub fn add_link_encryption_allowed_public_key(
                &self,
                entity: Option<&str>,
                index: i32,
                key: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(add_link_encryption_allowed_public_key) =
                    unsafe { (*self.inner).addLinkEncryptionAllowedPublicKey }
                {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char, index: i32, key: *const std::os::raw::c_char| unsafe {
                            add_link_encryption_allowed_public_key(entity, index, key)
                        },
                        entity,
                        index,
                        key,
                    )
                } else {
                    panic!("This version of Oden is too old to have the add_link_encryption_allowed_public_key function");
                }
            }

            /// Removes the specified allowed public key for the specified `entity` and link `index`.
            pub fn remove_link_encryption_allowed_public_key(
                &self,
                entity: Option<&str>,
                index: i32,
                key: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(remove_link_encryption_allowed_public_key) =
                    unsafe { (*self.inner).removeLinkEncryptionAllowedPublicKey }
                {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char, index: i32, key: *const std::os::raw::c_char| unsafe {
                            remove_link_encryption_allowed_public_key(entity, index, key)
                        },
                        entity,
                        index,
                        key,
                    )
                } else {
                    panic!("This version of Oden is too old to have the remove_link_encryption_allowed_public_key function");
                }
            }

            /// Returns the count of added allowed public keys for the link `index`.
            pub fn link_encryption_allowed_public_keys_count(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<u16, $crate::LinkError> {
                if let Some(link_encryption_allowed_public_keys_count) =
                    unsafe { (*self.inner).getLinkEncryptionAllowedPublicKeysCount }
                {
                    let mut count: i32 = 0;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe {
                                link_encryption_allowed_public_keys_count(
                                    entity.as_ptr(),
                                    index,
                                    &mut count,
                                )
                            }
                        }
                        None => unsafe {
                            link_encryption_allowed_public_keys_count(std::ptr::null(), index, &mut count)
                        },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(count as u16),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_encryption_internal_source_ip function");
                }
            }

            /// Returns an allowed public key for the specified `entity` and link `index`.
            pub fn link_encryption_allowed_public_key(
                &self,
                entity: Option<&str>,
                link_index: i32,
                key_index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_encryption_allowed_public_key) =
                    unsafe { (*self.inner).getLinkEncryptionAllowedPublicKey }
                {
                    let mut buffer = vec![0; 1024];
                    let ptr = buffer.as_mut_ptr();
                    let mut size = buffer.len() as i32;

                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe {
                                link_encryption_allowed_public_key(
                                    entity.as_ptr(),
                                    link_index,
                                    key_index,
                                    ptr as *mut std::os::raw::c_char,
                                    &mut size as *mut i32,
                                )
                            }
                        }
                        None => unsafe {
                            link_encryption_allowed_public_key(
                                std::ptr::null(),
                                link_index,
                                key_index,
                                ptr as *mut std::os::raw::c_char,
                                &mut size as *mut i32,
                            )
                        },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => {
                            if size > 0 {
                                buffer.resize((size - 1) as usize, 0)
                            } else {
                                buffer.clear()
                            }
                            match $crate::utils::utf8_from_raw(&buffer) {
                                Ok(buffer) => Ok(buffer),
                                Err(_) => Err($crate::LinkError::OdenLinkErrorUnknown),
                            }
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_encryption_allowed_public_key function");
                }
            }

            /// Sets the internal encryption source `ip` for the given link `index`.
            /// `ip` MUST be a valid IPv4 address.
            pub fn set_link_encryption_internal_source_ip(
                &self,
                entity: Option<&str>,
                index: i32,
                ip: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_encryption_internal_source_ip) =
                    unsafe { (*self.inner).setLinkEncryptionInternalSourceIp }
                {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        key_path: *const std::os::raw::c_char| {
                            unsafe { set_link_encryption_internal_source_ip(entity, index, key_path) }
                        },
                        entity,
                        index,
                        ip,
                    )
                } else {
                    panic!("This version of Oden is too old to have the set_link_encryption_internal_source_ip function");
                }
            }

            /// Returns the internal encryption source `ip` for the given link `index`.
            pub fn link_encryption_internal_source_ip(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_encryption_internal_source_ip) =
                    unsafe { (*self.inner).getLinkEncryptionInternalSourceIp }
                {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| {
                            unsafe { link_encryption_internal_source_ip(entity, index, ptr, size) }
                        },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_encryption_internal_source_ip function");
                }
            }

            /// Set the internal encryption destination `ip` for the given link `index`.
            /// `ip` MUST be a valid IPv4 address.
            pub fn set_link_encryption_internal_destination_ip(
                &self,
                entity: Option<&str>,
                index: i32,
                ip: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_encryption_internal_destination_ip) =
                    unsafe { (*self.inner).setLinkEncryptionInternalDestinationIp }
                {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        key_path: *const std::os::raw::c_char| {
                            unsafe { set_link_encryption_internal_destination_ip(entity, index, key_path) }
                        },
                        entity,
                        index,
                        ip,
                    )
                } else {
                    panic!("This version of Oden is too old to have the set_link_encryption_internal_destination_ip function");
                }
            }

            /// Returns the internal encryption destination `ip` for the given link `index`.
            pub fn link_encryption_internal_destination_ip(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(link_encryption_internal_destination_ip) =
                    unsafe { (*self.inner).getLinkEncryptionInternalDestinationIp }
                {
                    $crate::scene_api::link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        ptr: *mut std::os::raw::c_char,
                        size: *mut i32| {
                            unsafe { link_encryption_internal_destination_ip(entity, index, ptr, size) }
                        },
                        entity,
                        index,
                    )
                } else {
                    panic!("This version of Oden is too old to have the link_encryption_internal_destination_ip function");
                }
            }

            /// Adds a new network link.
            pub fn add_link(&self, entity: Option<&str>) -> Result<(), $crate::LinkError> {
                if let Some(add_link) = unsafe { (*self.inner).addLink } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { add_link(entity.as_ptr()) }
                        }
                        None => unsafe { add_link(std::ptr::null()) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the add_link function");
                }
            }

            /// Removes the link with index `index`.
            pub fn remove_link(&self, entity: Option<&str>, index: i32) -> Result<(), $crate::LinkError> {
                if let Some(remove_link) = unsafe { (*self.inner).removeLink } {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { remove_link(entity.as_ptr(), index) }
                        }
                        None => unsafe { remove_link(std::ptr::null(), index) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the remove_link function");
                }
            }

            /// Enable or disable HMD controller rendering.
            pub fn show_hmd_controllers(&self, show: bool) {
                if let Some(show_hmd_controllers) = unsafe { (*self.inner).showHmdControllers } {
                    unsafe { show_hmd_controllers(show) }
                } else {
                    panic!("This version of Oden is too old to have the show_hmd_controllers function");
                }
            }

            /// Retrieve the current HMD transform
            /// Optionally includes the the "HmdPostTransform" (see "pushHmdPostTransform")
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let hmd_transform = api.hmd_transform(true).unwrap();
            /// # }
            /// ```
            pub fn hmd_transform(&self, include_post_transformation: bool) -> Option<$crate::math::Matrix4> {
                if let Some(hmd_transform) = unsafe { ( *self.inner).getHmdTransform } {
                    let mut mat = $crate::math::Matrix4::default();
                    if unsafe { hmd_transform(&mut mat as *mut _, include_post_transformation) } {
                        Some(mat)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the hmd_transform function");
                }
            }

            /// Sets the center pixel position in the focus region for `entity` if it has an active focus region.
            ///
            /// Only valid for `Clone Stream` inputs that clone from a `Remote Streamer` entity.
            pub fn set_focus_region_center_pixel_position(&self, entity: &str, x: i32, y: i32) -> bool {
                if let Some(set_focus_region_center_pixel_position) =
                    unsafe { (*self.inner).setFocusRegionCenterPixelPosition }
                {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    unsafe { set_focus_region_center_pixel_position(entity.as_ptr(), x, y) }
                } else {
                    panic!("This version of Oden is too old to have the set_focus_region_center_pixel_position function");
                }
            }

            /// Returns the size in pixels of the video for `entity` and `stream`, or [`None`] if
            /// `entity` and `stream` is invalid.
            pub fn entity_video_size(&self, entity: &str, stream: i32) -> Option<$crate::math::Vec2i> {
                if let Some(entity_video_size) = unsafe { (*self.inner).getEntityVideoSize } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    let mut size = $crate::math::Vec2i::default();

                    if unsafe { entity_video_size(entity.as_ptr(), stream, &mut size as *mut _) } {
                        Some(size)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the entity_video_size function");
                }
            }

            /// Sets the regulator mode. The different regulator modes are Low, Medium and High.
            /// If Auto is sent, the regulator will determine the appropriate mode by itself depending
            /// on the current bandwidth.
            pub fn set_regulator_mode(&self, mode: $crate::RegulatorMode) {
                if let Some(set_regulator_mode) = unsafe { (*self.inner).setRegulatorMode } {
                    unsafe { set_regulator_mode(mode) }
                } else {
                    panic!("This version of Oden is too old to have the set_regulator_mode function");
                }
            }

            /// Returns the active [`RegulatorMode`](crate::RegulatorMode).
            pub fn regulator_mode(&self) -> $crate::RegulatorMode {
                if let Some(regulator_mode) = unsafe { (*self.inner).getRegulatorMode } {
                    unsafe { regulator_mode() }
                } else {
                    panic!("This version of Oden is too old to have the regulator_mode function");
                }
            }

            /// Sets sidebar GUI visibility.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.set_gui_visible(false);
            /// # }
            /// ```
            pub fn set_gui_visible(&self, enable: bool) {
                if let Some(set_gui_visible) = unsafe{(*self.inner).setGuiVisible} {
                    unsafe {
                        set_gui_visible(enable);
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_gui_visible function");
                }
             }

            /// Returns pressed state of `key`.
            ///
            /// Optionally check if any combination of modifier keys(`Ctrl`, `Shift`, `Alt`, and `Super`) are pressed.
            /// (`Super` is the Windows key).
            /// This function is intended to give state of keys pressed.
            /// It is not intended for text input!
            pub fn is_key_down(&self, key: i32, modifiers: Option<$crate::KeyModifiers>) -> bool {
                if let Some(is_key_down) = unsafe { (*self.inner).isKeyDown } {
                    unsafe {
                        let mut mod_mask: i32;
                        let mut c_mod: *mut i32 = std::ptr::null_mut();
                        if let Some(modifiers) = modifiers {
                            mod_mask = modifiers.mask();
                            c_mod = &mut mod_mask;
                        }
                        is_key_down(key, c_mod)
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_key_down function");
                }
            }

            /// Returns `true` if key has been pressed since last frame.
            ///
            /// This is a good way to catch things that are supposed to fire once (e.g. ctrl+s for save).
            /// This function is intended to give state of keys pressed.
            /// It is not intended for text input!
            pub fn is_key_pressed_since_last_swap(&self, key: i32, modifiers: Option<$crate::KeyModifiers>) -> bool {
                if let Some(is_key_pressed_since_last_swap) = unsafe { (*self.inner).isKeyPressedSinceLastSwap } {
                    unsafe {
                        let mut mod_mask: i32;
                        let mut c_mod: *mut i32 = std::ptr::null_mut();
                        if let Some(modifiers) = modifiers {
                            mod_mask = modifiers.mask();
                            c_mod = &mut mod_mask;
                        }
                        is_key_pressed_since_last_swap(key, c_mod)
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the is_key_pressed_since_last_swap function"
                    );
                }
            }

            /// Returns a list of all keys that have been pressed since last frame (swap).
            pub fn keys_pressed_since_last_swap(&self) -> Vec<(i32, $crate::KeyModifiers)> {
                if let Some(keys_pressed_since_last_swap) =
                    unsafe { (*self.inner).getKeysPressedSinceLastSwap }
                {
                    let mut count: i32 = 0;
                    if unsafe {
                        !keys_pressed_since_last_swap(
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            &mut count as *mut _,
                        )
                    } {
                        return Vec::new();
                    }

                    let mut keys = Vec::new();
                    keys.resize_with(count as usize, Default::default);

                    let mut modifiers = Vec::new();
                    modifiers.resize_with(count as usize, Default::default);

                    if unsafe {
                        !keys_pressed_since_last_swap(
                            keys.as_mut_ptr(),
                            modifiers.as_mut_ptr(),
                            &mut count as *mut _,
                        )
                    } {
                        return Vec::new();
                    }

                    keys.iter()
                        .copied()
                        .zip(modifiers.iter().copied())
                        .map(|(key, modifiers)| (key, $crate::KeyModifiers::from_mask(modifiers as u32)))
                        .collect()
                } else {
                    panic!(
                        "This version of Oden is too old to have the keys_pressed_since_last_swap function"
                    );
                }
            }

            /// Returns the window events that were emitted since last frame.
            pub fn window_events(&self) -> Vec<$crate::WindowEvent> {
                if let Some(window_events) =
                    unsafe { (*self.inner).getWindowEvents }
                {
                    let mut count: i32 = 0;
                    if unsafe {
                        !window_events(
                            std::ptr::null_mut(),
                            &mut count as *mut _,
                        )
                    } {
                        return Vec::new();
                    }

                    let mut events = Vec::new();
                    events.resize_with(count as usize, Default::default);

                    if unsafe {
                        !window_events(
                            events.as_mut_ptr(),
                            &mut count as *mut _,
                        )
                    } {
                        return Vec::new();
                    }

                    events.iter().map(|e| e.into()).collect()
                } else {
                    log::error!(
                        "This version of Oden is too old to have the window_events function"
                    );
                    Vec::new()
                }
            }

            /// Returns `true` if a raw recording is currently recording.
            ///
            /// To see if a raw recording is playing see [`playback_is_playing`](scene_api_impl::playback_is_playing).
            pub fn raw_record_is_running(&self) -> bool {
                if let Some(raw_record_is_running) = unsafe { (*self.inner).rawRecordIsRunning } {
                    unsafe { raw_record_is_running() }
                } else {
                    panic!("This version of Oden is too old to have the raw_record_is_running function");
                }
            }

            /// Starts playback of the raw recording.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.playback_start();
            /// # }
            /// ```
            pub fn playback_start(&self) {
                if let Some(playback_start) = unsafe {(*self.inner).playbackStart} {
                    unsafe {
                        playback_start();
                    }
                } else {
                    panic!("This version of Oden is too old to have the playback_start function");
                }
            }

            /// Pauses playback of the raw recording.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.playback_pause();
            /// # }
            /// ```
            pub fn playback_pause(&self) {
                if let Some(playback_pause) = unsafe {(*self.inner).playbackPause} {
                    unsafe {
                        playback_pause();
                    }
                } else {
                    panic!("This version of Oden is too old to have the playback_Pause function");
                }
            }

            /// Sets the raw recording playback to the desired timestamp `time`, in seconds.
            /// If `time` is outside the range given by [`playback_times`](scene_api_impl::playback_times)
            /// the time will be set to the maximum or minimum time the playback can be at,
            /// whichever is closest.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if api.playback_times().unwrap().current > 10.0 {
            ///     api.set_playback_time(0.0); // set playback time to 0.0 seconds
            /// }
            /// # }
            /// ```
            pub fn set_playback_time(&self, time: f64) {
                if let Some(set_playback_time) = unsafe {(*self.inner).playbackSetTime} {
                    unsafe {
                        set_playback_time(time);
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_playback_time function");
                }
            }

            /// Moves the current raw recording playback time forward with the amount of `time` given (in seconds).
            /// A negative value will move the time backwards.
            /// The current time will be clamped between the maximum and minimum time given by
            /// [`playback_times`](scene_api_impl::playback_times).
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if api.playback_times().unwrap().current > 10.0 {
            ///     api.playback_advance_time(10.0); // advance the timer by 10 seconds
            /// }
            /// # }
            /// ```
            pub fn playback_advance_time(&self, time: f64) {
                if let Some(playback_advance_time) = unsafe {(*self.inner).playbackAdvanceTime} {
                    unsafe {
                        playback_advance_time(time);
                    }
                } else {
                    panic!("This version of Oden is too old to have the playback_advance_time function");
                }
            }

            /// Sets if the playback should loop or not.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// api.set_playback_loop(true);
            /// # }
            /// ```
            pub fn set_playback_loop(&self, enable: bool) {
                if let Some(set_playback_loop) = unsafe {(*self.inner).playbackSetLoop} {
                    unsafe {
                        set_playback_loop(enable);
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_playback_loop function");
                }
            }

            /// Returns `true` if a raw recording project is playing.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if api.playback_is_playing() {
            ///     // Do something
            /// }
            /// # }
            /// ```
            pub fn playback_is_playing(&self) -> bool {
                if let Some(playback_is_playing) = unsafe { (*self.inner).playbackIsPlaying } {
                    unsafe { playback_is_playing() }
                } else {
                    panic!("This version of Oden is too old to have the playback_is_playing function");
                }
            }

            /// Returns the raw recording playback range and current time, or [`None`] if not currently
            /// playing back a raw recording.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let times = api.playback_times().unwrap();
            /// if times.current > 10.0 {
            ///     // do something
            /// }
            /// # }
            /// ```
            pub fn playback_times(&self) -> Option<$crate::PlaybackTimes> {
                unsafe {
                    if let Some(playback_get_times) = (*self.inner).playbackGetTimes {
                        let mut times = $crate::PlaybackTimes::default();
                        if playback_get_times(&mut times.min, &mut times.max, &mut times.current) {
                            if times.min > times.max {
                                // Not playing back.
                                return None;
                            }
                            return Some(times);
                        } else {
                            return None;
                        }
                    } else {
                        panic!("This version of Oden is too old to have the playback_times function");
                    }
                }
            }

            /// Returns the maximum supported message size.
            ///
            /// (This may change depending on the network backend, so it is an error to assume it is of any
            /// one specific value).
            pub fn com_channel_max_message_size(&self) -> i32 {
                if let Some(com_channel_max_message_size) = unsafe { (*self.inner).comChannelMaxMessageSize } {
                    unsafe { com_channel_max_message_size() }
                } else {
                    panic!("This version of Oden is too old to have the com_channel_max_message_size function");
                }
            }

            /// Sends a com channel message.
            pub fn com_channel_send_message_raw(&self, message_id: &str, data: &[u8])
            {
                if let Some(com_channel_send_message) = unsafe { (*self.inner).comChannelSendMessage } {
                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();

                    unsafe {
                        com_channel_send_message(
                            message_id.as_ptr(),
                            data.as_ptr() as *mut u8,
                            data.len() as i32,
                        )
                    };
                } else {
                    panic!("This version of Oden is too old to have the com_channel_send_message_raw function");
                }
            }

            /// Returns the number of com channel messages that are available in this frame.
            pub fn com_channel_message_count(&self, message_id: &str) -> i32 {
                if let Some(com_channel_message_count) = unsafe { (*self.inner).comChannelGetMessageCount }
                {
                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();
                    unsafe { com_channel_message_count(message_id.as_ptr()) }
                } else {
                    panic!(
                        "This version of Oden is too old to have the com_channel_message_count function"
                    );
                }
            }

            /// Returns a variable sized com channel message with the given `message_id` at `index` in the
            /// message queue.
            pub fn com_channel_var_size_message(&self, message_id: &str, index: i32) -> Option<Vec<u8>> {
                if let Some(com_channel_var_size_msg) = unsafe { (*self.inner).comChannelGetVarSizeMsg } {
                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();

                    let mut size: i32 = 0;

                    unsafe {
                        if !com_channel_var_size_msg(
                            message_id.as_ptr(),
                            index,
                            std::ptr::null_mut(),
                            &mut size as *mut _,
                        ) {
                            return None;
                        }
                    }

                    let mut buf: Vec<u8> = vec![0; size as usize];

                    if unsafe {
                        com_channel_var_size_msg(
                            message_id.as_ptr(),
                            index,
                            buf.as_mut_slice().as_mut_ptr(),
                            &mut size as *mut _,
                        )
                    } {
                        Some(buf)
                    } else {
                        None
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the com_channel_var_size_message function"
                    );
                }
            }

            /// Returns the latest variable sized com channel message with the given `message_id`.
            pub fn com_channel_last_var_size_msg(&self, message_id: &str) -> Option<Vec<u8>> {
                if let Some(com_channel_last_var_size_msg) =
                    unsafe { (*self.inner).comChannelGetLastVarSizeMsg }
                {
                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();

                    let mut size: i32 = 0;

                    unsafe {
                        if !com_channel_last_var_size_msg(
                            message_id.as_ptr(),
                            std::ptr::null_mut(),
                            &mut size as *mut _,
                        ) {
                            return None;
                        }
                    }

                    let mut buf: Vec<u8> = vec![0; size as usize];

                    if unsafe {
                        com_channel_last_var_size_msg(
                            message_id.as_ptr(),
                            buf.as_mut_slice().as_mut_ptr(),
                            &mut size as *mut _,
                        )
                    } {
                        Some(buf)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the com_channel_last_var_size_msg function");
                }
            }

            /// Returns the calibration parameters for a given `entity` and `stream`.
            pub fn camera_calibration(&self, entity: &str, stream: i32) -> Option<$crate::CameraCalibration> {
                if let Some(camera_calibration) = unsafe { (*self.inner).getCameraCalibration } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    let mut origin = $crate::CameraCalibration::default();

                    if unsafe { camera_calibration(entity.as_ptr(), stream, &mut origin as *mut _) } {
                        Some(origin)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_calibration function");
                }
            }

            /// Returns the ID of the scene that contains the plugin,
            /// or [`None`] if the plugin is a global plugin.
            pub fn scene_containing_plugin_entity(&self) -> Option<i32> {
                if let Some(scene_containing_plugin_entity) =
                    unsafe { (*self.inner).getSceneContainingPluginEntity }
                {
                    let mut scene_id: i32 = -1;

                    if unsafe { scene_containing_plugin_entity(&mut scene_id as *mut _) } {
                        Some(scene_id)
                    } else {
                        None
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the scene_containing_plugin_entity function"
                    );
                }
            }

            /// Saves the current project.
            pub fn save_project(&self) {
                unsafe {
                    if let Some(save_project) = (*self.inner).saveProject {
                        save_project();
                    } else {
                        panic!("This version of Oden is too old to have the save_project function");
                    }
                }
            }

            /// Returns `true` if the project has unsaved changes.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let save_status = api.project_has_unsaved_changes();
            /// # }
            /// ```
            pub fn project_has_unsaved_changes(&self) -> bool {
                if let Some(project_has_unsaved_changes) = unsafe{ (*self.inner).projectHasUnsavedChanges} {
                    unsafe {
                        project_has_unsaved_changes()
                    }
                } else {
                    panic!("This version of Oden is too old to have the project_has_unsaved_changes function");

                }
            }

            pub fn set_force_feedback_angle(&self, angle: f32) {
                unsafe {
                    if let Some(set_force_feedback_angle) = (*self.inner).setForceFeedbackAngle {
                        set_force_feedback_angle(angle);
                    } else {
                        panic!("This version of Oden is too old to have the set_force_feedback_angle function");
                    }
                }
            }

            // Returns `true` if the Streamer is running.
            pub fn is_streamer_running(&self) -> bool {
                unsafe {
                    if let Some(is_streamer_running) = (*self.inner).isStreamerRunning {
                        is_streamer_running()
                    } else {
                        panic!("This version of Oden is too old to have the is_streamer_running function");
                    }
                }
            }

            /// Return the time the last frame was received, the time is before any decoding or other
            /// processing are done.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let frame_times = api.camera_last_received_frame_time("EntityName", 1);
            /// # }
            /// ```
            pub fn camera_last_received_frame_time(&self, entity: &str, stream: i32) -> Option<u64> {
                unsafe {
                    if let Some(get_camera_last_frame_time) = (*self.inner).getCameraLastReceivedFrameTime {

                        let mut timestamp = 0;
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                        if get_camera_last_frame_time(entity.as_ptr(), stream, &mut timestamp as *mut _)  {
                            Some(timestamp)
                        } else {
                            None
                        }
                    } else {
                        panic!("This version of Oden is too old to have the camera_last_received_frame_time function");
                    }
                }
            }

            /// Start receiving audio
            ///
            /// `entity` is an [`Option`].
            /// If `entity` is a [`Some`] value, the function will start receiving audio
            /// from the Remote Streamer entity with the name `entity`.
            /// If `entity` is [`None`] and the plugin is running in Oden Streamer, the function
            /// will start receiving audio from the Streamer Output audio streaming.
            ///
            /// Example
            /// ```no_run
            /// # use oden_plugin_rs::log;
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi + oden_plugin_rs::SettingsApi)) {
            ///     if api.application_type() == oden_plugin_rs::ApplicationType::Streamer {
            ///         if api.button("Start Sending Audio") {
            ///             api.set_audio_receiving_enable(None, true).ok();
            ///         }
            ///     }
            ///     if api.application_type() == oden_plugin_rs::ApplicationType::Player {
            ///         if api.button("Start Sending Audio") {
            ///             api.set_audio_receiving_enable(Some("hej"), true).ok();
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn set_audio_receiving_enable(&self, entity: Option<&str>, enable: bool) -> Result<(), String> {
                unsafe {
                    if let Some(set_audio_receiving_enable) = (*self.inner).setAudioReceivingEnabled {
                        let res = match entity {
                            Some(entity) => {
                                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                                set_audio_receiving_enable(entity.as_ptr(), enable)
                            }
                            None => set_audio_receiving_enable(std::ptr::null(), enable)
                        };

                        if res {
                            Ok(())
                        } else {
                            Err("Failed to start receiving audio".to_string())
                        }
                    } else {
                        panic!("This version of Oden is too old to have the set_audio_receiving_enable function");
                    }
                }
            }

            /// Start sending audio
            ///
            /// `entity` is an [`Option`].
            /// If `entity` is a [`Some`] value, the function will start sending audio
            /// from the Remote Streamer entity with the name `entity`.
            /// If `entity` is [`None`] and the plugin is running in Oden Streamer, the function
            /// will start sending audio from the Streamer Output audio streaming.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi + oden_plugin_rs::SettingsApi)) {
            ///     if api.application_type() == oden_plugin_rs::ApplicationType::Streamer {
            ///         if api.button("Start Sending Audio") {
            ///             api.set_audio_sending_enable(None, true).ok();
            ///         }
            ///     }
            ///     if api.application_type() == oden_plugin_rs::ApplicationType::Player {
            ///         if api.button("Start Sending Audio") {
            ///             api.set_audio_sending_enable(Some("hej"), true).ok();
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn set_audio_sending_enable(&self, entity: Option<&str>, enable: bool) -> Result<(), String>{
                unsafe {
                    if let Some(set_audio_sending_enable) = (*self.inner).setAudioSendingEnabled {
                        let res = match entity {
                            Some(entity) => {
                                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                                set_audio_sending_enable(entity.as_ptr(), enable)
                            }
                            None => set_audio_sending_enable(std::ptr::null(), enable)
                        };

                        if res {
                            Ok(())
                        } else {
                            Err("Failed to start sending audio".to_string())
                        }
                    } else {
                        panic!("This version of Oden is too old to have the set_audio_sending_enable function");
                    }
                }
            }

            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Load Scene") {
            ///         api.load_scene("scene.vproj");
            ///     }
            /// # }
            /// ```
            pub fn load_scene(&self, path: &str) -> Option<std::ops::RangeInclusive<i32>> {
                unsafe {
                    if let Some(load_scene) = (*self.inner).loadScene {
                        let path = std::ffi::CString::new(path.trim_end_matches('\0')).unwrap();
                        let mut range_out = $crate::math::Vec2i::default();
                        if load_scene(path.as_ptr(), &mut range_out as *mut _) {
                            return Some(range_out.x()..=range_out.y());
                        } else {
                            return None;
                        }
                    } else {
                        panic!("This version of Oden is too old to have the load_scene function");
                    }
                }
            }

            /// Remove Scene
            /// Removes the scene with the supplied ID. The ID of the scene is not the same
            /// as the one shown in the Scenes tab. The available ID's get be requested through the
            /// all_scene_ids function.
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Remove Scene") {
            ///         let scene_id = 1;
            ///         api.remove_scene(scene_id);
            ///     }
            /// # }
            /// ```
            pub fn remove_scene(&self, scene_id: i32) {
                unsafe {
                    if let Some(remove_scene) = (*self.inner).removeScene {
                        remove_scene(scene_id);
                    } else {
                        panic!("This version of Oden is too old to have the remove_scene function");
                    }
                }
            }

            /// Returns bytes transferred since start for the specified link.
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get link stats") {
            ///         let _ = api.link_stats_bytes_transferred(None, 0);
            ///     }
            /// # }
            /// ```
            pub fn link_stats_bytes_transferred(
                &self,
                remote_streamer_entity: Option<&str>,
                index: i32
            ) -> Result<$crate::plugin_h::OdenLinkStatsTransferredBytes, $crate::LinkError> {
                if let Some(get_link_stats_bytes_transferred) = unsafe { (*self.inner).getLinkStatistics } {

                    let mut stats = $crate::plugin_h::OdenLinkStatsTransferredBytes {
                        type_: $crate::plugin_h::OdenLinkStatsStructureType::OdenLinkStatsStructureTypeTransferredBytes,
                        next: std::ptr::null_mut(),
                        bytesReceived: 0,
                        bytesSent: 0,
                    };

                    let res = match remote_streamer_entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { get_link_stats_bytes_transferred(entity.as_ptr(), index, &mut stats as *mut _ as *mut std::os::raw::c_void) }
                        }
                        None => unsafe { get_link_stats_bytes_transferred(std::ptr::null(), index, &mut stats as *mut _ as *mut std::os::raw::c_void) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(stats),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_stats_bytes_transferred function");
                }
            }

            /// Get the output alignment position for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get output alignment position") {
            ///         if let Ok(pos) = api.output_alignment_position("Entity Name or UUID") {
            ///             log::info!("position: {:?}", pos);
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn output_alignment_position(&self, entity: &str) -> Result<$crate::math::Vec2i, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamOutputAlignmentPosition {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentPosition,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        position: $crate::math::Vec2i::xy(0, 0),
                    };

                    let res = unsafe { get_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(pos.position),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the output_alignment_position function");
                }
            }

            /// Sets the output alignment position for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Set output alignment position") {
            ///         let _ = api.set_output_alignment_position(
            ///             "Entity Name or UUID",
            ///             oden_plugin_rs::math::Vec2i::xy(10, 10),
            ///         );
            ///     }
            /// # }
            /// ```
            pub fn set_output_alignment_position(&self, entity: &str, position: $crate::math::Vec2i) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamOutputAlignmentPosition {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentPosition,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        position,
                    };

                    let res = unsafe { set_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_output_alignment_position function");
                }
            }

            /// Get the output alignment scale for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get output alignment scale") {
            ///         if let Ok(pos) = api.output_alignment_scale("Entity Name or UUID") {
            ///             log::info!("scale: {:?}", pos);
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn output_alignment_scale(&self, entity: &str) -> Result<f32, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut scale = $crate::plugin_h::OdenSceneParamOutputAlignmentScale {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentScale,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        scale: 0.0,
                    };

                    let res = unsafe { get_scene_param(&mut scale as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(scale.scale),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the output_alignment_scale function");
                }
            }

            /// Sets the output alignment scale for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Set output alignment scale") {
            ///         let _ = api.set_output_alignment_scale("Entity Name or UUID", 0.5);
            ///     }
            /// # }
            /// ```
            pub fn set_output_alignment_scale(&self, entity: &str, scale: f32) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamOutputAlignmentScale {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentScale,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        scale,
                    };

                    let res = unsafe { set_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_output_alignment_scale function");
                }
            }

            /// Get the output alignment rotation for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get output alignment rotation") {
            ///         if let Ok(pos) = api.output_alignment_rotation("Entity Name or UUID") {
            ///             log::info!("rotation: {:?}", pos);
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn output_alignment_rotation(&self, entity: &str) -> Result<$crate::plugin_h::OdenRotation, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut rot = $crate::plugin_h::OdenSceneParamOutputAlignmentRotation {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentRotation,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        rotation: $crate::plugin_h::OdenRotation::OdenRotation0,
                    };

                    let res = unsafe { get_scene_param(&mut rot as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(rot.rotation),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the output_alignment_rotation function");
                }
            }

            /// Sets the output alignment rotation for a specified `entity`.
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video placed under a Output Alignment entity.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Set output alignment rotation") {
            ///         let _ = api.set_output_alignment_rotation("Entity Name or UUID", oden_plugin_rs::plugin_h::OdenRotation::OdenRotation90);
            ///     }
            /// # }
            /// ```
            pub fn set_output_alignment_rotation(
                &self,
                entity: &str,
                rotation: $crate::plugin_h::OdenRotation,
            ) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamOutputAlignmentRotation {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentRotation,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        rotation,
                    };

                    let res = unsafe { set_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_output_alignment_rotation function");
                }
            }

            /// Get streamer output resolution
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get streamer resolution") {
            ///         if let Ok(res) = api.streamer_resolution() {
            ///             log::info!("resolution: {:?}", res);
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn streamer_resolution(&self) -> Result<$crate::math::Vec2i, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut resolution = $crate::plugin_h::OdenSceneParamStreamerResolution {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeStreamerResolution,
                        next: std::ptr::null_mut(),
                        resolution: $crate::math::Vec2i::xy(0, 0),
                    };

                    let res = unsafe { get_scene_param(&mut resolution as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(resolution.resolution),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the streamer_resolution function");
                }
            }

            /// Set streamer output resolution
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Set streamer resolution") {
            ///         let _ = api.set_streamer_resolution(oden_plugin_rs::math::Vec2i::xy(1920, 1080));
            ///     }
            /// # }
            /// ```
            pub fn set_streamer_resolution(&self, resolution: $crate::math::Vec2i) -> Result<(), $crate::SceneParamError>
            {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut pos = $crate::plugin_h::OdenSceneParamStreamerResolution {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeStreamerResolution,
                        next: std::ptr::null_mut(),
                        resolution,
                    };

                    let res = unsafe { set_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_streamer_resolution function");
                }
            }

            /// Set the streamer to sync to the supplied video input.
            ///
            /// `entity_or_stream` can be:
            /// - unique name or UUID of a 2D Video
            /// - stream UUID (in which case the stream i32 is not required)
            /// - empty string to signal that any existing sync is to be removed (in which case the stream i32 is not required)
            ///
            /// `stream` is the number of the video input:
            /// - for 2D Video the number 1 should be supplied
            /// - for Stitched Video the stream number corresponds to the input number
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Set camera for sync") {
            ///         let _successful = api.set_sync_camera("Entity Name or UUID", 1);
            ///     }
            /// # }
            /// ```
            pub fn set_sync_camera(&self, entity_or_stream: &str, stream: i32) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let c_entity_or_stream = std::ffi::CString::new(entity_or_stream.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamSyncCamera {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeSyncCamera,
                        next: std::ptr::null_mut(),
                        entityOrStream: if entity_or_stream.is_empty() { std::ptr::null_mut() } else { c_entity_or_stream.as_ptr() },
                        stream,
                    };

                    let res = unsafe { set_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_sync_camera function");
                }
            }

            /// Get the last decoder status for the last 64 frames
            ///
            /// The `entity` MUST be the unique name or UUID of a 2D Video.
            /// The `stream` is the number of the video input, for 2D Video the number 1 should be supplied, for Stitched Video the stream number corresponds to the input number.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(stats) = api.decoder_stats("Entity Name or UUID", 1){
            ///         // Do something
            ///     }
            /// # }
            /// ```
            pub fn decoder_stats(&self, entity: &str, stream: i32) -> Result<[$crate::DecodedFrame; 64usize], $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut decoder_stats = $crate::plugin_h::OdenSceneParamDecoderStats {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeDecoderStats,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        stream,
                        decodedFrames: [$crate::DecodedFrame::default(); 64usize]
                    };

                    let res = unsafe { get_scene_param(&mut decoder_stats as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(decoder_stats.decodedFrames),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_sync_camera function");
                }
            }

            /// Get if the mouse is currently over a GUI element.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(stats) = api.is_over_gui() {
            ///         if stats {
            ///             log::info!("Gui Is Hovered");
            ///         } else {
            ///             log::info!("Gui Is Not Hovered");
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn is_over_gui(&self) -> Result<bool, $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                let mut gui_hovered = $crate::plugin_h::OdenSceneParamGuiHovered {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeGuiHovered,
                        next: std::ptr::null_mut(),
                        isHovered: false,
                    };

                    let res = unsafe { get_scene_param(&mut gui_hovered as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(gui_hovered.isHovered),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_over_gui function");
                }
            }

            /// Closes the current project
            ///
            /// Example
            /// ```no_run
            /// # fn close_project(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.close_project();
            /// # }
            /// ```
            pub fn close_project(&self) {
                if let Some(close_project) = unsafe { (*self.inner).closeProject } {
                    unsafe { close_project() }
                } else {
                    panic!("This version of Oden is too old to have the close_project function");
                }
            }

            /// Request that the software will take a print screen.
            /// The image data will only contain the data from the software and not from the whole screen.
            ///
            /// The data will be available after 2 frames, the print screen will be for the frame after it has been requested.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.request_print_screen()
            /// # }
            /// ```
            pub fn request_print_screen(&self) {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                let mut request = $crate::plugin_h::OdenSceneParamPrintScreen {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePrintScreen,
                        next: std::ptr::null_mut(),
                        width: 0,
                        height: 0,
                        size: 0,
                        data: std::ptr::null_mut(),
                    };

                    unsafe { set_scene_param(&mut request as *mut _ as *mut std::os::raw::c_void) };

                } else {
                    panic!("This version of Oden is too old to have the is_over_gui function");
                }
            }

            /// Retrieves the frame data after a request_print_screen() has been issued. Return None if data is not yet ready
            /// or if no request_print_screen() has been called before. The image data will only contain the data from the
            /// software and not from the whole screen. The data is packed RGB and is only available 2 frames after
            /// request_print_screen() has been issued.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(img) = api.print_screen() {
            ///         // Do something with the image
            ///     }
            /// # }
            /// ```
            #[allow(
                clippy::needless_lifetimes
            )]
            pub fn print_screen<'a>(&'a self) -> Result<$crate::scene_api::PrintScreen<'a>, $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut data_out = $crate::plugin_h::OdenSceneParamPrintScreen {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePrintScreen,
                            next: std::ptr::null_mut(),
                            width: 0,
                            height: 0,
                            size: 0,
                            data: std::ptr::null_mut(),
                        };

                    let res = unsafe { get_scene_param(&mut data_out as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            if data_out.size > 0 && !data_out.data.is_null() {
                                let data = unsafe{ $crate::scene_api::PrintScreen{
                                    data: std::slice::from_raw_parts(data_out.data, data_out.size as usize),
                                    width:  data_out.width,
                                    height:  data_out.height
                                }};

                                Ok(data)
                            } else {
                                Err($crate::SceneParamError::OdenSceneParamErrorError)
                            }
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_over_gui function");
                }
            }

            /// Retreives which window mode that is currently active
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(mode) = api.window_mode() {
            ///         match mode {
            ///             oden_plugin_rs::WindowMode::OdenWindowModeWindowed => log::info!("Windowed"),
            ///             oden_plugin_rs::WindowMode::OdenWindowModeWindowedFullscreen => log::info!("Windowed Fullscreen"),
            ///             oden_plugin_rs::WindowMode::OdenWindowModeSpanningFullscreen => log::info!("Spanning Fullscreen"),
            ///             oden_plugin_rs::WindowMode::OdenWindowModeFullscreen => log::info!("Fullscreen"),
            ///             _ => log::info!("Error"),
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn window_mode(&self) -> Result<$crate::WindowMode, $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut window_mode = $crate::plugin_h::OdenSceneParamWindowMode {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeWindowMode,
                            next: std::ptr::null_mut(),
                            mode: $crate::WindowMode::OdenWindowModeFullscreen,
                        };

                    let res = unsafe { get_scene_param(&mut window_mode as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(window_mode.mode)
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_over_gui function");
                }
            }

            /// Returns the index for the given link `uuid`.
            ///
            /// Use [`link_uuid`](scene_api_impl::link_uuid) to get the uuid
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     use oden_plugin_rs::OdenUuid;
            ///
            ///     let entity = None;
            ///     let uuid = OdenUuid::default();
            ///     if let Ok(index) = api.link_index(entity, &uuid) {
            ///         log::info!("Got link index: {index}");
            ///     }
            /// # }
            /// ```
            pub fn link_index(
                &self,
                entity: Option<&str>,
                uuid: &$crate::plugin_h::OdenUuid,
            ) -> Result<i32, $crate::LinkError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity_str = if let Some(entity) = entity {
                        std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap()
                    } else {
                        std::ffi::CString::new("").unwrap()
                    };

                    let entity_ptr = if entity.is_some() {
                        entity_str.as_ptr()
                    } else {
                        std::ptr::null()
                    };

                    let mut param = $crate::plugin_h::OdenSceneParamLinkIndex {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLinkIndex,
                        next: std::ptr::null_mut(),
                        entityId: entity_ptr,
                        uuid: uuid as *const _,
                        index: 0,
                        result: $crate::LinkError::OdenLinkErrorUnknown,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(param.index)
                        },
                        _ => Err(param.result),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_index function");
                }
            }

            /// Returns the UUID for the given link `index`.
            ///
            /// The UUID can be used to keep track of the link when adding/removing links.
            /// Use [`link_index`](scene_api_impl::link_index) to get the index
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     if let Ok(uuid) = api.link_uuid(entity, index) {
            ///         log::info!("Got link UUID: {uuid:?}");
            ///     }
            /// # }
            /// ```
            pub fn link_uuid(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<$crate::plugin_h::OdenUuid, $crate::LinkError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity_str = if let Some(entity) = entity {
                        std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap()
                    } else {
                        std::ffi::CString::new("").unwrap()
                    };

                    let entity_ptr = if entity.is_some() {
                        entity_str.as_ptr()
                    } else {
                        std::ptr::null()
                    };

                    let mut param = $crate::plugin_h::OdenSceneParamLinkUuid {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLinkUuid,
                        next: std::ptr::null_mut(),
                        entityId: entity_ptr,
                        index,
                        uuid: $crate::plugin_h::OdenUuid::default(),
                        result: $crate::LinkError::OdenLinkErrorUnknown,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(param.uuid)
                        },
                        _ => Err(param.result),
                    }

                } else {
                    panic!("This version of Oden is too old to have the link_index function");
                }
            }

            /// Returns the interface statistics for a certain link. If the link has not gotten any traffic
            /// in over a second it will return the error OdenLinkErrorLinkHasNotGottenTraffic
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     if api.button("Get link stats") {
            ///         let _ = api.link_stats_interface(Some("Remote Streamer Name"), 0);
            ///     }
            /// # }
            /// ```
            pub fn link_stats_interface(
                &self,
                remote_streamer_entity: Option<&str>,
                index: i32
            ) -> Result<$crate::plugin_h::OdenLinkStatsInterfaceStatistics, $crate::LinkError>{
                if let Some(get_link_statistics) = unsafe { (*self.inner).getLinkStatistics } {
                    let mut stats = $crate::plugin_h::OdenLinkStatsInterfaceStatistics {
                        type_: $crate::plugin_h::OdenLinkStatsStructureType::OdenLinkStatsStructureTypeInterfaceStatistics,
                        next: std::ptr::null_mut(),
                        statistics: $crate::plugin_h::OdenReceiveInterfaceStatistics::default(),
                        timeSinceLastReceivedPacketNs: 0,
                    };

                    let res = match remote_streamer_entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { get_link_statistics(entity.as_ptr(), index, &mut stats as *mut _ as *mut std::os::raw::c_void) }
                        }
                        None => unsafe { get_link_statistics(std::ptr::null(), index, &mut stats as *mut _ as *mut std::os::raw::c_void) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(stats),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_stats_interface function");
                }
            }

            /// Set if the bandwidth regulator should be enabled or disabled.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if api.set_bandwidth_regulator_state(true).is_ok() {
            ///        log::info!("Bandwidth regulator is now enabled");
            ///     }
            /// # }
            /// ```
            pub fn set_bandwidth_regulator_state(&self, enabled: bool) -> Result<(), $crate::SceneParamError>  {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut bandwidth_regulator = $crate::plugin_h::OdenSceneParamBandwidthControlState {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeBandwidthControlState,
                            next: std::ptr::null_mut(),
                            enabled,
                        };

                    let res = unsafe { set_scene_param(&mut bandwidth_regulator as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_bandwidth_regulator_state function");
                }
            }

            /// Get the bandwidth regulator state
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(state) = api.bandwidth_regulator_state() {
            ///        log::info!("Bandwidth regulator is {state:?}");
            ///     }
            /// # }
            /// ```
            pub fn bandwidth_regulator_state(&self) -> Result<bool, $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut bandwidth_regulator = $crate::plugin_h::OdenSceneParamBandwidthControlState {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeBandwidthControlState,
                            next: std::ptr::null_mut(),
                            enabled: false,
                        };

                    let res = unsafe { get_scene_param(&mut bandwidth_regulator as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(bandwidth_regulator.enabled)
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the bandwidth_regulator_state function");
                }
            }

            /// Get a Scene as a string based on its `scene_id`.
            ///
            /// `full_scene_config` controls if the full scene config should be included.
            ///
            /// If `scene_id` is `None` then the currently selected Scene is used
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let scene_as_string = api.scene_to_string(Some(1), false).unwrap();
            /// # }
            /// ```
            pub fn scene_to_string(&self, scene_id: Option<i32>, full_scene_config: bool) -> Result<String, $crate::SceneParamError>  {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let scene_id = scene_id.unwrap_or(-1);

                    let mut size = unsafe {
                        let mut size = 0;
                        let mut param = $crate::plugin_h::OdenSceneParamSceneAsStringEx {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeSceneAsStringEx,
                            next: std::ptr::null_mut(),
                            sceneId: scene_id,
                            sceneString: std::ptr::null_mut(),
                            size: &mut size as *mut _,
                            fullSceneConfig: full_scene_config,
                        };
                        let res = get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void);

                        if res != $crate::SceneParamError::OdenSceneParamErrorOk {
                            return Err(res);
                        }

                        size
                    };

                    let mut scene_string = vec![0_u8; size as usize];

                    let mut param = $crate::plugin_h::OdenSceneParamSceneAsStringEx {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeSceneAsStringEx,
                        next: std::ptr::null_mut(),
                        sceneId: scene_id,
                        sceneString: scene_string.as_mut_ptr() as *mut _,
                        size: &mut size as *mut _,
                        fullSceneConfig: full_scene_config,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let out = unsafe { std::ffi::CStr::from_ptr(param.sceneString) };
                            let out = out.to_str().unwrap();
                            let out: String = out.to_owned();
                            Ok(out)
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the scene_to_string function");
                }
            }

            /// Load a Scene from string
            /// The string `scene_string` containing the scene information is loaded into a new Scene.
            ///
            /// Use [`scene_to_string`](scene_api_impl::scene_to_string) to export a Scene to String.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     let mut scene_string = String::new(); // String containing the scene file contents
            ///
            ///     if api.button("Load Scene from String") {
            ///         api.load_scene_from_string(&scene_string).unwrap();
            ///     }
            /// # }
            /// ```
            pub fn load_scene_from_string(&self, scene_string: &str) -> Result<(), $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let scene_string = std::ffi::CString::new(scene_string.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamLoadSceneFromString {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLoadSceneFromString,
                        next: std::ptr::null_mut(),
                        sceneString: scene_string.as_ptr(),
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the load_scene_from_string function");
                }
            }

            /// Get all entity Uuids from the specified scene
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let scene_id = 0;
            ///     let scene_uuids = api.scene_entity_uuids(scene_id);
            /// # }
            /// ```
            pub fn scene_entity_uuids(&self, scene_id: i32) -> Result<Vec<$crate::math::Uuid>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut uuid_count: i32 = 0;
                    let mut scene_entity_uuids = $crate::plugin_h::OdenSceneParamSceneEntityUuids {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeSceneEntityUuids,
                            next: std::ptr::null_mut(),
                            sceneId: scene_id,
                            uuidCount: &mut uuid_count as *mut i32,
                            uuids: std::ptr::null_mut(),
                        };

                    let res = unsafe { get_scene_param(&mut scene_entity_uuids as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut uuids = vec![$crate::math::Uuid::default(); uuid_count as usize];
                            let uuids_slice = uuids.as_mut_slice();

                            let mut param = $crate::plugin_h::OdenSceneParamSceneEntityUuids {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeSceneEntityUuids,
                                next: std::ptr::null_mut(),
                                sceneId: scene_id,
                                uuidCount: &mut uuid_count as *mut i32,
                                uuids: uuids_slice.as_mut_ptr() as *mut _,
                            };

                            let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                    $crate::SceneParamError::OdenSceneParamErrorOk => {
                                        Ok(uuids)
                                    },
                                    _ => Err(res),
                                }
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the scene_entity_uuids function");
                }
            }

            /// Get flexbox layout
            ///
            /// `padding` contains the outer padding in (top, right, bottom, left) order
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let layout = api.flexbox_layout("name or uuid").unwrap();
            /// # }
            /// ```
            pub fn flexbox_layout(&self, entity_id: &str) -> Result<Vec<$crate::plugin_h::OdenFlexboxLayout>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut layout_count: i32 = 0;
                    let mut padding = $crate::math::Vec4::default();

                    let mut param = $crate::plugin_h::OdenSceneParamFlexboxLayout {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeFlexboxLayout,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        padding: &mut padding as *mut _,
                        layouts: std::ptr::null_mut(),
                        layoutCount: &mut layout_count as *mut i32,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut video_layouts = vec![$crate::plugin_h::OdenFlexboxLayout::default(); layout_count as usize];
                            let video_layouts_slice = video_layouts.as_mut_slice();

                            let mut flexbox_layout = $crate::plugin_h::OdenSceneParamFlexboxLayout {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeFlexboxLayout,
                                next: std::ptr::null_mut(),
                                entityId: entity_id.as_ptr(),
                                padding: &mut padding as *mut _,
                                layouts: video_layouts_slice.as_mut_ptr() as *mut _,
                                layoutCount: &mut layout_count as *mut i32,
                            };

                            let res = unsafe { get_scene_param(&mut flexbox_layout as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                $crate::SceneParamError::OdenSceneParamErrorOk => {
                                    Ok(video_layouts)
                                },
                                _ => Err(res),
                            }
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the flexbox_layout function");
                }
            }

            /// Set flexbox layout padding
            ///
            /// `padding` is applied as (top, right, bottom, left)
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let padding = oden_plugin_rs::math::Vec4::xyzw(10.0, 20.0, 0.0, 0.0);
            ///     api.set_flexbox_padding("name or uuid", &padding).unwrap();
            /// # }
            /// ```
            pub fn set_flexbox_padding(&self, entity_id: &str, padding: &$crate::math::Vec4) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut padding = *padding;

                    let mut param = $crate::plugin_h::OdenSceneParamFlexboxLayout {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeFlexboxLayout,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        padding: &mut padding as *mut _,
                        layoutCount: std::ptr::null_mut(),
                        layouts: std::ptr::null_mut(),
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_flexbox_padding function");
                }
            }

            /// Get the target bitrate at streamer
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     log::info!("Bitrate {}", api.target_bitrate().unwrap());
            /// # }
            /// ```
            pub fn target_bitrate(&self, ) -> Result<f32, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamTargetBitrate {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeTargetBitrate,
                        next: std::ptr::null_mut(),
                        bitrate: 0.0,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(param.bitrate)
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the target_bitrate function");
                }
            }

            /// Return `true` if the `entity` is enabled in the scene graph (not hidden), otherwise `false`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     log::info!("Enabled {}", api.entity_enabled("Test Entity").unwrap());
            /// # }
            /// ```
            pub fn entity_enabled(&self, entity_id: &str) -> Result<bool, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut enabled = false;

                    let mut param = $crate::plugin_h::OdenSceneParamGetEntityEnabled {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeGetEntityEnabled,
                        next: std::ptr::null_mut(),
                        entityNameOrUuid: entity_id.as_ptr(),
                        enabled: &mut enabled,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(enabled)
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the entity_enabled function");
                }
            }

            /// Return the `alignment_id` for the supplied entity `entity_id`.
            ///
            /// `entity_id` must refer to:
            ///     - a 2D video with an OutputAlignment parent
            ///
            /// This function only works in Streamer.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     log::info!("Alignment ID: {}", api.output_alignment_id("entity_name_or_uuid").unwrap());
            /// # }
            /// ```
            pub fn output_alignment_id(&self, entity_id: &str) -> Result<i32, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut alignment_id = -1;

                    let mut param = $crate::plugin_h::OdenSceneParamOutputAlignmentId {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeOutputAlignmentId,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        alignmentId: &mut alignment_id as *mut _
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(alignment_id)
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the output_alignment_id function");
                }
            }

            /// Return the Uuid for a Clone source from `entity_id`.
            /// The `stream` input is used when an entity can clone from more than one source, such a Stitched Video.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(clone_source_id) = api.clone_source_uuid("My Entity Id", 1) {
            ///         log::info!("Source Entity Uuid: {clone_source_id}");
            ///     }
            /// # }
            /// ```
            pub fn clone_source_uuid(&self, entity_id: &str, stream: i32) -> Result<$crate::math::Uuid, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut uuid = $crate::math::Uuid::default();

                    let mut param = $crate::plugin_h::OdenSceneParamCloneSourceUuid {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeCloneSourceUuid,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        stream,
                        uuid: &mut uuid as *mut _,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(uuid)
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the clone_source_uuid function");
                }
            }

            /// Set the Uuid for a Clone source from `entity_id`.
            /// The `stream` input is used when an entity can clone from more than one source, such a Stitched Video.
            ///
            //// / Example
            //// / ```no_run
            //// / # fn example(api: &impl oden_plugin_rs::SceneApi) {
            //// /     if let Ok(clone_source_id) = api.clone_source_uuid("My Entity Id", 1) {
            //// /         log::info!("Source Entity Uuid: {clone_source_id}");
            //// /     }
            //// / # }
            //// / ```
            pub fn set_clone_stream_source(&self, clone_entity_id: &str, clone_stream: i32, source_entity_id: &str, source_stream: i32) -> bool  {
                let clone_entity_id = std::ffi::CString::new(clone_entity_id.trim_end_matches('\0')).unwrap();
                let source_entity_id = std::ffi::CString::new(source_entity_id.trim_end_matches('\0')).unwrap();

                if let Some(set_clone_stream_source) = unsafe { (*self.inner).setCloneStreamSource } {
                    unsafe { set_clone_stream_source(clone_entity_id.as_ptr(), clone_stream, source_entity_id.as_ptr(), source_stream) }
                } else {
                    panic!("This version of Oden is too old to have the set_clone_stream_source function");
                }
            }

            /// Get Rigid Bodies
            ///
            /// Rigid Bodies can be optionally added to models to ease integration with physics engines in plugins.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let rigid_bodies = api.rigid_bodies().unwrap();
            /// # }
            /// ```
            pub fn rigid_bodies(&self) -> Result<Vec<$crate::scene_api::RigidBody>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut count: i32 = 0;

                    let mut rigid_bodies_param = $crate::plugin_h::OdenSceneParamRigidBodies {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRigidBodies,
                        next: std::ptr::null_mut(),
                        rigidBodies: std::ptr::null_mut(),
                        rigidBodyCount: &mut count as *mut i32,
                    };

                    let res = unsafe { get_scene_param(&mut rigid_bodies_param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut rigid_bodies = vec![$crate::plugin_h::OdenRigidBody::default(); count as usize];
                            let rigid_bodies_slice = rigid_bodies.as_mut_slice();

                            let mut rigid_bodies_param = $crate::plugin_h::OdenSceneParamRigidBodies {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRigidBodies,
                                next: std::ptr::null_mut(),
                                rigidBodies: rigid_bodies_slice.as_mut_ptr() as *mut _,
                                rigidBodyCount: &mut count as *mut i32,
                            };

                            let res = unsafe { get_scene_param(&mut rigid_bodies_param as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                $crate::SceneParamError::OdenSceneParamErrorOk => {

                                    let mut res = Vec::new();

                                    for raw_rigid_body in rigid_bodies {

                                        let entity_id = unsafe { std::ffi::CStr::from_ptr(raw_rigid_body.entityId.as_ptr() as _) };
                                        let entity_id = entity_id.to_string_lossy().to_string();

                                        let mut typed_rigid_body = $crate::scene_api::RigidBody{ entity_id, dynamic: raw_rigid_body.dynamic, colliders: Vec::new() };

                                        for i in 0..raw_rigid_body.colliderCount as usize{
                                            let raw_collider = &raw_rigid_body.colliders[i];
                                            typed_rigid_body.colliders.push($crate::scene_api::Collider{
                                                collider_type: match raw_collider.colliderType {
                                                    $crate::plugin_h::OdenColliderType_s_OdenColliderTypeBox => { $crate::scene_api::ColliderType::Box },
                                                    $crate::plugin_h::OdenColliderType_s_OdenColliderTypeCylinder => { $crate::scene_api::ColliderType::Cylinder },
                                                    _ => panic!("Illegal enum value"),
                                                },
                                                density: raw_collider.density,
                                                friction_coefficient: raw_collider.frictionCoefficient,
                                                offset: raw_collider.offset,
                                                rotation: raw_collider.rotation,
                                                size: raw_collider.size,
                                                });
                                        }

                                        res.push(typed_rigid_body);
                                    }

                                    Ok(res)
                                },
                                _ => Err(res),
                            }
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the colliders function");
                }
            }

            /// Get Rigid Bodies (but with uuid and entity name)
            ///
            /// Rigid Bodies can be optionally added to models to ease integration with physics engines in plugins.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let rigid_bodies = api.rigid_bodies().unwrap();
            /// # }
            /// ```
            pub fn rigid_bodies_uuid(&self) -> Result<Vec<$crate::scene_api::RigidBodyUuid>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut count: i32 = 0;

                    let mut rigid_bodies_param = $crate::plugin_h::OdenSceneParamRigidBodiesUuid {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRigidBodiesUuid,
                        next: std::ptr::null_mut(),
                        rigidBodies: std::ptr::null_mut(),
                        rigidBodyCount: &mut count as *mut i32,
                    };

                    let res = unsafe { get_scene_param(&mut rigid_bodies_param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut rigid_bodies = vec![$crate::plugin_h::OdenRigidBodyUuid::default(); count as usize];
                            let rigid_bodies_slice = rigid_bodies.as_mut_slice();

                            let mut rigid_bodies_param = $crate::plugin_h::OdenSceneParamRigidBodiesUuid {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRigidBodiesUuid,
                                next: std::ptr::null_mut(),
                                rigidBodies: rigid_bodies_slice.as_mut_ptr() as *mut _,
                                rigidBodyCount: &mut count as *mut i32,
                            };

                            let res = unsafe { get_scene_param(&mut rigid_bodies_param as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                $crate::SceneParamError::OdenSceneParamErrorOk => {

                                    let mut res = Vec::new();

                                    for raw_rigid_body in rigid_bodies {

                                        let entity_name = unsafe { std::ffi::CStr::from_ptr(raw_rigid_body.entityName.as_ptr() as _) };
                                        let entity_name = entity_name.to_string_lossy().to_string();

                                        let mut typed_rigid_body = $crate::scene_api::RigidBodyUuid{ entity_name, entity_uuid: raw_rigid_body.entityUuid, dynamic: raw_rigid_body.dynamic, colliders: Vec::new() };

                                        for i in 0..raw_rigid_body.colliderCount as usize{
                                            let raw_collider = &raw_rigid_body.colliders[i];
                                            typed_rigid_body.colliders.push($crate::scene_api::Collider{
                                                collider_type: match raw_collider.colliderType {
                                                    $crate::plugin_h::OdenColliderType_s_OdenColliderTypeBox => { $crate::scene_api::ColliderType::Box },
                                                    $crate::plugin_h::OdenColliderType_s_OdenColliderTypeCylinder => { $crate::scene_api::ColliderType::Cylinder },
                                                    _ => panic!("Illegal enum value"),
                                                },
                                                density: raw_collider.density,
                                                friction_coefficient: raw_collider.frictionCoefficient,
                                                offset: raw_collider.offset,
                                                rotation: raw_collider.rotation,
                                                size: raw_collider.size,
                                                });
                                        }

                                        res.push(typed_rigid_body);
                                    }

                                    Ok(res)
                                },
                                _ => Err(res),
                            }
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the colliders function");
                }
            }

            /// Set the calibration for the stitched video entity with the supplied name and for the
            /// video stream with the supplied ID
            ///
            /// `entity_id` must refer to:
            ///     - a stitched video entity
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let calibration = oden_plugin_rs::plugin_h::OdenCameraCalibration::default();
            ///
            ///     if api.set_camera_calibration("Entity Name", 1, calibration){
            ///         log::info!("Calibration was set successful");
            ///     }
            /// # }
            /// ```
            pub fn set_camera_calibration(&self, entity_id: &str, stream: i32, camera_calibration: $crate::CameraCalibration) -> bool {
                if let Some(set_camera_calibration) = unsafe { (*self.inner).setCameraCalibration } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    unsafe { set_camera_calibration(entity_id.as_ptr(), stream, &camera_calibration as *const _) }
                } else {
                    panic!("This version of Oden is too old to have the set_camera_calibration function");
                }
            }

            /// Set the camera calibration from a calibration string (JSON format)
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let calibration_json = r#"{"k": [0.5, 0.1, 0.0, 0.0, 0.0], "offset": {"x": 0.0, "y": 0.0}, "max_theta": 2.5}"#;
            ///
            ///     if api.set_camera_calibration_string("Entity Name", calibration_json){
            ///         log::info!("Calibration was set successful");
            ///     }
            /// # }
            /// ```
            pub fn set_camera_calibration_string(&self, entity_id: &str, calibration: &str) -> bool {
                if let Some(set_camera_calibration_string) = unsafe { (*self.inner).setCameraCalibrationString } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let calibration = std::ffi::CString::new(calibration.trim_end_matches('\0')).unwrap();

                    unsafe { set_camera_calibration_string(entity_id.as_ptr(), calibration.as_ptr()) }
                } else {
                    panic!("This version of Oden is too old to have the set_camera_calibration_string function");
                }
            }

            /// Get dewarping parameters for a 2D video entity
            ///
            /// Returns (enabled, fov_deg, k\[5\], offset\[2\], rotation\[3\])
            ///
            /// rotation is euler angles, degrees
            ///
            /// `entity_id` must refer to a 2D video entity
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     match api.dewarping_params("My 2D Video Entity") {
            ///         Ok((enabled, fov_deg, k, offset, rotation)) => {
            ///             log::info!("Dewarping enabled: {}, FoV: {}°", enabled, fov_deg);
            ///             log::info!("K parameters: {:?}", k);
            ///             log::info!("Offset: {:?}", offset);
            ///             log::info!("Rotation (euler deg): {:?}", rotation);
            ///         }
            ///         Err(e) => log::error!("Failed to get dewarping params: {}", e),
            ///     }
            /// # }
            /// ```
            pub fn dewarping_params(
                &self,
                entity_id: &str,
            ) -> Result<(bool, f32, [f32; 5], [f32; 2], [f32; 3]), &'static str> {
                if let Some(get_dewarping_params) = unsafe { (*self.inner).getDewarpingParams } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut enabled = false;
                    let mut fov_deg = 0.0f32;
                    let mut k = [0.0f32; 5];
                    let mut offset = [0.0f32; 2];
                    let mut rotation = [0.0f32; 3];

                    if unsafe {
                        get_dewarping_params(
                            entity_id.as_ptr(),
                            &mut enabled as *mut _,
                            &mut fov_deg as *mut _,
                            k.as_mut_ptr(),
                            offset.as_mut_ptr(),
                            rotation.as_mut_ptr(),
                        )
                    } {
                        Ok((enabled, fov_deg, k, offset, rotation))
                    } else {
                        Err("Failed to get dewarping params")
                    }
                } else {
                    panic!("This version of Oden is too old to have the get_dewarping_params function");
                }
            }

            /// Set dewarping parameters for a 2D video entity
            ///
            /// `enabled`: whether dewarping is active
            /// `fov_deg`: field of view in degrees (None = don't change)
            /// `k`: camera calibration K parameters (None = don't change)
            /// `offset`: offset parameters (None = don't change)
            /// `rotation_euler_deg`: rotation in euler degrees [x, y, z] (None = don't change)
            ///
            /// `entity_id` must refer to a 2D video entity (FlatVideoConfig type)
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let k = [1.5, 0.1, 0.0, 0.0, 0.0];
            ///     let offset = [0.0, 0.0];
            ///     let rotation = [4.0, 0.0, 0.0];
            ///
            ///     if api.set_dewarping_params("My 2D Video Entity", true, Some(100.0), Some(k), Some(offset), Some(rotation)) {
            ///         log::info!("Dewarping parameters set successfully");
            ///     }
            ///
            ///     if api.set_dewarping_params("My 2D Video Entity", false, None, None, None, None) {
            ///         log::info!("Dewarping disabled");
            ///     }
            /// # }
            /// ```
            pub fn set_dewarping_params(
                &self,
                entity_id: &str,
                enabled: bool,
                fov_deg: Option<f32>,
                k: Option<[f32; 5]>,
                offset: Option<[f32; 2]>,
                rotation_euler_deg: Option<[f32; 3]>,
            ) -> bool {
                if let Some(set_dewarping_params) = unsafe { (*self.inner).setDewarpingParams } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    let fov_ptr = fov_deg.as_ref().map_or(std::ptr::null(), |v| v as *const _);
                    let k_ptr = k.as_ref().map_or(std::ptr::null(), |v| v.as_ptr());
                    let offset_ptr = offset.as_ref().map_or(std::ptr::null(), |v| v.as_ptr());
                    let rotation_ptr = rotation_euler_deg
                        .as_ref()
                        .map_or(std::ptr::null(), |v| v.as_ptr());

                    unsafe {
                        set_dewarping_params(
                            entity_id.as_ptr(),
                            enabled,
                            fov_ptr,
                            k_ptr,
                            offset_ptr,
                            rotation_ptr,
                        )
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_dewarping_params function");
                }
            }


            /// Return the GUI visibility status
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     log::info!("Visible: {}", api.gui_visible());
            /// # }
            /// ```
            pub fn gui_visible(&self) -> bool {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut param = $crate::plugin_h::OdenSceneParamGuiVisible {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeGuiVisible,
                        next: std::ptr::null_mut(),
                        visible: false
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {

                            param.visible
                        }
                        _ => false,
                    }
                } else {
                    panic!("This version of Oden is too old to have the gui_visible function");
                }
            }

            /// Set the position of a virtual camera.
            pub fn set_virtual_camera_position(&self, entity: &str, position: $crate::math::Vec3) {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(set_virtual_camera_position) = unsafe { (*self.inner).setVirtualCameraPosition }
                {
                    unsafe {
                        set_virtual_camera_position(entity.as_ptr(), position);
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the set_virtual_camera_position function"
                    );
                }
            }

            /// Set the rotation of a virtual camera (Euler angles, xyz, radians).
            pub fn set_virtual_camera_rotation(&self, entity: &str, rotation: $crate::math::Vec3) {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(set_virtual_camera_rotation) = unsafe { (*self.inner).setVirtualCameraRotation }
                {
                    unsafe {
                        set_virtual_camera_rotation(entity.as_ptr(), rotation);
                    }
                } else {
                    panic!(
                        "This version of Oden is too old to have the set_virtual_camera_rotation function"
                    );
                }
            }

            /// Set the field of view of a virtual camera (degrees).
            pub fn set_virtual_camera_fov(&self, entity: &str, fov: f32) {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(set_virtual_camera_fov) = unsafe { (*self.inner).setVirtualCameraFov } {
                    unsafe {
                        set_virtual_camera_fov(entity.as_ptr(), fov);
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_virtual_camera_fov function");
                }
            }

            /// Set the resolution of a virtual camera
            pub fn set_virtual_camera_resolution(&self, entity: &str, resolution: $crate::math::Vec2i) {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                if let Some(set_virtual_camera_resolution) = unsafe { (*self.inner).setVirtualCameraResolution } {
                    unsafe {
                        set_virtual_camera_resolution(entity.as_ptr(), resolution.x(), resolution.y());
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_virtual_camera_resolution function");
                }
            }

            /// Fetch the resolution of a virtual camera
            pub fn virtual_camera_resolution(&self, entity: &str) -> Result<$crate::math::Vec2i, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamVirtualCameraResolution {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeVirtualCameraResolution,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        resolution: $crate::math::Vec2i::xy(0, 0),
                    };

                    let res = unsafe { get_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(pos.resolution),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the virtual_camera_resolution function");
                }
            }

            /// Get a quality metric (and confidence of that quality metric) for a Stitched Video's
            ///
            /// Quality is [0, 1], where 0 is a very bad quality and 1 is an exceptional quality.
            /// Confidence is a measure of how much data is used to calculate the quality, also [0, 1].
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Ok(quality) = api.calibration_quality("My Stitched Video Name") {
            ///     // Do something with quality
            /// }
            /// # }
            /// ```
            pub fn calibration_quality(&self, entity: &str) -> Result<$crate::CalibrationQuality, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut pos = $crate::plugin_h::OdenSceneParamStitchedVideoQuality {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeStitchedVideoCalibrationQuality,
                        next: std::ptr::null_mut(),
                        entityId: entity.as_ptr(),
                        quality: 0.0,
                        confidence: 0.0,
                    };

                    let res = unsafe { get_scene_param(&mut pos as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(
                            $crate::CalibrationQuality {
                                quality: pos.quality,
                                confidence: pos.confidence,
                            }),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the calibration_quality function");
                }
            }

            /// Add an entity to the entity tree
            ///
            /// The entity is added after the update loop so any function call to that entity will not be valid
            /// until the next frame.
            ///
            /// `entity` is the config string for the entity:
            /// ```ignore
            /// config : {
            ///     uuid : "f5588fa3-a1b4-41c3-8281-a017253ebdbc";
            ///     scale : [ 0.1, 0.1, 0.1 ];
            ///     type : "gltf_model";
            ///     file : "./assets/rock.glb";
            ///     colliders : ( {
            ///         offset : [ 0.0, 0.0, 0.0 ];
            ///         size : [ 2.4, 2.24, 3.0 ];
            ///         density : 2700.0;
            ///         friction_coefficient : 0.61;
            ///     } );
            /// }
            /// ```
            /// Important thar the `config:` part of the entity string is included.
            ///
            /// `parent` parent entity (will be placed under root if None)
            ///
            /// Returns the uuid of the newly created entity
            #[allow(clippy::needless_lifetimes)]
            pub fn add_entity<'a>(&self, entity: &str, parent: Option<&'a str>) -> Option<$crate::math::Uuid> {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                let parent_str = if let Some(parent) = parent {
                    std::ffi::CString::new(parent.trim_end_matches('\0')).unwrap()
                } else {
                    std::ffi::CString::new("").unwrap()
                };

                let parent_ptr = if parent.is_some() {
                    parent_str.as_ptr()
                } else {
                    std::ptr::null()
                };

                unsafe {
                    if let Some(add_entity) = (*self.inner).addEntity {
                        let uuid = add_entity(entity.as_ptr(), parent_ptr);
                        if uuid.uuid == [0; 16] {
                            return None;
                        } else {
                            return Some(uuid);
                        }
                    } else {
                        panic!("This version of Oden is too old to have the add_entity function");
                    }
                }

            }

            /// Remove an entity
            ///
            /// `entity` is the name/id or uuid of the entity to be removed
            pub fn remove_entity(&self, entity: &str) {
                let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                unsafe {
                    if let Some(remove_entity) = (*self.inner).removeEntity {
                        remove_entity(entity.as_ptr());
                    } else {
                        panic!("This version of Oden is too old to have the remove_entity function");
                    }
                }
            }

            /// Fetch the calibration of a stitched video from the streamer
            ///
            /// `remote_streamer` is the uuid of the Remote Streamer entity that is receiving data from the streamer.
            /// `remote_entity` is the uuid of the enity on the streamer we want the calibration from
            ///
            /// When calling this function the first time a request will be issued and None will be returned. On subsequent calls when the data is received it will
            /// return Some. You should call this function once a frame.
            pub fn poll_remote_calibration(
                &self,
                remote_streamer: $crate::plugin_h::OdenUuid,
                remote_entity: $crate::plugin_h::OdenUuid,
            ) -> Option<$crate::scene_api::RemoteCalibrationResult> {
                unsafe {
                    if let Some(poll_remote_calibration) = (*self.inner).pollRemoteCalibration {

                        let mut count = 0;
                        poll_remote_calibration(remote_streamer, remote_entity, &mut count, std::ptr::null_mut(), std::ptr::null_mut());

                        let mut res = $crate::scene_api::RemoteCalibrationResult {
                            floor_height: 0.0,
                            calibrations: Vec::new(),
                        };
                        res.calibrations.resize(count as usize, Default::default());

                        if poll_remote_calibration(remote_streamer, remote_entity, &mut count, res.calibrations.as_mut_ptr(), &mut res.floor_height as *mut _) {
                            return Some(res);
                        }

                        None

                    } else {
                        panic!("This version of Oden is too old to have the poll_remote_calibration function");
                    }
                }
            }

            /// Gets the crop information from the entity with the supplied name and stream.
            /// Return None if entity or stream does not exist, supply 1 as stream on 2D videos.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Some(crop) = api.camera_crop("My Stitched Video Name", 1) {
            ///     // Do something with crop
            ///     }
            /// # }
            /// ```
            pub fn camera_crop(&self, entity: &str, stream: i32) -> Option<$crate::CameraCropData> {
                if let Some(camera_crop_data) = unsafe { (*self.inner).getCameraCropData } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                    let mut crop = $crate::CameraCropData::default();

                    if unsafe { camera_crop_data(entity.as_ptr(), stream, &mut crop as *mut _) } {
                        Some(crop)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_crop function");
                }
            }

            /// Set the crop for the entity with the supplied name and stream.
            /// Return None if entity or if stream does not exist, supply 1 as stream on 2D videos.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let crop = &oden_plugin_rs::CameraCropData::default();
            ///     api.set_camera_crop("My Stitched Video Name", 1, &crop);
            /// # }
            /// ```
            pub fn set_camera_crop(&self, entity: &str, stream: i32, crop: &$crate::CameraCropData) -> Result<(), $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamCameraCrop {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeCameraCrop,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        cropData: *crop,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_camera_crop function");
                }
            }

            /// Gets the hard crop information from the entity with the supplied name and stream.
            /// Return None if entity stream does not exist, the source is not a clone stream, or it does not use
            /// "Copy To Texture" setting. Supply 1 as stream on 2D videos.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Ok(hard_crop) = api.camera_hard_crop("My Stitched Video Name", 1) {
            ///     // Do something with crop
            ///     }
            /// # }
            /// ```
            pub fn camera_hard_crop(&self, entity: &str, stream: i32) -> Result<$crate::math::Vec4, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamCameraHardCrop {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeCameraHardCrop,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        hardCropData: $crate::math::Vec4::default(),
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(param.hardCropData)
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the camera_hard_crop function");
                }
            }

            /// Sets the hard crop for the entity with the supplied name and stream, if video does not have "Copy To Texture"
            /// enabled it will enabled after the function call.
            /// Return None if entity, stream does not exist or if the source is not a clone stream, supply 1 as stream on 2D videos.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let hard_crop = oden_plugin_rs::math::Vec4::xyzw(0.5, 0.0, 0.5, 0.0);
            /// if let Ok(crop) = api.set_camera_hard_crop("My Stitched Video Name", 1, &hard_crop) {
            ///     // Do something with crop
            ///     }
            /// # }
            /// ```
            pub fn set_camera_hard_crop(&self, entity: &str, stream: i32, hard_crop: &$crate::math::Vec4) -> Result<(), $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamCameraHardCrop {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeCameraHardCrop,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        hardCropData: *hard_crop,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_camera_hard_crop function");
                }
            }

            pub fn drop_detector_timeout(&self, entity: &str, stream: i32) -> Result<f32, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamDropDetectorTimeout {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeDropDetectorTimeout,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        timeoutMs: 0.0,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(param.timeoutMs),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the drop_detector_timeout function");
                }
            }

            pub fn set_drop_detector_timeout(&self, entity: &str, stream: i32, timeout_ms: f32) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamDropDetectorTimeout {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeDropDetectorTimeout,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        timeoutMs: timeout_ms,
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_drop_detector_timeout function");
                }
            }

            pub fn show_no_signal_screen(&self, entity: &str, stream: i32) -> Result<bool, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamShowNoSignalScreen {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeShowNoSignalScreen,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        show: false,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(param.show),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the show_no_signal_screen function");
                }
            }

            pub fn set_show_no_signal_screen(&self, entity: &str, stream: i32, show: bool) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamShowNoSignalScreen {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeShowNoSignalScreen,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        streamIndex: stream,
                        show,
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_show_no_signal_screen function");
                }
            }

            /// Get the scale of an entity
            ///
            /// Returns the current scale (x, y, z) of the specified entity
            ///
            /// # Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     match api.entity_scale("Scene 1@my_entity") {
            ///         Ok(scale) => {
            ///             log::info!("Entity scale: x={}, y={}, z={}", scale.x(), scale.y(), scale.z());
            ///         }
            ///         Err(e) => log::error!("Failed to get entity scale: {:?}", e),
            ///     }
            /// # }
            /// ```
            pub fn entity_scale(&self, entity: &str) -> Result<$crate::math::Vec3, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamEntityScale {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeEntityScale,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        scale: $crate::plugin_h::OdenVec3 { d: [0.0, 0.0, 0.0] },
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok($crate::math::Vec3::xyz(param.scale.d[0], param.scale.d[1], param.scale.d[2]))
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the entity_scale function");
                }
            }

            /// Set the scale of an entity
            ///
            /// Sets the scale (x, y, z) of the specified entity
            ///
            /// # Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     use oden_plugin_rs::math::Vec3;
            ///
            ///     let scale = Vec3::xyz(2.0, 2.0, 2.0);  // 2x scale in all directions
            ///     if let Err(e) = api.set_entity_scale("Scene 1@my_entity", &scale) {
            ///         log::error!("Failed to set entity scale: {:?}", e);
            ///     }
            /// # }
            /// ```
            pub fn set_entity_scale(&self, entity: &str, scale: &$crate::math::Vec3) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let entity_id = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    let mut param = $crate::plugin_h::OdenSceneParamEntityScale {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeEntityScale,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        scale: $crate::plugin_h::OdenVec3 { d: [scale.x(), scale.y(), scale.z()] },
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_entity_scale function");
                }
            }

            /// Retrive the storing location for raw recordings.
            ///
            /// For Linux systems, the default save location is the %HOME%/USER/ folder.
            /// For Windows systems, the default save location is the %USERPROFILE%\Videos\ directory.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// if let Ok(raw_recording_folder) = api.raw_record_folder() {
            ///     // Do something with raw_recording_folder
            ///     }
            /// # }
            /// ```
            pub fn raw_record_folder(&self) -> Result<String, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut path_size = {
                        let mut path_size = 0;

                        let mut get = $crate::plugin_h::OdenSceneParamRawRecordingFolder {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRawRecordingFolder,
                            next: std::ptr::null_mut(),
                            path: std::ptr::null_mut(),
                            pathSize: &mut path_size as *mut i32
                        };

                        let res = unsafe { get_scene_param(&mut get as *mut _ as *mut std::os::raw::c_void) };

                        if res != $crate::SceneParamError::OdenSceneParamErrorOk {
                            return Err(res);
                        }

                        path_size
                    };

                    let mut raw_recording_folder_path = vec![0_u8; path_size as usize];

                    let mut param = $crate::plugin_h::OdenSceneParamRawRecordingFolder {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRawRecordingFolder,
                        next: std::ptr::null_mut(),
                        path: raw_recording_folder_path.as_mut_ptr() as *mut _,
                        pathSize: &mut path_size as *mut i32
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let out = unsafe { std::ffi::CStr::from_ptr(param.path) };
                            let out = out.to_str().unwrap();
                            let out: String = out.to_owned();
                            Ok(out)
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the raw_record_folder function");
                }
            }

            /// Specifies the directory for storing raw recordings.
            ///
            /// If no path is set, the storage location defaults to the current project's directory.
            /// If there's no active project, the system's standard save location is used instead.
            /// For Linux systems, the default save location is the %HOME%/USER/ folder.
            /// For Windows systems, the default save location is the %USERPROFILE%\Videos\ directory.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            /// let mut folder = String::new();
            /// if api.input_text("Folder: ", &mut folder) {
            ///     api.set_raw_record_folder(&folder).ok();
            /// }
            /// # }
            /// ```
            pub fn set_raw_record_folder(&self, folder_path: &str) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut folder_path = std::ffi::CString::new(folder_path.trim_end_matches('\0')).unwrap().into_bytes();

                    let mut set = $crate::plugin_h::OdenSceneParamRawRecordingFolder {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRawRecordingFolder,
                        next: std::ptr::null_mut(),
                        path: folder_path.as_mut_ptr() as *mut _,
                        pathSize: std::ptr::null_mut(),
                    };

                    let res = unsafe { set_scene_param(&mut set as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_raw_record_folder function");
                }
            }

            /// Get the list of uuids that a virtual camera has in its Render Only list. The three supported entities are Virtual Camera, Virtual Cubemap Camera and Persistent Map.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let scene_uuids = api.virtual_camera_render_only_uuids("entity_id");
            /// # }
            /// ```
            pub fn virtual_camera_render_only_uuids(&self, entity_id: &str) -> Result<Vec<$crate::math::Uuid>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut uuid_count: i32 = 0;
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    let mut scene_entity_uuids = $crate::plugin_h::OdenSceneParamVirtualCameraOnlyRenderUuids {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeVirtualCameraOnlyRenderUuids,
                            next: std::ptr::null_mut(),
                            entityId: entity_id.as_ptr(),
                            uuidCount: &mut uuid_count as *mut i32,
                            uuids: std::ptr::null_mut(),
                        };

                    let res = unsafe { get_scene_param(&mut scene_entity_uuids as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut uuids = vec![$crate::math::Uuid::default(); uuid_count as usize];
                            let uuids_slice = uuids.as_mut_slice();

                            let mut param = $crate::plugin_h::OdenSceneParamVirtualCameraOnlyRenderUuids {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeVirtualCameraOnlyRenderUuids,
                                next: std::ptr::null_mut(),
                                entityId: entity_id.as_ptr(),
                                uuidCount: &mut uuid_count as *mut i32,
                                uuids: uuids_slice.as_mut_ptr() as *mut _,
                            };

                            let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                    $crate::SceneParamError::OdenSceneParamErrorOk => {
                                        Ok(uuids)
                                    },
                                    _ => Err(res),
                                }
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the scene_entity_uuids function");
                }
            }

            /// Sets the geometry to project a stitched video onto
            ///
            /// The vertices parameter is a list of triplets of vertices forming triangles, where each vertex is a triplet of coordinates, x, y, and z.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.set_external_stitch_geometry("Scene 1@my_stitched_video", &[0.0, 0.5, 0.0, -1.0, -0.5, -1.0,  1.0, -0.5, 0.0]);
            /// # }
            /// ```
            pub fn set_external_stitch_geometry(&self, entity: &str, vertices: &[f32]) -> Result<(), $crate::SceneParamError> {

                if let Some(set_external_stitch_geometry) = unsafe { (*self.inner).setExternalStitchGeometry } {
                    let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                    if unsafe { set_external_stitch_geometry(entity.as_ptr(), vertices.as_ptr(), vertices.len() as i32) } {
                        Ok(())
                    } else {
                        Err($crate::SceneParamError::OdenSceneParamErrorInvalidEntity)
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_external_stitch_geometry function");
                }
            }

            /// Makes a ray intersection with the supplied video source and works for both 2D Videos and Stitched video.
            /// Will give the closest intersection if multiple is found. This function is relative heavy so use carefully
            /// especially when intersecting with imported models. The resulting vector will be of the same length as
            /// the number of streams available in the video entity. The results in the vector is in the same order as
            /// the streams, i.e stream 1 is on position 0 and stream 2 is on position 1 since there are no stream 0.
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::UpdateApi)) {
            ///     let Some(ray) = api.virtual_camera_pixel_to_ray("virtual_camera_entity_id", 100.0, 100.0) else{
            ///         return;
            ///     };
            ///
            ///     let intersections = api.ray_to_video_intersect("stitched_video_entity_id", ray);
            /// # }
            /// ```
            pub fn ray_to_video_intersect(&self, entity_id: &str, ray: $crate::plugin_h::OdenRay ) -> Result<Vec<$crate::plugin_h::OdenRayToVideoIntersect>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut stream_count: i32 = 0;

                    let mut scene_entity_uuids = $crate::plugin_h::OdenSceneParamRayToVideoIntersect {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRayToVideoIntersect,
                            next: std::ptr::null_mut(),
                            entityId: entity_id.as_ptr(),
                            ray,
                            resultCount: &mut stream_count as *mut i32,
                            intersectResult: std::ptr::null_mut(),
                        };

                    let res = unsafe { get_scene_param(&mut scene_entity_uuids as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut result = vec![$crate::plugin_h::OdenRayToVideoIntersect::default(); stream_count as usize];
                            let result_slice = result.as_mut_slice();

                            let mut param = $crate::plugin_h::OdenSceneParamRayToVideoIntersect {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRayToVideoIntersect,
                                next: std::ptr::null_mut(),
                                entityId: entity_id.as_ptr(),
                                ray,
                                resultCount: &mut stream_count as *mut i32,
                                intersectResult: result_slice.as_mut_ptr() as *mut _,
                            };

                            let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                    $crate::SceneParamError::OdenSceneParamErrorOk => {
                                        Ok(result)
                                    },
                                    _ => Err(res),
                                }
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the scene_entity_uuids function");
                }
            }

            /// Retrieves the UUID of an entity given its UUID or name.
            ///
            /// This function looks up an entity by its UUID or name and returns the corresponding UUID if found.
            /// If the entity is not found, it returns `None`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity_uuid = api.entity_uuid("Entity Id").unwrap();
            /// # }
            /// ```
            pub fn entity_uuid(&self, entity_id: &str) -> Option<$crate::math::Uuid> {
                let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                if let Some(get_entity_uuid) = unsafe { (*self.inner).getEntityUuid } {
                    let mut uuid = $crate::math::Uuid::default();
                    if unsafe {get_entity_uuid(entity_id.as_ptr(), &mut uuid)}{
                        Some(uuid)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the entity_uuid function");
                }
            }

            /// Retrieves the parent entity's UUID given the UUID or name of a child entity.
            ///
            /// This function looks up the parent of a specified entity by its UUID or name and returns the parent's UUID if found.
            /// If the parent entity or the specified entity is not found, it returns `None`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SettingsApi + oden_plugin_rs::SceneApi)) {
            ///     if let Some(parent) = api
            ///         .instance_uuid()
            ///         .map(|uuid| uuid.to_string())
            ///         .and_then(|uuid| api.parent_uuid(&uuid))
            ///     {
            ///         // Do something with parent
            ///     }
            /// # }
            /// ```
            pub fn parent_uuid(&self, entity_id: &str) -> Option<$crate::math::Uuid> {
                let entity = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                if let Some(get_parent_uuid) = unsafe { (*self.inner).getParentUuid } {
                    let mut parent = $crate::math::Uuid::default();
                    if unsafe {get_parent_uuid(entity.as_ptr(), &mut parent)}{
                        Some(parent)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the parent_uuid function");
                }
            }

            /// Returns the ancestor UUID of an entity, `nth` levels up from the specified `entity_id`.
            ///
            /// This function traverses the entity's parent hierarchy to find the ancestor at the specified level.
            /// If the entity doesn't have enough ancestors (i.e., if it's less than `nth` levels deep), the function returns `None`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SettingsApi + oden_plugin_rs::SceneApi)) {
            ///     if let Some(parent) = api
            ///         .instance_uuid()
            ///         .map(|uuid| uuid.to_string())
            ///         .and_then(|uuid| api.ancestor_uuid(&uuid, 2))
            ///     {
            ///         // Do something with parent
            ///     }
            /// # }
            /// ```
            pub fn ancestor_uuid(&self, entity_id: &str, nth: i32) -> Option<$crate::math::Uuid> {
                if let Some(mut current_entity) = self.entity_uuid(entity_id).map(|uuid| uuid.to_string()) {
                    for _ in 0..nth {
                        match self.parent_uuid(&current_entity) {
                            Some(parent) => current_entity = parent.to_string(),
                            None => return None,
                        }
                    }

                    current_entity.parse().ok()
                } else {
                    None
                }
            }

            /// Returns the rotations of each flexbox.
            ///
            /// Take in the entity name or UUID for the flexbox entity and return the rotation of all its children.
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(parent) = api
            ///         .flexbox_rotation("Flexbox_name")
            ///     {
            ///         // Do something with rotations
            ///     }
            /// # }
            /// ```
            pub fn flexbox_rotation(&self, entity_id: &str) -> Result<Vec<$crate::plugin_h::OdenFlexboxRotation>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();
                    let mut rotation_count: i32 = 0;

                    let mut param = $crate::plugin_h::OdenSceneParamFlexboxRotation {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeFlexboxRotation,
                        next: std::ptr::null_mut(),
                        entityId: entity_id.as_ptr(),
                        rotations: std::ptr::null_mut(),
                        rotationsCount: &mut rotation_count as *mut i32,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut rotations = vec![$crate::plugin_h::OdenFlexboxRotation::default(); rotation_count as usize];
                            let rotations_slice = rotations.as_mut_slice();

                            let mut flexbox_rotations = $crate::plugin_h::OdenSceneParamFlexboxRotation {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeFlexboxRotation,
                                next: std::ptr::null_mut(),
                                entityId: entity_id.as_ptr(),
                                rotations: rotations_slice.as_mut_ptr() as *mut _,
                                rotationsCount: &mut rotation_count as *mut i32,
                            };

                            let res = unsafe { get_scene_param(&mut flexbox_rotations as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                $crate::SceneParamError::OdenSceneParamErrorOk => {
                                    Ok(rotations)
                                },
                                _ => Err(res),
                            }
                        }
                        _ => Err(res),
                    }


                } else {
                    panic!("This version of Oden is too old to have the flexbox_rotation function");
                }
            }

            /// Returns performance metrics for the last frame
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     if let Ok(metrics) = api.performance_metrics() {
            ///         // Do something with metrics
            ///     }
            /// # }
            /// ```
            pub fn performance_metrics(&self) -> Result<$crate::PerformanceMetrics<'_>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamPerformanceMetrics_s {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePerformanceMetrics,
                        ..Default::default()
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok($crate::PerformanceMetrics::from(param))
                        }
                        _ => Err(res),
                    }

                } else {
                    panic!("This version of Oden is too old to have the performance_metrics function");
                }
            }

            /// Enable or disable Encryption for a specific link `index`.
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     api.set_link_encryption_enabled(entity, index, true);
            /// # }
            /// ```
            pub fn set_link_encryption_enabled(
                &self,
                entity: Option<&str>,
                index: i32,
                enable: bool
            ) -> Result<(), $crate::LinkError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity_str = if let Some(entity) = entity {
                        std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap()
                    } else {
                        std::ffi::CString::new("").unwrap()
                    };

                    let entity_ptr = if entity.is_some() {
                        entity_str.as_ptr()
                    } else {
                        std::ptr::null()
                    };

                    let mut param = $crate::plugin_h::OdenSceneParamLinkEncryption {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLinkEncryption,
                        next: std::ptr::null_mut(),
                        entityId: entity_ptr,
                        enable,
                        index,
                        result: $crate::LinkError::OdenLinkErrorUnknown,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok(())
                        },
                        _ => Err(param.result),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_encryption_enabled function");
                }
            }

            /// Clears all the allowed encryption keys for a specific link `index`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     api.clear_link_encryption_allowed_public_keys(entity, index);
            /// # }
            /// ```
            pub fn clear_link_encryption_allowed_public_keys(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<(), $crate::LinkError> {
                if let Some(clear_link_encryption_allowed_public_keys) = unsafe{(*self.inner).clearLinkEncryptionAllowedPublicKeys} {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { clear_link_encryption_allowed_public_keys(entity.as_ptr(), index) }
                        }
                        None => unsafe { clear_link_encryption_allowed_public_keys(std::ptr::null(), index) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the clear_link_encryption_allowed_public_keys function");
                }
            }

            /// Sets the link with `index` to a Relay Link.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     api.set_link_to_relay_link(entity, index, true);
            /// # }
            /// ```
            pub fn set_link_to_relay_link(
                &self,
                entity: Option<& str>,
                index: i32,
                enable: bool,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_to_relay_link) = unsafe{(*self.inner).setLinkToRelayLink} {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_to_relay_link(entity.as_ptr(), index, enable) }
                        }
                        None => unsafe { set_link_to_relay_link(std::ptr::null(), index, enable) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_to_relay_link function");
                }
            }

            /// Determine if the link with `index` is a Relay Link.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     if let Ok(is_relay_link) = api.is_link_relay_link(entity, index) {
            ///         if is_relay_link {
            ///             log::info!("Link is relay link!")
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn is_link_relay_link(
                &self,
                entity: Option<& str>,
                index: i32,
            ) -> Result<bool, $crate::LinkError> {
                if let Some(is_link_relay_link) = unsafe{(*self.inner).isLinkRelayLink} {
                    let mut res_bool = false;
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { is_link_relay_link(entity.as_ptr(), index, &mut res_bool) }
                        }
                        None => unsafe { is_link_relay_link(std::ptr::null(), index, &mut res_bool) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(res_bool),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_link_relay_link function");
                }
            }

            /// Start or stop relaying traffic from the link with `index` to the relay link(s).
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     api.set_link_send_to_relay(entity, index, true);
            /// # }
            /// ```
            pub fn set_link_send_to_relay(
                &self,
                entity: Option<& str>,
                index: i32,
                enable: bool,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_send_to_relay) = unsafe{(*self.inner).setLinkSendToRelay} {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { set_link_send_to_relay(entity.as_ptr(), index, enable) }
                        }
                        None => unsafe { set_link_send_to_relay(std::ptr::null(), index, enable) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_send_to_relay function");
                }
            }

            /// Determine if the link with `index` is relaying traffic to the relay link(s).
            ///
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     if let Ok(is_sending_to_relay_link) = api.is_link_sending_to_relay(entity, index) {
            ///         if is_sending_to_relay_link {
            ///             log::info!("Link is sending to relay link(s)!")
            ///         }
            ///     }
            /// # }
            /// ```
            pub fn is_link_sending_to_relay(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<bool, $crate::LinkError> {
                if let Some(is_link_sending_to_relay) = unsafe{(*self.inner).isLinkSendingToRelay} {
                    let mut res_bool = false;
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { is_link_sending_to_relay(entity.as_ptr(), index, &mut res_bool) }
                        }
                        None => unsafe { is_link_sending_to_relay(std::ptr::null(), index, &mut res_bool) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(res_bool),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_link_sending_to_relay function");
                }
            }

            /// Start or stop outgoing network traffic from the link with `index`.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let entity = None;
            ///     let index = 0;
            ///     api.set_link_drop_outgoing_packets(entity, index, true);
            /// # }
            /// ```
            pub fn set_link_drop_outgoing_packets(
                &self,
                entity: Option<& str>,
                index: i32,
                drop: bool,
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_link_drop_outgoing_packets) = unsafe{(*self.inner).setLinkDropOutgoingPackets} {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                            unsafe { set_link_drop_outgoing_packets(entity.as_ptr(), index, drop) }
                        }
                        None => unsafe { set_link_drop_outgoing_packets(std::ptr::null(), index, drop) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_link_drop_outgoing_packets function");
                }
            }

            /// Get video info for the given remote streamer entity
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let remote_streamer_videos = api.remote_streamer_video_info("remote streamer id");
            ///     // Do something with the videos
            /// # }
            /// ```
            pub fn remote_streamer_video_info(
                &self,
                remote_streamer_entity: &str
            ) -> Result<Vec<$crate::plugin_h::OdenRemoteStreamerVideo>, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let remote_streamer_entity = std::ffi::CString::new(remote_streamer_entity.trim_end_matches('\0')).unwrap();
                    let mut videos_count: i32 = 0;

                    let mut data_out = $crate::plugin_h::OdenSceneParamRemoteStreamerVideos {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRemoteStreamerVideos,
                        next: std::ptr::null_mut(),
                        remoteStreamerEntity: remote_streamer_entity.as_ptr(),
                        videos: std::ptr::null_mut(),
                        videosCount: &mut videos_count as *mut i32,
                    };

                    let res = unsafe { get_scene_param(&mut data_out as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let mut videos = vec![$crate::plugin_h::OdenRemoteStreamerVideo::default(); videos_count as usize];
                            let videos_slice = videos.as_mut_slice();

                            let mut data_out = $crate::plugin_h::OdenSceneParamRemoteStreamerVideos {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeRemoteStreamerVideos,
                                next: std::ptr::null_mut(),
                                remoteStreamerEntity: remote_streamer_entity.as_ptr(),
                                videos: videos_slice.as_mut_ptr() as *mut _,
                                videosCount: &mut videos_count as *mut i32,
                            };

                            let res = unsafe { get_scene_param(&mut data_out as *mut _ as *mut std::os::raw::c_void) };
                            match res {
                                $crate::SceneParamError::OdenSceneParamErrorOk => {
                                    Ok(videos)
                                },
                                _ => Err(res),
                            }
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the get_scene_param function");
                }
            }

            /// Get the name for the stream of the given capture uuid
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let stream_name = api.stream_name("Entity Id", 1);
            ///     // Do something with the stream name
            /// # }
            /// ```
            pub fn stream_name(&self, entity_id: &str, stream: i32) -> Result<String, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let entity_id = std::ffi::CString::new(entity_id.trim_end_matches('\0')).unwrap();

                    let mut string_len = {
                        let mut string_len = 0;


                        let mut get = $crate::plugin_h::OdenSceneParamStreamName {
                            type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeStreamName,
                            next: std::ptr::null_mut(),
                            entityId:entity_id.as_ptr(),
                            stream,
                            streamName: std::ptr::null_mut(),
                            streamNameLen: &mut string_len as *mut i32
                        };

                        let res = unsafe { get_scene_param(&mut get as *mut _ as *mut std::os::raw::c_void) };

                        if res != $crate::SceneParamError::OdenSceneParamErrorOk {
                            return Err(res);
                        }

                        string_len
                    };

                    let mut stream_name = vec![0_u8; string_len as usize];

                    let mut param = $crate::plugin_h::OdenSceneParamStreamName {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeStreamName,
                        next: std::ptr::null_mut(),
                        entityId:entity_id.as_ptr(),
                        stream,
                        streamName: stream_name.as_mut_ptr() as *mut _,
                        streamNameLen: &mut string_len as *mut i32
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            let out = unsafe { std::ffi::CStr::from_ptr(param.streamName) };
                            let out = out.to_str().unwrap();
                            let out: String = out.to_owned();
                            Ok(out)
                        },
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the get_scene_param function");
                }
            }

            /// Enable the packer.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.set_packer_enabled(true).unwrap();
            /// # }
            /// ```
            pub fn set_packer_enabled(&self, enabled: bool) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamPackerEnabled {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePackerEnabled,
                        next: std::ptr::null_mut(),
                        enabled,
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_packer_enabled function");
                }
            }

            /// Enable the packer auto crop feature.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.set_packer_auto_crop(true).unwrap();
            /// # }
            /// ```
            pub fn set_packer_auto_crop(&self, enabled: bool) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamPackerAutoCrop {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePackerAutoCrop,
                        next: std::ptr::null_mut(),
                        enabled,
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_packer_auto_crop function");
                }
            }

            /// Returns if the packer is enabled or not for the loaded project
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let packer_enabled = api.is_packer_enabled().unwrap();
            /// # }
            /// ```
            pub fn is_packer_enabled(&self) -> Result<bool, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamPackerEnabled {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePackerEnabled,
                        next: std::ptr::null_mut(),
                        enabled: false,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(param.enabled),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_packer_enabled function");
                }
            }

            /// Returns if the packer auto crop feaature is enabled or not for the loaded project
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let packer_enabled = api.is_packer_auto_crop_enabled().unwrap();
            /// # }
            /// ```
            pub fn is_packer_auto_crop_enabled(&self) -> Result<bool, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamPackerAutoCrop {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypePackerAutoCrop,
                        next: std::ptr::null_mut(),
                        enabled: false,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(param.enabled),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the is_packer_auto_crop_enabled function");
                }
            }

            pub fn set_background_color(&self, color: &$crate::math::Vec3) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamBackgroundColor {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeBackgroundColor,
                        next: std::ptr::null_mut(),
                        color: $crate::plugin_h::OdenVec3 { d: [color.x(), color.y(), color.z()] },
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_background_color function");
                }
            }

            pub fn background_color(&self) -> Result<$crate::math::Vec3, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamBackgroundColor {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeBackgroundColor,
                        next: std::ptr::null_mut(),
                        color: $crate::plugin_h::OdenVec3 { d: [0.0, 0.0, 0.0] },
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            Ok($crate::math::Vec3::xyz(param.color.d[0], param.color.d[1], param.color.d[2]))
                        }
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the background_color function");
                }
            }

            pub fn clear_background_color(&self) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                    let mut param = $crate::plugin_h::OdenSceneParamClearBackgroundColor {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeClearBackgroundColor,
                        next: std::ptr::null_mut(),
                    };

                    let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the clear_background_color function");
                }
            }

            /// Returns whether the Oden window is focused or not.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &(impl oden_plugin_rs::SceneApi + oden_plugin_rs::GuiApi)) {
            ///     api.label(&format!("Focused: {}", api.window_focused()));
            /// # }
            /// ```
            pub fn window_focused(&self) -> bool {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let mut param = $crate::plugin_h::OdenSceneParamWindowFocus {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeWindowFocus,
                        next: std::ptr::null_mut(),
                        focused: false,
                    };

                    let _ = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    param.focused
                } else {
                    panic!("This version of Oden is too old to have the window_focused function");

                }
            }

            /// Set current Oden window to be in focus
            ///
            /// Ie make it the top window, where eg button presses go.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.focus_window();
            /// # }
            /// ```
            pub fn focus_window(&self) {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let mut param = $crate::plugin_h::OdenSceneParamWindowFocus {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeWindowFocus,
                        next: std::ptr::null_mut(),
                        focused:true,
                    };
                    let _ = unsafe { set_scene_param(&mut param as *mut _ as *const std::os::raw::c_void) };
                } else {
                    panic!("This version of Oden is too old to have the set_window_focus function");
                }
            }

            /// Returns the current audio output volume for a given Remote Streamer or Streamer.
            ///
            /// Value in fractions, [0.0, 1.0]
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let volume = api.audio_output_volume(Some("Remote Streamer Id")).unwrap();
            /// # }
            /// ```
            pub fn audio_output_volume(&self, remote_streamer: Option<&str>) -> Result<f32, $crate::SceneParamError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let volume;

                    let res = match remote_streamer {
                        Some(remote_streamer) => {
                            let remote_streamer = std::ffi::CString::new(remote_streamer.trim_end_matches('\0')).unwrap();
                            let mut param = $crate::plugin_h::OdenSceneParamAudioOutputVolume {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeAudioOutputVolume,
                                next: std::ptr::null_mut(),
                                remoteStreamerId: remote_streamer.as_ptr(),
                                volume: 1.0,
                            };

                            let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                            volume = param.volume;

                            res
                        }
                        None => {
                            let mut param = $crate::plugin_h::OdenSceneParamAudioOutputVolume {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeAudioOutputVolume,
                                next: std::ptr::null_mut(),
                                remoteStreamerId: std::ptr::null(),
                                volume: 1.0,
                            };

                            let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                            volume = param.volume;

                            res
                        }
                    };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(volume),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the audio_output_volume function");
                }
            }

            /// Sets the current audio output volume for a given Remote Streamer or Streamer.
            ///
            /// Value in fractions, [0.0, 1.0]
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let _ = api.set_audio_output_volume(Some("Remote Streamer Id"), 0.7).unwrap();
            /// # }
            /// ```
            pub fn set_audio_output_volume(&self, remote_streamer: Option<&str>, volume: f32) -> Result<(), $crate::SceneParamError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let res = match remote_streamer {
                        Some(remote_streamer) => {
                            let remote_streamer = std::ffi::CString::new(remote_streamer.trim_end_matches('\0')).unwrap();
                            let mut param = $crate::plugin_h::OdenSceneParamAudioOutputVolume {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeAudioOutputVolume,
                                next: std::ptr::null_mut(),
                                remoteStreamerId: remote_streamer.as_ptr(),
                                volume,
                            };

                            unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) }
                        }
                        None => {
                            let mut param = $crate::plugin_h::OdenSceneParamAudioOutputVolume {
                                type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeAudioOutputVolume,
                                next: std::ptr::null_mut(),
                                remoteStreamerId: std::ptr::null(),
                                volume,
                            };

                            unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) }
                        }
                    };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the audio_output_volume function");
                }
            }

            /// Returns the bind device for the given link `index`.
            ///
            /// Specify `entity` as [`None`] for Remote Streamer on Streamer-
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     let bind_device = api.link_bind_device(None, 0);
            /// # }
            /// ```
            pub fn link_bind_device(
                &self,
                entity: Option<&'_ str>,
                index: i32,
            ) -> Result<String, $crate::LinkError> {
                if let Some(get_scene_param) = unsafe { (*self.inner).getSceneParam } {
                    let entity_str = if let Some(entity) = entity {
                        std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap()
                    } else {
                        std::ffi::CString::new("").unwrap()
                    };

                    let entity_ptr = if entity.is_some() {
                        entity_str.as_ptr()
                    } else {
                        std::ptr::null()
                    };

                    let mut buffer = vec![0; 1024];
                    let ptr = buffer.as_mut_ptr();
                    let mut size = buffer.len() as i32;

                    let mut param = $crate::plugin_h::OdenSceneParamLinkBindDevice {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLinkBindDevice,
                        next: std::ptr::null_mut(),
                        entityId: entity_ptr,
                        index,
                        buffer: ptr as *mut std::os::raw::c_char,
                        buffer_lenght: &mut size as *mut i32,
                        result: $crate::LinkError::OdenLinkErrorUnknown,
                    };

                    let res = unsafe { get_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                    match res {
                        $crate::SceneParamError::OdenSceneParamErrorOk => {
                            match $crate::utils::utf8_from_raw(&buffer) {
                                Ok(buffer) => Ok(buffer),
                                Err(_) => Err($crate::LinkError::OdenLinkErrorUnknown),
                            }
                        }
                        _ => Err($crate::LinkError::OdenLinkErrorUnknown),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_bind_device function");
                }
            }

            /// Sets the bind device for the given link `index`.
            /// `bind_device` must be a valid network device.
            ///
            /// Specify `entity` as [`None`] for Remote Streamer on Streamer-
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.set_link_bind_device(None, 0, "enp5s0").expect("Invalid Bind Device");
            /// # }
            /// ```
            pub fn set_link_bind_device(
                &self,
                entity: Option<&'_ str>,
                index: i32,
                bind_device: &str
            ) -> Result<(), $crate::LinkError> {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {

                let entity_str = if let Some(entity) = entity {
                    std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap()
                } else {
                    std::ffi::CString::new("").unwrap()
                };

                let entity_ptr = if entity.is_some() {
                    entity_str.as_ptr()
                } else {
                    std::ptr::null()
                };

                let  buffer = std::ffi::CString::new(bind_device.trim_end_matches('\0')).unwrap();
                let ptr = buffer.as_ptr();

                let mut param = $crate::plugin_h::OdenSceneParamLinkBindDevice {
                    type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeLinkBindDevice,
                    next: std::ptr::null_mut(),
                    entityId: entity_ptr,
                    index,
                    buffer: ptr as *mut std::os::raw::c_char,
                    buffer_lenght: std::ptr::null_mut(),
                    result: $crate::LinkError::OdenLinkErrorUnknown,
                };

                let res = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };

                match res {
                    $crate::SceneParamError::OdenSceneParamErrorOk => Ok(()),
                    _ => Err($crate::LinkError::OdenLinkErrorUnknown),
                }

                } else {
                    panic!("This version of Oden is too old to have the set_link_bind_device function");
                }
            }

            /// Diables hotkeys for the current frame.
            /// Needs to be called every frame.
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.inhibit_hotkeys();
            /// # }
            /// ```
            pub fn inhibit_hotkeys(&self) {
                if let Some(set_scene_param) = unsafe { (*self.inner).setSceneParam } {
                    let mut param = $crate::plugin_h::OdenSceneParamInhibitHotkeys {
                        type_: $crate::plugin_h::OdenSceneParamType::OdenSceneParamTypeInhibitHotkeys,
                        next: std::ptr::null_mut(),
                    };

                    let _ = unsafe { set_scene_param(&mut param as *mut _ as *mut std::os::raw::c_void) };
                } else {
                    panic!("This version of Oden is too old to have the inhibit_hotkeys function");

                }
            }

            /// Sets the mouse cursor of the operating system
            ///
            /// Example
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            ///     api.set_mouse_cursor(oden_plugin_rs::MouseCursorArrow);
            /// # }
            /// ```
            pub fn set_mouse_cursor(&self, cursor: $crate::MouseCursor) {
                if let Some(set_mouse_cursor) = unsafe { (*self.inner).setMouseCursor } {
                    unsafe { set_mouse_cursor(cursor) };
                } else {
                    panic!("This version of Oden is too old to have the set_mouse_cursor function");

                }
            }

            /// Adds a `subnet` to be excluded from Peer-to-Peer connections for a
            /// specified link
            ///
            /// The `subnet` must be in CIDR notation, e.g. 100.100.0.0/24
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let _ = api.add_link_p2p_excluded_subnet(None, 0, "100.100.0.0/24").unwrap();
            /// # }
            /// ```
            pub fn add_link_p2p_excluded_subnet(
                &self,
                entity: Option<&str>,
                index: i32,
                subnet: &str,
            ) -> Result<(), $crate::LinkError> {
                if let Some(add_link_p2p_excluded_subnet) = unsafe { (*self.inner).addLinkP2pExcludedSubnet } {
                    $crate::scene_api::set_link_string(
                        |entity: *const std::os::raw::c_char,
                        index: i32,
                        subnet: *const std::os::raw::c_char| {
                            unsafe { add_link_p2p_excluded_subnet(entity, index, subnet) }
                        },
                        entity,
                        index,
                        subnet,
                    )
                } else {
                    panic!("This version of Oden is too old to have the add_link_p2p_excluded_subnet function");
                }
            }

            /// Clears subnets excluded from Peer-to-Peer connections for a specified link
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SceneApi) {
            /// let _ = api.clear_link_p2p_excluded_subnets(None, 0).unwrap();
            /// # }
            /// ```
            pub fn clear_link_p2p_excluded_subnets(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<(), $crate::LinkError> {
                if let Some(clear_link_p2p_excluded_subnets) = unsafe{(*self.inner).clearLinkP2pExcludedSubnets} {
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();

                            unsafe { clear_link_p2p_excluded_subnets(entity.as_ptr(), index) }
                        }
                        None => unsafe { clear_link_p2p_excluded_subnets(std::ptr::null(), index) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(()),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the clear_link_p2p_excluded_subnets function");
                }
            }

            /// Returns the time in nanoseconds since the last packet was received
            /// on the given link. Useful for checking if a link is alive.
            pub fn link_time_since_last_received_packet_ns(
                &self,
                entity: Option<&str>,
                index: i32,
            ) -> Result<i64, $crate::LinkError> {
                if let Some(get_link_time_since_last_received_packet_ns) = unsafe { (*self.inner).getLinkTimeSinceLastReceivedPacketNs } {
                    let mut time_ns: i64 = 0;
                    let res = match entity {
                        Some(entity) => {
                            let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                            unsafe { get_link_time_since_last_received_packet_ns(entity.as_ptr(), index, &mut time_ns) }
                        }
                        None => unsafe { get_link_time_since_last_received_packet_ns(std::ptr::null(), index, &mut time_ns) },
                    };

                    match res {
                        $crate::LinkError::OdenLinkErrorOk => Ok(time_ns),
                        _ => Err(res),
                    }
                } else {
                    panic!("This version of Oden is too old to have the link_time_since_last_received_packet_ns function");
                }
            }
        }
    };
}

pub trait SceneApiExt {
    fn com_channel_send_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(
        &self,
        message_id: &str,
        data: &T,
    );
    fn com_channel_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(
        &self,
        message_id: &str,
        index: i32,
    ) -> Option<T>;
    fn com_channel_last_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(
        &self,
        message_id: &str,
    ) -> Option<T>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_scene_api_ext {
    ($interface:ident) => {
        // For proper docs links because we can't use $interface directly in doc comments
        #[allow(unused_imports)]
        use $interface as scene_api_ext_impl;

        #[inherent::inherent]
        impl $crate::scene_api::SceneApiExt for $interface<'_> {

            /// Sends a com channel message.
            pub fn com_channel_send_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(&self, message_id: &str, data: &T)
            {
                if let Some(com_channel_send_message) = unsafe { (*self.inner).comChannelSendMessage } {
                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();
                    let data = data.as_bytes();

                    unsafe {
                        com_channel_send_message(
                            message_id.as_ptr(),
                            data.as_ptr() as *mut u8,
                            data.len() as i32,
                        )
                    };
                } else {
                    panic!("This version of Oden is too old to have the com_channel_send_message function");
                }
            }

            /// Returns the com channel message at the specific `index` for the given `message_id`,
            /// or [`None`] if the `message_id` or `index` is incorrect.
            pub fn com_channel_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(&self, message_id: &str, index: i32) -> Option<T> {
                if let Some(com_channel_message) = unsafe { (*self.inner).comChannelGetMessage } {
                    let expected_data_size = std::mem::size_of::<T>() as i32;

                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();

                    let mut buf: Vec<u8> = vec![0; expected_data_size as usize];

                    let data: *mut std::ffi::c_void = buf.as_mut_ptr() as *mut std::ffi::c_void;
                    if unsafe {
                        com_channel_message(
                            message_id.as_ptr(),
                            index,
                            data as *mut _,
                            expected_data_size,
                        )
                    } {
                        let mut dest_message = T::default();
                        let dest = dest_message.as_bytes_mut();

                        let src = &buf[..dest.len()];

                        dest.copy_from_slice(src);

                        Some(dest_message)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the com_channel_message function");
                }
            }

            /// Returns the latest com channel message with the given `message_id`,
            /// or [`None`] if the message size is incorrect.
            ///
            /// (This function is made for fixed-size messages, so a difference in size is considered an error)
            pub fn com_channel_last_message<T: zerocopy::FromBytes + zerocopy::AsBytes + Default + 'static>(&self, message_id: &str) -> Option<T> {
                if let Some(oden_last_message) = unsafe { (*self.inner).comChannelGetLastMessage } {
                    let expected_data_size = std::mem::size_of::<T>() as i32;

                    let message_id = std::ffi::CString::new(message_id.trim_end_matches('\0')).unwrap();

                    let mut buf: Vec<u8> = vec![0; expected_data_size as usize];

                    let data: *mut std::ffi::c_void = buf.as_mut_ptr() as *mut std::ffi::c_void;
                    if unsafe { oden_last_message(message_id.as_ptr(), data as *mut _, expected_data_size) }
                    {
                        let mut dest_message = T::default();
                        let dest = dest_message.as_bytes_mut();

                        let src = &buf[..dest.len()];

                        dest.copy_from_slice(src);

                        Some(dest_message)
                    } else {
                        None
                    }
                } else {
                    panic!("This version of Oden is too old to have the com_channel_last_message function");
                }
            }
        }
    };
}
