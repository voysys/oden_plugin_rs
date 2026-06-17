//! Oden Rust Plugin Wrapper
//!
//! `oden_plugin_rs` is a Rust wrapper for the Oden C plugin API, enabling users
//! to write plugins in Rust that calls the C API.
//!
//! # Oden Plugin API Overview
//! Oden offers an API for creating plugins. Several functions are called by Oden
//! during the lifetime of a plugin. These functions, together with some meta data,
//! are registered using a macro.
//! Examples of what plugins can be used for:
//! - Visualize sensor data
//! - Support control surfaces
//! - Data analysis
//! - Autonomous functions (based on sensor data)
//! - Create custom 2D or 3D GUI:s
//! - Custom visualizations
//!
//! # DLL and .so
//! Plugins compile to `.dll` files on Windows and `.so` files on Linux. Oden will find
//! plugins by looking for `.dll`/`.so` files in the Oden installation directory
//! and in the project specific plugin directory.
//!
//! > **Note**: The UUID that identifies a plugin needs to be unique for each
//! > plugin. Generators for UUIDs can be found freely online.
//!
//! # Calling API methods
//! Each plugin callback receives a `*Params` value (for example
//! [`InitParams`], [`UpdateParams`] or [`GuiParams`]). The whole API is exposed
//! as methods you call directly on that value, e.g. `api.dt()` or
//! `api.im_draw_add_text(..)`.
//!
//! These methods are declared on traits such as [`UpdateApi`] and
//! [`SettingsApi`], but they are re-exposed as *inherent* methods on the
//! `*Params` structs. You therefore do **not** need to import the `*Api` traits
//! to call them — doing so only produces "unused import" warnings.
//!
//! # Plugin Structure
//!
//! Filename: hello_world.rs
//! ```
//! use oden_plugin_rs::{
//!     math::{Vec3, Vec4},
//!     register_plugin, GuiParams, InitParams, OdenPlugin, ShutdownParams, UpdateParams,
//! };
//!
//! struct State {
//!     _dummy: bool,
//! }
//!
//! impl OdenPlugin for State {
//!     fn init(_api: &InitParams) -> Self {
//!         State { _dummy: false }
//!     }
//!
//!     fn shutdown(self, api: &ShutdownParams) {}
//!
//!     fn update(&mut self, api: &UpdateParams) {
//!         api.im_draw_add_text(
//!             "Hello World!",
//!             Vec3::xyz(-1.0, 0.0, -2.0),
//!             Vec3::xyz(0.0, 0.3, 0.0),
//!             Vec3::xyz(1.0, 0.0, 0.0),
//!             Vec4::rgba(0.8, 0.8, 0.8, 1.0),
//!             0,
//!         );
//!     }
//!
//!     fn gui(&mut self, api: &GuiParams) {
//!         api.label("Hello World!");
//!     }
//! }
//!
//!
//! ```
//!
//! ## Init
//!
//! [`OdenPlugin::init`] is called when the plugin is initialized.
//! This is an appropriate place to create data for storing state and/or initializing threads.
//!
//! ## Update and GUI
//! The Oden Application runs a loop in which it calls [`OdenPlugin::update`] on all entities and then allows them
//! to draw a GUI, and then does the drawing to screen. The rate of this loop is
//! displayed in the top right corner (in Hz / fps).
//!
//! [`OdenPlugin::update`] is called once per Oden iteration for all non-hidden entities. This is the
//! proper place to do drawing or light recurring compute tasks.
//!
//! > **Note**: For heavier compute-tasks a separate thread should be spawned from [`OdenPlugin::init`].
//!
//! The [`OdenPlugin::gui`] function is called if the sidebar GUI of a plugin is visible. This is a
//! good place to inspect and alter variables and show debug information in an easy
//! way.
//!
//! ## Draw
//!
//! The function [`OdenPlugin::draw`] allows for using raw OpenGL commands.
//!
//! > **Note**: This is very complex and is not covered in the example plugin.
//!
//! ## Shutdown
//! The [`OdenPlugin::shutdown`] call comes when the plugin is closing down. Data allocated during
//! [`OdenPlugin::init`] must be freed here.
//!
//! [`OdenPlugin::update`]: crate::OdenPlugin::update()
//! [`OdenPlugin::init`]: crate::OdenPlugin::init()
//! [`OdenPlugin::gui`]: crate::OdenPlugin::gui()
//! [`OdenPlugin::shutdown`]: crate::OdenPlugin::shutdown()
//! [`OdenPlugin::draw`]: crate::OdenPlugin::draw()
//!
//! ## API Reference for OdenPlugin trait
//! [`InitParams`] contains the functions implemented for the init function. \
//! [`UpdateParams`] contains the functions implemented for the update function. \
//! [`GuiParams`] contains the functions implemented for the gui function. \
//! [`DrawParams`] contains the functions implemented for the draw function. \
//!
//! ## Functions In-parameters
//! There are some global functions available in the plugin API (accessed via the
//! `oden` global variable). The global API functions are safe to call from all
//! functions and they are also safe to call from other threads. All other API calls
//! are available on parameters passed into each of the functions that get called
//! from Oden (init, update, gui, shutdown, draw). It is safe to call all functions available in
//! the `api` parameter given in each function call. It is not safe to call functions
//! passed to one of the functions in any of the other functions, or to call them from
//! another thread.
//!
//! ## Data Management
//! All data needs to be managed and created in a way that allows for multiple
//! instances of each plugin to coexist.
//!
//! > **Important**: Do not use global or static variables. It will
//! > prevent multiple plugin instances and is not thread safe.
//!
//! ## Control and Names for Entities
//! Many functions in the plugin API use names to identify entities. When entities
//! are added to the scene graph in Oden they are default created without names.
//! To make these functions work, it is important to give them names.
//!
//! With all `entity` related functions, it is also possible to concatenate _@_
//! and a scene name to `entity` in order to get/set an entity in a specific scene, e.g.
//! _Remote Streamer@Input_ if the entity is called _Remote Streamer_ and the scene _Input_.
//! This is useful if the plugin is running in a different scene to the entity that you want.
//!
//! > **Note**: Entities are named using the “Name” option available when
//! > selecting the entity in the scene graph.
//!
//! # Immediate Mode: GUI and ImDraw
//! Oden uses a paradigm called “immediate mode” for GUI and drawing functions
//! (ImDraw). An immediate mode GUI is different from a traditional retained
//! mode GUI. In retained mode GUIs a widget is registered and callbacks or state
//! variables are set up that signals when things happen in the GUI. In contrast,
//! an immediate mode GUI is drawn in each frame and the actions happen in the
//! same code that draws the gui.
//!
//! ## Dear ImGui
//! The GUI framework that Oden uses for the sidebar GUI is called “Dear ImGui”.
//! Documentation is available for Dear ImGui online.
//!
//! ## Label Uniqueness
//! Dear ImGui uses canonical label strings to identify widgets. These need to be
//! unique.
//!
//! # Raw Recording
//!
//! A plugin may be used to provide support for something that provides or accepts
//! real time data. Writing a plugin allows visualizing sensor data in Oden,
//! and at the same time e.g. show video from cameras.
//! Oden provides a raw recording functionality to save exact
//! data coming into Oden for later playback.
//!
//! Raw recording is useful in many scenarios, e.g. algorithm development.
//! To support the same kind of functionality as Oden
//! support for raw recording, there are functions provide during update and also a
//! separate function call available for playback.
//!
//! During update, data can be sent to raw recording from the plugin. During
//! playback the recordingPlayback function will be called and will be passed data
//! that was sent from update when the system was live.
//!
//! # Hot Reloading
//!
//! During development it is important to have a quick iteration time from code change to running application. During the development of a plugin Oden
//! therefore offers hot reloading, so if the plugin `.dll`/`.so` is updated while Oden is
//! running then the plugin will be reloaded with the new code.
//!
//! Hot reloading resets the data state of the plugin (shutdown is called on the old
//! plugin and init is called on the new). In some cases this is not desired, and for
//! these cases Oden offers data migration functionality.
//!
//! > **Important Note**: Hot reloading is highly discouraged for production use and
//! > should only be used in development!

#![warn(missing_docs)]
// uncomment to check which functions does not have any documentation
// #![warn(missing_doc_code_examples)] // deny api if examples for all functions does not exist
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::derive_partial_eq_without_eq)]

#[rustfmt::skip]
#[doc(hidden)]
pub mod plugin_h;

#[doc(hidden)]
pub mod oden_log;

#[doc(hidden)]
pub mod utils;

pub mod async_com_channel_api;
pub mod block_transfer;
pub mod draw_api;
pub mod global;
pub mod graphics_api;
pub mod gui_api;
pub mod im_draw_api;
pub mod init_and_update_api;
pub mod input_api;
pub mod math;
pub mod message_bus;
pub mod named_mpmc_channel;
pub mod named_shared_data;
pub mod scene_api;
pub mod settings_api;
pub mod update_api;

#[cfg(feature = "webview_user_message")]
pub mod webview_user_message;

#[cfg(feature = "oden_egui")]
pub mod oden_egui;

mod draw_params;
mod events;
mod gui_params;
mod init_params;
mod layout_id;
mod neural_networks;
mod oden_plugin;
mod shutdown_params;
mod update_params;

#[doc(hidden)]
#[cfg(feature = "mock")]
pub mod mocks;

pub use block_transfer::{BlockTransferClient, BlockTransferServer};
pub use draw_api::DrawApi;
pub use draw_params::DrawParams;
pub use events::{
    CharEvent, FramebufferResizeEvent, KeyEvent, MouseButtonEvent, ScrollEvent, TouchInputAction,
    TouchInputEvent, WindowEvent, WindowFocusEvent, WindowPosEvent, WindowResizeEvent,
};
pub use graphics_api::{GraphicsApi, InitAndGraphicsApi};
pub use gui_api::GuiApi;
pub use gui_params::GuiParams;
pub use im_draw_api::ImDrawApi;
pub use init_and_update_api::{FontDescriptor, InitAndUpdateApi};
pub use init_params::InitParams;
pub use message_bus::{BusClient, BusOwner, OwnedMessage, RecvFuture, SendError, Subscription};
pub use neural_networks::{InferenceSettings, NeuralNetworkInfo, TensorData, TensorInput};
pub use oden_plugin::{
    OdenPlugin, OdenPluginHasNoProperties, OdenPluginMockable, OdenPluginWithProperties,
    PluginStateWrapper, RegisterType,
};
pub use plugin_h::{
    OdenApplicationType as ApplicationType,
    OdenAsyncComChannelMessageId as AsyncComChannelMessageId,
    OdenAsyncComChannelReceiveError as AsyncComChannelReceiveError,
    OdenAsyncComChannelVarSizeMsgId as AsyncComChannelVarSizeMsgId,
    OdenCameraCalibration_s as CameraCalibration, OdenCameraCropData as CameraCropData,
    OdenCameraMetadata_s as CameraMetadata,
    OdenCameraMetadata_s__bindgen_ty_1 as CameraMetadataInner,
    OdenCameraStreamState_e as CameraStreamState,
    OdenCameraStreamState_e_OdenCameraStreamStatePaused as CameraStreamStatePaused,
    OdenCameraStreamState_e_OdenCameraStreamStatePlaying as CameraStreamStatePlaying,
    OdenCameraStreamState_e_OdenCameraStreamStateStopped as CameraStreamStateStopped,
    OdenCameraStreamState_e_OdenCameraStreamStateUnknown as CameraStreamStateUnknown,
    OdenColliderType_s_OdenColliderTypeBox as ColliderTypeBox,
    OdenColliderType_s_OdenColliderTypeCylinder as ColliderTypeCylinder,
    OdenDecodedFrame_s as DecodedFrame, OdenEventType_e_OdenEventTypeChar as EventTypeChar,
    OdenEventType_e_OdenEventTypeFramebufferResize as EventTypeFramebufferResize,
    OdenEventType_e_OdenEventTypeKey as EventTypeKey,
    OdenEventType_e_OdenEventTypeMouseButton as EventTypeMouseButton,
    OdenEventType_e_OdenEventTypeScroll as EventTypeScroll,
    OdenEventType_e_OdenEventTypeShutdown as EventTypeShutdown,
    OdenEventType_e_OdenEventTypeTouchInput as EventTypeTouchInput,
    OdenEventType_e_OdenEventTypeWindowFocus as EventTypeWindowFocus,
    OdenEventType_e_OdenEventTypeWindowPos as EventTypeWindowPos,
    OdenEventType_e_OdenEventTypeWindowResize as EventTypeWindowResize,
    OdenFpsCameraType_e_OdenFpsCameraTypeFps as FpsCameraTypeFps,
    OdenFpsCameraType_e_OdenFpsCameraTypeOrbit as FpsCameraTypeOrbit,
    OdenGamepadStateV2_s as GamepadState, OdenGlLoaderFunc as GlLoader,
    OdenLinkError_e as LinkError, OdenLinkMode as LinkMode,
    OdenLinkP2PStatusWrapper_s as LinkP2PStatusWrapper,
    OdenLinkStatsInterfaceStatistics as LinkStatsInterfaceStatistics,
    OdenLogLevel_e_OdenLogCritical as LogCritical, OdenLogLevel_e_OdenLogDebug as LogDebug,
    OdenLogLevel_e_OdenLogError as LogError, OdenLogLevel_e_OdenLogInfo as LogInfo,
    OdenLogLevel_e_OdenLogNone as LogNone, OdenLogLevel_e_OdenLogTrace as LogTrace,
    OdenLogLevel_e_OdenLogWarning as LogWarning, OdenNeuralNetworkError_e as NeuralNetworkError,
    OdenPluginFontRangeEntry_s as FontRangeEntry, OdenPluginImDrawImage_s as ImDrawImage,
    OdenPluginImDrawTextAlignment as ImDrawTextAlignment,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentCentered as ImDrawTextAlignmentCentered,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentCentered as PluginImDrawTextAlignmentCentered,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentLeft as ImDrawTextAlignmentLeft,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentLeft as PluginImDrawTextAlignmentLeft,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentRight as ImDrawTextAlignmentRight,
    OdenPluginImDrawTextAlignment_e_OdenPluginImDrawTextAlignmentRight as PluginImDrawTextAlignmentRight,
    OdenPluginMouseCursor_e as MouseCursor,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorArrow as MouseCursorArrow,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorPointingHand as MouseCursorPointingHand,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorResizeEW as MouseCursorResizeEW,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorResizeNESW as MouseCursorResizeNESW,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorResizeNS as MouseCursorResizeNS,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorResizeNWSE as MouseCursorResizeNWSE,
    OdenPluginMouseCursor_e_OdenPluginMouseCursorTextInput as MouseCursorTextInput,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatArgb as VideoFormatArgb,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatBgr as VideoFormatBgr,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatBgra as VideoFormatBgra,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatInvalid as VideoFormatInvalid,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatMono16 as VideoFormatMono16,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatMono8 as VideoFormatMono8,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatNv12 as VideoFormatNv12,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatRaw as VideoFormatRaw,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatRaw10 as VideoFormatRaw10,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatRaw16 as VideoFormatRaw16,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatRgb as VideoFormatRgb,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatRgba as VideoFormatRgba,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatUyvy as VideoFormatUyvy,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatUyyvyy as VideoFormatUyyvyy,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatV210 as VideoFormatV210,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatYuv as VideoFormatYuv,
    OdenPluginVideoFormat_e_OdenPluginVideoFormatYuyv as VideoFormatYuyv,
    OdenPluginVideoFrame as VideoFrame, OdenQueryDataMetadata_s as QueryMetadata,
    OdenQueryDataResult_e_OdenQueryDataResultDataIdNotFound as QueryDataResultDataIdNotFound,
    OdenQueryDataResult_e_OdenQueryDataResultDataIsNotNotExpectedSize as QueryDataResultDataIsNotNotExpectedSize,
    OdenQueryDataResult_e_OdenQueryDataResultInvalidParameters as QueryDataResultInvalidParameters,
    OdenQueryDataResult_e_OdenQueryDataResultOk as QueryDataResultOk,
    OdenRegulatorMode as RegulatorMode, OdenSceneParamError_e as SceneParamError,
    OdenStreamStatisticsEx as StreamStatisticsEx, OdenStreamStatistics_s as StreamStatistics,
    OdenTextureFormat_e_OdenTextureFormatBGRA as TextureFormatBGRA,
    OdenTextureFormat_e_OdenTextureFormatRGB as TextureFormatRGB,
    OdenTextureFormat_e_OdenTextureFormatRGBA as TextureFormatRGBA,
    OdenTouchInputAction_e_Move as TouchInputActionMove,
    OdenTouchInputAction_e_Press as TouchInputActionPress,
    OdenTouchInputAction_e_Release as TouchInputActionRelease,
    OdenUploadImageParams as UploadImageParams, OdenUuid, OdenViewportInfo_s as ViewportInfo,
    OdenWindowMode_s as WindowMode, ODEN_FAULT_HMD_DISCONNECTED, ODEN_FAULT_HMD_REMOVED_FROM_HEAD,
    ODEN_FAULT_OCULUS_DISCONNECTED, PLUGIN_JOYSTICK_1 as JOYSTICK_1,
    PLUGIN_JOYSTICK_10 as JOYSTICK_10, PLUGIN_JOYSTICK_11 as JOYSTICK_11,
    PLUGIN_JOYSTICK_12 as JOYSTICK_12, PLUGIN_JOYSTICK_13 as JOYSTICK_13,
    PLUGIN_JOYSTICK_14 as JOYSTICK_14, PLUGIN_JOYSTICK_15 as JOYSTICK_15,
    PLUGIN_JOYSTICK_16 as JOYSTICK_16, PLUGIN_JOYSTICK_2 as JOYSTICK_2,
    PLUGIN_JOYSTICK_3 as JOYSTICK_3, PLUGIN_JOYSTICK_4 as JOYSTICK_4,
    PLUGIN_JOYSTICK_5 as JOYSTICK_5, PLUGIN_JOYSTICK_6 as JOYSTICK_6,
    PLUGIN_JOYSTICK_7 as JOYSTICK_7, PLUGIN_JOYSTICK_8 as JOYSTICK_8,
    PLUGIN_JOYSTICK_9 as JOYSTICK_9, PLUGIN_JOYSTICK_LAST as JOYSTICK_LAST,
    PLUGIN_JOYSTICK_MAX_AXES as JOYSTICK_MAX_AXES, PLUGIN_KEY_0 as KEY_0, PLUGIN_KEY_1 as KEY_1,
    PLUGIN_KEY_2 as KEY_2, PLUGIN_KEY_3 as KEY_3, PLUGIN_KEY_4 as KEY_4, PLUGIN_KEY_5 as KEY_5,
    PLUGIN_KEY_6 as KEY_6, PLUGIN_KEY_7 as KEY_7, PLUGIN_KEY_8 as KEY_8, PLUGIN_KEY_9 as KEY_9,
    PLUGIN_KEY_A as KEY_A, PLUGIN_KEY_ACTION_PRESS as KEY_ACTION_PRESS,
    PLUGIN_KEY_ACTION_RELEASE as KEY_ACTION_RELEASE, PLUGIN_KEY_ACTION_REPEAT as KEY_ACTION_REPEAT,
    PLUGIN_KEY_APOSTROPHE as KEY_APOSTROPHE, PLUGIN_KEY_B as KEY_B,
    PLUGIN_KEY_BACKSLASH as KEY_BACKSLASH, PLUGIN_KEY_BACKSPACE as KEY_BACKSPACE,
    PLUGIN_KEY_C as KEY_C, PLUGIN_KEY_CAPS_LOCK as KEY_CAPS_LOCK, PLUGIN_KEY_COMMA as KEY_COMMA,
    PLUGIN_KEY_D as KEY_D, PLUGIN_KEY_DELETE as KEY_DELETE, PLUGIN_KEY_DOWN as KEY_DOWN,
    PLUGIN_KEY_E as KEY_E, PLUGIN_KEY_END as KEY_END, PLUGIN_KEY_ENTER as KEY_ENTER,
    PLUGIN_KEY_EQUAL as KEY_EQUAL, PLUGIN_KEY_ESCAPE as KEY_ESCAPE, PLUGIN_KEY_F as KEY_F,
    PLUGIN_KEY_F1 as KEY_F1, PLUGIN_KEY_F10 as KEY_F10, PLUGIN_KEY_F11 as KEY_F11,
    PLUGIN_KEY_F12 as KEY_F12, PLUGIN_KEY_F13 as KEY_F13, PLUGIN_KEY_F14 as KEY_F14,
    PLUGIN_KEY_F15 as KEY_F15, PLUGIN_KEY_F16 as KEY_F16, PLUGIN_KEY_F17 as KEY_F17,
    PLUGIN_KEY_F18 as KEY_F18, PLUGIN_KEY_F19 as KEY_F19, PLUGIN_KEY_F2 as KEY_F2,
    PLUGIN_KEY_F20 as KEY_F20, PLUGIN_KEY_F21 as KEY_F21, PLUGIN_KEY_F22 as KEY_F22,
    PLUGIN_KEY_F23 as KEY_F23, PLUGIN_KEY_F24 as KEY_F24, PLUGIN_KEY_F25 as KEY_F25,
    PLUGIN_KEY_F3 as KEY_F3, PLUGIN_KEY_F4 as KEY_F4, PLUGIN_KEY_F5 as KEY_F5,
    PLUGIN_KEY_F6 as KEY_F6, PLUGIN_KEY_F7 as KEY_F7, PLUGIN_KEY_F8 as KEY_F8,
    PLUGIN_KEY_F9 as KEY_F9, PLUGIN_KEY_G as KEY_G, PLUGIN_KEY_GRAVE_ACCENT as KEY_GRAVE_ACCENT,
    PLUGIN_KEY_H as KEY_H, PLUGIN_KEY_HOME as KEY_HOME, PLUGIN_KEY_I as KEY_I,
    PLUGIN_KEY_INSERT as KEY_INSERT, PLUGIN_KEY_J as KEY_J, PLUGIN_KEY_K as KEY_K,
    PLUGIN_KEY_KP_0 as KEY_KP_0, PLUGIN_KEY_KP_1 as KEY_KP_1, PLUGIN_KEY_KP_2 as KEY_KP_2,
    PLUGIN_KEY_KP_3 as KEY_KP_3, PLUGIN_KEY_KP_4 as KEY_KP_4, PLUGIN_KEY_KP_5 as KEY_KP_5,
    PLUGIN_KEY_KP_6 as KEY_KP_6, PLUGIN_KEY_KP_7 as KEY_KP_7, PLUGIN_KEY_KP_8 as KEY_KP_8,
    PLUGIN_KEY_KP_9 as KEY_KP_9, PLUGIN_KEY_KP_ADD as KEY_KP_ADD,
    PLUGIN_KEY_KP_DECIMAL as KEY_KP_DECIMAL, PLUGIN_KEY_KP_DIVIDE as KEY_KP_DIVIDE,
    PLUGIN_KEY_KP_ENTER as KEY_KP_ENTER, PLUGIN_KEY_KP_EQUAL as KEY_KP_EQUAL,
    PLUGIN_KEY_KP_MULTIPLY as KEY_KP_MULTIPLY, PLUGIN_KEY_KP_SUBTRACT as KEY_KP_SUBTRACT,
    PLUGIN_KEY_L as KEY_L, PLUGIN_KEY_LAST as KEY_LAST, PLUGIN_KEY_LEFT as KEY_LEFT,
    PLUGIN_KEY_LEFT_ALT as KEY_LEFT_ALT, PLUGIN_KEY_LEFT_BRACKET as KEY_LEFT_BRACKET,
    PLUGIN_KEY_LEFT_CONTROL as KEY_LEFT_CONTROL, PLUGIN_KEY_LEFT_SHIFT as KEY_LEFT_SHIFT,
    PLUGIN_KEY_LEFT_SUPER as KEY_LEFT_SUPER, PLUGIN_KEY_M as KEY_M, PLUGIN_KEY_MENU as KEY_MENU,
    PLUGIN_KEY_MINUS as KEY_MINUS, PLUGIN_KEY_N as KEY_N, PLUGIN_KEY_NUM_LOCK as KEY_NUM_LOCK,
    PLUGIN_KEY_O as KEY_O, PLUGIN_KEY_P as KEY_P, PLUGIN_KEY_PAGE_DOWN as KEY_PAGE_DOWN,
    PLUGIN_KEY_PAGE_UP as KEY_PAGE_UP, PLUGIN_KEY_PAUSE as KEY_PAUSE,
    PLUGIN_KEY_PERIOD as KEY_PERIOD, PLUGIN_KEY_PRINT_SCREEN as KEY_PRINT_SCREEN,
    PLUGIN_KEY_Q as KEY_Q, PLUGIN_KEY_R as KEY_R, PLUGIN_KEY_RIGHT as KEY_RIGHT,
    PLUGIN_KEY_RIGHT_ALT as KEY_RIGHT_ALT, PLUGIN_KEY_RIGHT_BRACKET as KEY_RIGHT_BRACKET,
    PLUGIN_KEY_RIGHT_CONTROL as KEY_RIGHT_CONTROL, PLUGIN_KEY_RIGHT_SHIFT as KEY_RIGHT_SHIFT,
    PLUGIN_KEY_RIGHT_SUPER as KEY_RIGHT_SUPER, PLUGIN_KEY_S as KEY_S,
    PLUGIN_KEY_SCROLL_LOCK as KEY_SCROLL_LOCK, PLUGIN_KEY_SEMICOLON as KEY_SEMICOLON,
    PLUGIN_KEY_SLASH as KEY_SLASH, PLUGIN_KEY_SPACE as KEY_SPACE, PLUGIN_KEY_T as KEY_T,
    PLUGIN_KEY_TAB as KEY_TAB, PLUGIN_KEY_U as KEY_U, PLUGIN_KEY_UNKNOWN as KEY_UNKNOWN,
    PLUGIN_KEY_UP as KEY_UP, PLUGIN_KEY_V as KEY_V, PLUGIN_KEY_W as KEY_W,
    PLUGIN_KEY_WORLD_1 as KEY_WORLD_1, PLUGIN_KEY_WORLD_2 as KEY_WORLD_2, PLUGIN_KEY_X as KEY_X,
    PLUGIN_KEY_Y as KEY_Y, PLUGIN_KEY_Z as KEY_Z, PLUGIN_MOD_ALT as MOD_ALT,
    PLUGIN_MOD_CONTROL as MOD_CONTROL, PLUGIN_MOD_SHIFT as MOD_SHIFT,
    PLUGIN_MOD_SUPER as MOD_SUPER, PLUGIN_MOUSE_BUTTON_1 as MOUSE_BUTTON_1,
    PLUGIN_MOUSE_BUTTON_2 as MOUSE_BUTTON_2, PLUGIN_MOUSE_BUTTON_3 as MOUSE_BUTTON_3,
    PLUGIN_MOUSE_BUTTON_4 as MOUSE_BUTTON_4, PLUGIN_MOUSE_BUTTON_5 as MOUSE_BUTTON_5,
    PLUGIN_MOUSE_BUTTON_6 as MOUSE_BUTTON_6, PLUGIN_MOUSE_BUTTON_7 as MOUSE_BUTTON_7,
    PLUGIN_MOUSE_BUTTON_8 as MOUSE_BUTTON_8, PLUGIN_MOUSE_BUTTON_LAST as MOUSE_BUTTON_LAST,
    PLUGIN_MOUSE_BUTTON_LEFT as MOUSE_BUTTON_LEFT,
    PLUGIN_MOUSE_BUTTON_MIDDLE as MOUSE_BUTTON_MIDDLE,
    PLUGIN_MOUSE_BUTTON_RIGHT as MOUSE_BUTTON_RIGHT,
    PLUGIN_TENSOR_SHAPE_MAX_DIMS as ODEN_TENSOR_SHAPE_MAX_DIMS,
};
#[deprecated]
pub use plugin_h::{PLUGIN_MOD_ALT, PLUGIN_MOD_CONTROL, PLUGIN_MOD_SHIFT, PLUGIN_MOD_SUPER};

pub use scene_api::{
    CalibrationQuality, GamepadAxis, GamepadButton, JoystickState, KeyModifiers,
    PerformanceMetrics, PlaybackTimes, QueryError, SceneApi, SceneApiExt,
};
pub use settings_api::SettingsApi;
#[cfg(feature = "serialize")]
pub use settings_api::SettingsExtApi;
pub use shutdown_params::ShutdownParams;
pub use update_api::UpdateApi;
pub use update_params::UpdateParams;

pub use log;
pub use zerocopy;

#[cfg(feature = "glam_conversion")]
pub use glam;

#[cfg(feature = "nalgebra_conversion")]
pub use nalgebra;

#[cfg(feature = "oden_egui")]
pub use egui;

#[doc(hidden)]
pub use paste;

#[doc(hidden)]
pub use backtrace;

#[doc(hidden)]
pub static mut ODEN_GLOBAL: *mut plugin_h::OdenPluginGlobalFunctions = std::ptr::null_mut();

#[cfg(feature = "mock")]
pub use mocks::*;

#[cfg(feature = "glam_conversion")]
pub use utils::{reproject_point_to_camera_coords, ReprojectionResult};

#[cfg(feature = "strum_ex")]
pub use strum;
