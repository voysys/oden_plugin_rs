//! Input API
//!
//! API for handling input from joysticks and gamepads.

use crate::{
    plugin_h::{OdenGamepadState_s, OdenJoystickState_s, OdenPluginGlobalFunctions},
    JoystickState, ODEN_GLOBAL,
};

/// Check if a joystick with the given id is present.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::is_joystick_present;
///
/// if is_joystick_present(0) {
///     // Joystick 0 is present
/// }
/// ```
pub fn is_joystick_present(joystick_id: i32) -> bool {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return false;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.isJoystickPresent {
        unsafe { func(joystick_id) }
    } else {
        panic!("This version of Oden is too old to have the is_joystick_present function");
    }
}

/// Get the state of a joystick with the given id.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::joystick_state;
///
/// if let Some(state) = joystick_state(0) {
///     // Use state
/// }
/// ```
pub fn joystick_state(joystick_id: i32) -> Option<JoystickState> {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return None;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.getJoystickState {
        let mut state = OdenJoystickState_s::default();
        if unsafe { func(joystick_id, &mut state) } {
            Some(JoystickState::from_c(state))
        } else {
            None
        }
    } else {
        panic!("This version of Oden is too old to have the joystick_state function");
    }
}

/// A gamepad with named buttons and axes.
#[derive(Debug, Clone, PartialEq)]
pub struct Gamepad {
    /// The id of the gamepad.
    pub id: i32,
    /// The uuid of the gamepad.
    pub uuid: crate::OdenUuid,
    /// The left stick x axis.
    pub left_x: f32,
    /// The left stick y axis.
    pub left_y: f32,
    /// The right stick x axis.
    pub right_x: f32,
    /// The right stick y axis.
    pub right_y: f32,
    /// The left trigger axis.
    pub left_trigger: f32,
    /// The right trigger axis.
    pub right_trigger: f32,
    /// The south button (A on Xbox, X on PS).
    pub south: bool,
    /// The east button (B on Xbox, O on PS).
    pub east: bool,
    /// The west button (X on Xbox, Square on PS).
    pub west: bool,
    /// The north button (Y on Xbox, Triangle on PS).
    pub north: bool,
    /// The back button (Select/Share).
    pub back: bool,
    /// The guide button (Home/PS).
    pub guide: bool,
    /// The start button (Start/Options).
    pub start: bool,
    /// The left stick click button.
    pub left_stick: bool,
    /// The right stick click button.
    pub right_stick: bool,
    /// The left shoulder button (L1).
    pub left_shoulder: bool,
    /// The right shoulder button (R1).
    pub right_shoulder: bool,
    /// The d-pad up button.
    pub dpad_up: bool,
    /// The d-pad down button.
    pub dpad_down: bool,
    /// The d-pad left button.
    pub dpad_left: bool,
    /// The d-pad right button.
    pub dpad_right: bool,
}

impl From<OdenGamepadState_s> for Gamepad {
    fn from(state: OdenGamepadState_s) -> Self {
        let button = |i: usize| state.buttons.get(i).copied().unwrap_or(0) != 0;
        let axis = |i: usize| *state.axes.get(i).unwrap_or(&0.0);
        Self {
            id: state.id,
            uuid: state.uuid,
            left_x: axis(0),
            left_y: axis(1),
            right_x: axis(2),
            right_y: axis(3),
            left_trigger: axis(4) * 0.5 + 0.5,
            right_trigger: axis(5) * 0.5 + 0.5,
            south: button(0),
            east: button(1),
            west: button(2),
            north: button(3),
            left_shoulder: button(4),
            right_shoulder: button(5),
            back: button(6),
            start: button(7),
            guide: button(8),
            left_stick: button(9),
            right_stick: button(10),
            dpad_up: button(11),
            dpad_right: button(12),
            dpad_down: button(13),
            dpad_left: button(14),
        }
    }
}

/// Get the state of a gamepad with the given id.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::gamepad_state;
///
/// if let Some(gamepad) = gamepad_state(0) {
///     if gamepad.south {
///         // South button pressed
///     }
/// }
/// ```
pub fn gamepad_state(joystick_id: i32) -> Option<Gamepad> {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return None;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.getGamepadState {
        let mut state = OdenGamepadState_s::default();
        if unsafe { func(joystick_id, &mut state) } {
            Some(state.into())
        } else {
            None
        }
    } else {
        panic!("This version of Oden is too old to have the gamepad_state function");
    }
}

/// Get the state of the gamepad if exactly one is connected.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::gamepad;
///
/// if let Some(gamepad) = gamepad() {
///     if gamepad.south {
///         // South button pressed
///     }
/// }
/// ```
pub fn gamepad() -> Option<Gamepad> {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return None;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.getGamepad {
        let mut state = OdenGamepadState_s::default();
        if unsafe { func(&mut state) } {
            Some(state.into())
        } else {
            None
        }
    } else {
        panic!("This version of Oden is too old to have the gamepad function");
    }
}

/// Add a gamepad mapping.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::add_gamepad_mapping;
///
/// add_gamepad_mapping("030000006d0400004fc2000000000000,Logitech G HUB G29 Driving Force Racing Wheel USB,platform:Windows,a:b0,b:b2,x:b1,y:b3,back:b7,guide:b24,start:b6,leftstick:b11,rightstick:b10,leftshoulder:b5,rightshoulder:b4,dpup:h0.1,dpdown:h0.4,dpleft:h0.8,dpright:h0.2,leftx:a0,lefttrigger:a2~,righttrigger:a1~,");
/// ```
pub fn add_gamepad_mapping(mapping: &str) -> bool {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return false;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.addGamepadMapping {
        let mapping_c = std::ffi::CString::new(mapping).unwrap();
        unsafe { func(mapping_c.as_ptr()) }
    } else {
        panic!("This version of Oden is too old to have the add_gamepad_mapping function");
    }
}

/// Power state of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// Unknown power state.
    Unknown = 0,
    /// The system is running on battery.
    OnBattery = 1,
    /// The system is running on AC power.
    NoBattery = 2,
    /// The battery is charging.
    Charging = 3,
    /// The battery is charged.
    Charged = 4,
}

impl PowerState {
    fn from_c(state: crate::plugin_h::OdenPowerState) -> Self {
        use crate::plugin_h::OdenPowerState::*;
        #[allow(non_snake_case)]
        match state {
            OdenPowerStateUnknown => PowerState::Unknown,
            OdenPowerStateOnBattery => PowerState::OnBattery,
            OdenPowerStateNoBattery => PowerState::NoBattery,
            OdenPowerStateCharging => PowerState::Charging,
            OdenPowerStateCharged => PowerState::Charged,
            _ => PowerState::Unknown,
        }
    }
}

/// Get the power info of the system.
///
/// Returns (seconds, percent, state).
///
/// This function can be called from any thread at any time.
///
/// There might be an overhead to calling this function on some systems, it is recommended to not all it more than once per second.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::get_power_info;
///
/// if let Some((seconds, percent, state)) = get_power_info() {
///     println!("Seconds: {}, Percent: {}, State: {:?}", seconds, percent, state);
/// }
/// ```
pub fn get_power_info() -> Option<(i32, i32, PowerState)> {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return None;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(func) = globals.getPowerInfo {
        let mut seconds = 0;
        let mut percent = 0;
        let mut state = crate::plugin_h::OdenPowerState::OdenPowerStateUnknown;
        if unsafe { func(&mut seconds, &mut percent, &mut state) } {
            Some((seconds, percent, PowerState::from_c(state)))
        } else {
            None
        }
    } else {
        panic!("This version of Oden is too old to have the get_power_info function");
    }
}
