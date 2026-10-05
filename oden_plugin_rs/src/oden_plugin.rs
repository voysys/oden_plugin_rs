use crate::{
    draw_params::DrawParams, DrawApi, GraphicsApi, GuiApi, GuiParams, ImDrawApi,
    InitAndGraphicsApi, InitAndUpdateApi, InitParams, SceneApi, SettingsApi, ShutdownParams,
    UpdateApi, UpdateParams,
};
use std::sync::mpsc::{self, Receiver, SyncSender};

#[doc(hidden)]
pub trait OdenPluginMockable {
    fn init(api: &(impl SettingsApi + InitAndUpdateApi + InitAndGraphicsApi)) -> Self;
    fn shutdown(self, api: &impl SettingsApi);
    fn update(
        &mut self,
        api: &(impl SettingsApi
              + SceneApi
              + InitAndUpdateApi
              + GraphicsApi
              + InitAndGraphicsApi
              + UpdateApi
              + ImDrawApi),
    );
    fn draw(
        &mut self,
        _api: &(impl SettingsApi + SceneApi + GraphicsApi + InitAndGraphicsApi + DrawApi),
    ) {
    }
    fn gui(&mut self, api: &(impl SettingsApi + SceneApi + GuiApi));
    fn init_logger() -> bool {
        true
    }
}

impl<T: OdenPluginMockable> OdenPlugin for T {
    fn init(api: &InitParams) -> Self {
        <Self as OdenPluginMockable>::init(api)
    }

    fn shutdown(self, api: &ShutdownParams) {
        <Self as OdenPluginMockable>::shutdown(self, api);
    }

    fn update(&mut self, api: &UpdateParams) {
        <Self as OdenPluginMockable>::update(self, api);
    }

    fn draw(&mut self, api: &DrawParams) {
        <Self as OdenPluginMockable>::draw(self, api);
    }

    fn gui(&mut self, api: &GuiParams) {
        <Self as OdenPluginMockable>::gui(self, api);
    }

    fn init_logger() -> bool {
        <Self as OdenPluginMockable>::init_logger()
    }
}

/// # OdenPlugin Trait
/// Implement this trait for a type in order to register that type as a plugin entity type.
pub trait OdenPlugin {
    /// Initializes a new instance of the plugin.
    fn init(api: &InitParams) -> Self;

    /// Destructor for cleaning up resources when plugin instance is unloaded.
    fn shutdown(self, api: &ShutdownParams);

    /// Called on each frame update during the plugin instances lifetime.
    fn update(&mut self, api: &UpdateParams);

    /// Called for each surface that oden want the plugin to draw to
    fn draw(&mut self, _api: &DrawParams) {}

    /// Called on each frame when the Oden config GUI is displayed. Function is responsible
    /// for displaying plugin configuration elements.
    fn gui(&mut self, api: &GuiParams);

    /// Called to determine if the plugin wants the oden logger to be initialized
    fn init_logger() -> bool {
        true
    }
}

#[doc(hidden)]
pub trait OdenPluginWithProperties {
    fn load_properties(&mut self, api: &InitParams);
    fn save_properties(&mut self, api: &UpdateParams);
    fn gui_properties(&mut self, api: &GuiParams);
}

#[doc(hidden)]
pub trait OdenPluginHasNoProperties {
    fn as_oden_plugin_with_properties(&mut self) -> Option<&mut dyn OdenPluginWithProperties>;
}

impl<T> OdenPluginHasNoProperties for T {
    fn as_oden_plugin_with_properties(&mut self) -> Option<&mut dyn OdenPluginWithProperties> {
        None
    }
}

#[doc(hidden)]
pub struct PluginStateWrapper<T> {
    pub plugin_name: String,
    pub playback_receiver: Receiver<Vec<u8>>,
    pub has_panicked: bool,
    pub state: Option<T>,

    // Accessed from the recording playback thread
    pub playback_sender: SyncSender<Vec<u8>>,
}

impl<T> PluginStateWrapper<T>
where
    T: OdenPlugin,
{
    pub fn init(plugin_name: &str, api: &InitParams) -> Self {
        let (send, recv) = mpsc::sync_channel(128);

        let state = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| T::init(api)));
        if state.is_err() {
            api.set_has_crashed();
            log::error!("Panic occurred in {plugin_name} Init");
        }

        Self {
            plugin_name: plugin_name.to_owned(),
            playback_receiver: recv,
            has_panicked: state.is_err(),
            state: state.ok(),
            playback_sender: send,
        }
    }
}

/// Used by the ´register!´ macro to determine how to register the plugin with Oden
pub enum RegisterType {
    /// Used to register an entity type that can be added to a scene
    Entity,
    /// Used to register a global entity instance that will be created when the plugin loads
    GlobalInstance,
}

/// This macro emits the necessary code for Oden to use the code as a plugin.
///
/// This macro must be called at the end of every plugin
/// # Examples
/// ```ignore
/// register!(
///     {
///         "Global Instance",
///         "1076c1b8-2599-4ccf-84be-e10e75ff6953",
///         State1,
///         RegisterType::GlobalInstance,
///     },
///     {
///         "Some Entity",
///         "56f16145-765b-4229-a09a-287e9f7dacba",
///         State2,
///         RegisterType::Entity,
///     },
/// );
/// ```
#[macro_export]
macro_rules! register {
    {
        $({ $name:expr, $uuid:expr, $state_type:ident, $register_type:expr $(,)? } $(,)? )+
    } => {
        $crate::paste::paste! {
            $(
                mod [< oden_plugin_internal_ $state_type:lower >] {

                    use $crate::log::info;
                    use $crate::OdenPlugin;
                    use $crate::OdenPluginHasNoProperties;

                    pub extern "C" fn init(api: *mut $crate::plugin_h::OdenPluginEntityInitParams) {
                        let mut wrapper: $crate::PluginStateWrapper<super::$state_type> =
                            $crate::PluginStateWrapper::init(
                                $name,
                                &$crate::InitParams {
                                    inner: api,
                                    phantom: std::marker::PhantomData,
                                },
                            );

                        if let Some(state) = wrapper.state.as_mut() {
                            if let Some(has_properties) = state.as_oden_plugin_with_properties() {
                                $crate::OdenPluginWithProperties::load_properties(
                                    has_properties,
                                    &$crate::InitParams {
                                        inner: api,
                                        phantom: std::marker::PhantomData,
                                    },
                                );
                            }
                        }

                        unsafe {
                            (*api).entityUserData =
                                Box::into_raw(Box::new(wrapper)) as *mut std::ffi::c_void;
                        }
                    }

                    pub extern "C" fn shutdown(api: *mut $crate::plugin_h::OdenPluginEntityShutdownParams) {
                        let mut wrapper = unsafe {
                            Box::from_raw(
                                (*api).entityUserData
                                    as *mut $crate::PluginStateWrapper<super::$state_type>,
                            )
                        };

                        let shutdown_params = $crate::ShutdownParams {
                            inner: api,
                            phantom: std::marker::PhantomData,
                        };

                        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            if let Some(state) = wrapper.state.take() {
                                state.shutdown(&shutdown_params)
                            }
                        }))
                        .is_err()
                        {
                            wrapper.has_panicked = true;
                            shutdown_params.set_has_crashed();
                            $crate::log::error!("Panic occurred in {} Shutdown", wrapper.plugin_name);
                        }
                    }

                    pub extern "C" fn update(api: *mut $crate::plugin_h::OdenPluginEntityUpdateParams) {
                        let mut wrapper = unsafe {
                            &mut *((*api).entityUserData
                                as *mut $crate::PluginStateWrapper<super::$state_type>)
                        };

                        let update_params = $crate::UpdateParams {
                            inner: api,
                            recording_playback_channel: &wrapper.playback_receiver,
                        };

                        if !wrapper.has_panicked {
                            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                if let Some(state) = wrapper.state.as_mut() {
                                    state.update(&update_params)
                                }
                            }))
                            .is_err()
                            {
                                wrapper.has_panicked = true;
                                update_params.set_has_crashed();
                                $crate::log::error!("Panic occurred in {} Update", wrapper.plugin_name);
                            }
                        }

                        if let Some(state) = wrapper.state.as_mut() {
                            if let Some(has_properties) = state.as_oden_plugin_with_properties() {
                                $crate::OdenPluginWithProperties::save_properties(
                                    has_properties,
                                    &update_params,
                                );
                            }
                        }
                    }

                    pub extern "C" fn draw(api: *mut $crate::plugin_h::OdenPluginEntityDrawParams) {
                        let mut wrapper = unsafe {
                            &mut *((*api).entityUserData
                                as *mut $crate::PluginStateWrapper<super::$state_type>)
                        };

                        let draw_params = $crate::DrawParams {
                            inner: api,
                            recording_playback_channel: &wrapper.playback_receiver,
                        };

                        if !wrapper.has_panicked {
                            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                if let Some(state) = wrapper.state.as_mut() {
                                    state.draw(&draw_params)
                                }
                            }))
                            .is_err()
                            {
                                wrapper.has_panicked = true;
                                draw_params.set_has_crashed();
                                $crate::log::error!("Panic occurred in {} Draw", wrapper.plugin_name);
                            }
                        }
                    }

                    pub extern "C" fn gui(api: *mut $crate::plugin_h::OdenPluginEntityGuiParams) {
                        let mut wrapper = unsafe {
                            &mut *((*api).entityUserData
                                as *mut $crate::PluginStateWrapper<super::$state_type>)
                        };

                        if let Some(state) = wrapper.state.as_mut() {
                            if let Some(has_properties) = state.as_oden_plugin_with_properties() {
                                $crate::OdenPluginWithProperties::gui_properties(
                                    has_properties,
                                    &$crate::GuiParams {
                                        inner: api,
                                        phantom: std::marker::PhantomData,
                                    },
                                );
                            }
                        }

                        let gui_params = $crate::GuiParams {
                            inner: api,
                            phantom: std::marker::PhantomData,
                        };

                        if !wrapper.has_panicked {
                            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                if let Some(state) = wrapper.state.as_mut() {
                                    state.gui(&gui_params)
                                }
                            }))
                            .is_err()
                            {
                                wrapper.has_panicked = true;
                                gui_params.set_has_crashed();
                                $crate::log::error!("Panic occurred in {} Gui", wrapper.plugin_name);
                            }
                        }
                    }

                    pub extern "C" fn recording_playback(
                        api: *mut $crate::plugin_h::OdenPluginEntityDataRecordingPlaybackParams,
                    ) {
                        let wrapper_ptr = unsafe {
                            ((*api).entityUserDataThreadUnsafe
                                as *const $crate::PluginStateWrapper<super::$state_type>)
                        };

                        let sender = unsafe { &(*wrapper_ptr).playback_sender };

                        let data =
                            unsafe { std::slice::from_raw_parts((*api).data, (*api).dataSize as usize) };

                        sender.try_send(data.to_owned()).ok();
                    }
                }
            )+
        }

        /// # Safety
        /// Function is called from Oden to get plugin version.
        #[no_mangle]
        pub extern "C" fn oden_plugin_version() -> i32 {
            return $crate::plugin_h::ODEN_PLUGIN_VERSION as i32;
        }

        /// # Safety
        /// Function is called from Oden on plugin registration.
        #[no_mangle]
        pub unsafe extern "C" fn oden_plugin_register(
            api: *mut $crate::plugin_h::OdenPluginRegistrationParams,
            global: *mut $crate::plugin_h::OdenPluginGlobalFunctions,
        ) {
            #![allow(clippy::string_lit_as_bytes)]
            use std::ffi::CString;
            use $crate::OdenPlugin;

            std::panic::set_hook(Box::new(|panic_info| {
                let backtrace = $crate::backtrace::Backtrace::new();
                $crate::log::error!("{}", panic_info);
                $crate::log::error!("backtrace {:?}", backtrace);
            }));

            unsafe {
                $crate::ODEN_GLOBAL = global;
            }

            let mut init_logger = true;

            $crate::paste::paste! {
                $(
                    {
                        init_logger &= $state_type::init_logger();
                    }
                )+
            }

            if init_logger {
                $crate::oden_log::init().expect("Failed to initialize logger, this is likely because the plugin was loaded twice. Check your plugin load paths.");
            }

            $crate::paste::paste! {
                $(
                    {
                        let mut entity_registration: $crate::plugin_h::OdenPluginEntityRegistration =
                            unsafe { std::mem::zeroed() };

                        {
                            let c_uuid: &[u8] = $uuid.as_bytes();
                            entity_registration.entityId[..c_uuid.len()].copy_from_slice(unsafe {
                                &*(&c_uuid[..] as *const _ as *const [std::os::raw::c_char])
                            })
                        }

                        {
                            let c_name: &[u8] = $name.as_bytes();
                            let len = c_name.len().min(entity_registration.entityName.len() - 1);
                            entity_registration.entityName[..len].copy_from_slice(unsafe {
                                &*(&c_name[..len] as *const _ as *const [std::os::raw::c_char])
                            })
                        }

                        entity_registration.init = Some([< oden_plugin_internal_ $state_type:lower >]::init);
                        entity_registration.shutdown = Some([< oden_plugin_internal_ $state_type:lower >]::shutdown);
                        entity_registration.update = Some([< oden_plugin_internal_ $state_type:lower >]::update);
                        entity_registration.draw = Some([< oden_plugin_internal_ $state_type:lower >]::draw);
                        entity_registration.gui = Some([< oden_plugin_internal_ $state_type:lower >]::gui);
                        entity_registration.recordingPlayback = Some([< oden_plugin_internal_ $state_type:lower >]::recording_playback);

                        init_logger &= $state_type::init_logger();

                        match $register_type {
                            $crate::RegisterType::Entity => {
                                unsafe {
                                    if let Some(register_entity) = (*api).registerEntity {
                                        register_entity(&mut entity_registration as *mut _);
                                    }
                                }
                            },
                            $crate::RegisterType::GlobalInstance => {
                                unsafe {
                                    if let Some(instantiate_global_entity) = (*api).instantiateGlobalEntity {
                                        instantiate_global_entity(&mut entity_registration as *mut _);
                                    }
                                }
                            }
                        }
                    }
                )+
            }
        }
    };
}

/// This macro emits the necessary code for Oden to use the code as a plugin.
///
/// This macro must be called at the end of every plugin with the field plugin name, uuid and state as shown in the example below.
/// # Examples
/// ```ignore
/// register_plugin!(
///     "Hello World from Rust",
///     "185d4603-e803-4fa2-9f56-266782cf2e98",
///     State
/// );
/// ```
#[macro_export]
macro_rules! register_plugin {
    ($name:expr, $uuid:expr, $state_type:ident) => {
        $crate::register! {
            { $name, $uuid, $state_type, $crate::RegisterType::Entity, }
        }
    };
}

/// This macro emits the necessary code for Oden to use the code as a global plugin.
///
/// Global plugins are different from regular plugins as global plugins are not added to the scene graph.
/// All global plugins that Oden can find will be running regardless of which project that is loaded.
/// For example, a global plugin can be use to change project depending on an external signal.
///
/// Global plugins are registered in the same way as regular plugins except with the `instantiate_global_entity`
/// macro instead as shown in the example below.
///
/// # Examples
/// ```ignore
/// instantiate_global_entity!(
///     "Hello World from Rust",
///     "312fe6ca-9ada-4224-8320-7554aa21864f",
///     State
/// );
/// ```
#[macro_export]
macro_rules! instantiate_global_entity {
    ($name:expr, $uuid:expr, $state_type:ident) => {
        $crate::register! {
            { $name, $uuid, $state_type, $crate::RegisterType::GlobalInstance, }
        }
    };
}
