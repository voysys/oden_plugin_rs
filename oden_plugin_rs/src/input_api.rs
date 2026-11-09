//! Input API
//!
//! API for handling input from joysticks and gamepads.

use crate::{
    math, oden_global,
    plugin_h::{OdenGamepadState_s, OdenHandTrackingState_s, OdenJoystickState_s},
    JoystickState,
};

#[cfg(feature = "glam_conversion")]
use glam;

pub use crate::plugin_h::{
    ODEN_JOYSTICK_POSE_AXIS, ODEN_JOYSTICK_POSE_CALIBRATED, ODEN_JOYSTICK_POSE_FLAGS_AXIS,
    ODEN_JOYSTICK_POSE_ORIENTATION, ODEN_JOYSTICK_POSE_POSITION, ODEN_JOYSTICK_POSE_RELATIVE_YAW,
    ODEN_JOYSTICK_POSE_VALID, ODEN_JOYSTICK_REFERENCE_AXIS,
};

impl JoystickState {
    /// Optional capability/status bits stored in axis 96 by pose-aware producers.
    /// Older VR backends omit this channel; `None` means unknown capabilities.
    pub fn pose_flags(&self) -> Option<u32> {
        let flags = *self.axes.get(ODEN_JOYSTICK_POSE_FLAGS_AXIS as usize)?;
        if !flags.is_finite() || flags < 0.0 || flags > 16_777_215.0 || flags.fract() != 0.0 {
            return None;
        }
        Some(flags as u32)
    }

    /// Controller-to-tracking-space pose from the existing VR joystick channels.
    /// Returns `None` for absent, stale or malformed matrices. Orientation-only
    /// input has zero translation; this does not mean its position was tracked.
    /// Check `pose_flags()` for capabilities. Legacy VR poses lack those flags.
    /// No headset/reference transform is applied automatically.
    pub fn pose(&self) -> Option<math::Matrix4> {
        if self.axes.len() > ODEN_JOYSTICK_POSE_FLAGS_AXIS as usize {
            let flags = self.pose_flags()?;
            if flags & ODEN_JOYSTICK_POSE_VALID == 0 || flags & ODEN_JOYSTICK_POSE_ORIENTATION == 0
            {
                return None;
            }
        }
        self.pose_matrix_at(ODEN_JOYSTICK_POSE_AXIS as usize)
    }

    /// Backend-specific headset/reference matrix (axes 80..95).
    /// Orientation-only SDL input supplies identity. VR semantics are unchanged.
    pub fn pose_reference(&self) -> Option<math::Matrix4> {
        self.pose_matrix_at(ODEN_JOYSTICK_REFERENCE_AXIS as usize)
    }

    fn pose_matrix_at(&self, start: usize) -> Option<math::Matrix4> {
        let d: [f32; 16] = self.axes.get(start..start + 16)?.try_into().ok()?;
        if !d.iter().all(|v| v.is_finite())
            || [d[3], d[7], d[11], d[15] - 1.0]
                .iter()
                .any(|v| v.abs() > 0.001)
        {
            return None;
        }
        for a in 0..3 {
            for b in a..3 {
                let dot: f32 = (0..3).map(|r| d[a * 4 + r] * d[b * 4 + r]).sum();
                let expected = if a == b { 1.0 } else { 0.0 };
                if (dot - expected).abs() > 0.01 {
                    return None;
                }
            }
        }
        let determinant = d[0] * (d[5] * d[10] - d[6] * d[9]) - d[4] * (d[1] * d[10] - d[2] * d[9])
            + d[8] * (d[1] * d[6] - d[2] * d[5]);
        if (determinant - 1.0).abs() > 0.01 {
            return None;
        }
        Some(math::Matrix4 {
            d: std::array::from_fn(|c| std::array::from_fn(|r| d[c * 4 + r])),
        })
    }
}

#[cfg(test)]
mod joystick_pose_tests {
    use super::*;

    fn legacy_pose() -> JoystickState {
        let mut state = JoystickState::default();
        state.axes.resize(96, 0.0);
        for i in 0..4 {
            state.axes[64 + 5 * i] = 1.0;
            state.axes[80 + 5 * i] = 1.0;
        }
        state.axes[76] = 2.0;
        state
    }

    #[test]
    fn legacy_vr_pose_keeps_translation_and_unknown_capabilities() {
        let state = legacy_pose();
        assert_eq!(state.pose().unwrap().d[3][0], 2.0);
        assert_eq!(state.pose_flags(), None);
        assert_eq!(state.pose_reference().unwrap().d[3][3], 1.0);
    }

    #[test]
    fn orientation_only_pose_requires_fresh_data() {
        let mut state = legacy_pose();
        state.axes[76] = 0.0;
        let flags = ODEN_JOYSTICK_POSE_ORIENTATION | ODEN_JOYSTICK_POSE_RELATIVE_YAW;
        state.axes.push(flags as f32);
        assert!(state.pose().is_none());
        state.axes[96] = (flags | ODEN_JOYSTICK_POSE_VALID) as f32;
        assert!(state.pose().is_some());
        assert_eq!(state.pose_flags().unwrap() & ODEN_JOYSTICK_POSE_POSITION, 0);
    }

    #[test]
    fn missing_zero_nan_and_nonrigid_poses_are_rejected() {
        assert!(JoystickState::default().pose().is_none());
        let mut state = legacy_pose();
        state.axes[64] = f32::NAN;
        assert!(state.pose().is_none());
        state.axes[64] = 2.0;
        assert!(state.pose().is_none());
        state.axes.fill(0.0);
        assert!(state.pose().is_none());
    }

    #[test]
    fn invalid_status_and_each_nonfinite_matrix_element_are_rejected() {
        for flags in [
            -1.0,
            0.5,
            f32::NAN,
            f32::INFINITY,
            16_777_216.0,
            0.0,
            4.0,
            1.0,
        ] {
            let mut state = legacy_pose();
            state.axes.push(flags);
            assert!(state.pose().is_none(), "invalid flags {flags}");
        }
        for offset in 0..16 {
            for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut state = legacy_pose();
                state.axes[64 + offset] = bad;
                assert!(state.pose().is_none());
            }
        }
        let mut state = legacy_pose();
        state.axes[64] = -1.0;
        assert!(state.pose().is_none(), "A reflection is not a rotation");
        state.axes[64] = 1.0;
        state.axes[67] = 0.1;
        assert!(state.pose().is_none(), "A perspective matrix is not a pose");
    }

    #[test]
    fn truncated_and_random_channels_do_not_panic_or_decode_as_poses() {
        let mut seed = 12345u32;
        for length in 0..=128 {
            for _ in 0..100 {
                let mut state = JoystickState::default();
                for _ in 0..length {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    state.axes.push(f32::from_bits(seed));
                }
                assert!(state.pose().is_none());
                assert!(state.pose_reference().is_none());
                let _ = state.pose_flags();
            }
        }
    }

    #[test]
    fn valid_rotations_and_future_status_bits_preserve_column_order() {
        for i in 0..1000 {
            let angle = i as f32 * 0.01;
            let (s, c) = angle.sin_cos();
            let mut state = legacy_pose();
            state.axes[64] = c;
            state.axes[66] = -s;
            state.axes[72] = s;
            state.axes[74] = c;
            state
                .axes
                .push((ODEN_JOYSTICK_POSE_ORIENTATION | ODEN_JOYSTICK_POSE_VALID | 1024) as f32);
            let pose = state.pose().unwrap();
            assert_eq!(pose.d[0][2], -s);
            assert_eq!(pose.d[2][0], s);
            assert_eq!(pose.d[3][0], 2.0);
        }
    }
}

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
    let Some(globals) = oden_global() else {
        return false;
    };

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
    let globals = oden_global()?;

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

/// Turn gyroscope and accelerometer input on or off for the joystick with the given id.
///
/// Motion input is off by default. While it is on, a gamepad with a gyroscope and an
/// accelerometer appends six raw IMU axes after its touchpad channels and publishes its
/// orientation in the pose channels, see [`JoystickState::pose`]. The setting belongs to
/// the joystick id: it stays until it is turned off, also across reconnects, and other
/// plugins share it. Returns `false` if the id is out of range or this Oden build has no
/// gamepad motion support.
///
/// This function can be called from any thread at any time.
///
/// # Examples
/// ```no_run
/// use oden_plugin_rs::input_api::{joystick_state, set_joystick_motion_enabled};
///
/// set_joystick_motion_enabled(0, true);
/// if let Some(pose) = joystick_state(0).and_then(|state| state.pose()) {
///     // Use pose
/// }
/// ```
pub fn set_joystick_motion_enabled(joystick_id: i32, enabled: bool) -> bool {
    let Some(globals) = oden_global() else {
        return false;
    };

    if let Some(func) = globals.setJoystickMotionEnabled {
        unsafe { func(joystick_id, enabled) }
    } else {
        panic!("This version of Oden is too old to have the set_joystick_motion_enabled function");
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
    let globals = oden_global()?;

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
    let globals = oden_global()?;

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
    let Some(globals) = oden_global() else {
        return false;
    };

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
    let globals = oden_global()?;

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

// ---- Hand Tracking (OpenXR 26-joint model) ----

/// Hand tracking is active and data is valid.
pub const HAND_FLAG_ACTIVE: u8 = 1;
/// This is the left hand.
pub const HAND_FLAG_LEFT: u8 = 2;
/// This is the right hand.
pub const HAND_FLAG_RIGHT: u8 = 4;
/// Joint positions are in wrist-local coordinates (default from API).
pub const HAND_FLAG_LOCAL: u8 = 8;

/// OpenXR 26-joint hand tracking state.
///
/// Joint positions are in the coordinate space indicated by `flags`:
/// - `HAND_FLAG_LOCAL` set: wrist-local
/// - `HAND_FLAG_LOCAL` clear: world-space
///
/// The wrist-local frame is oriented so that an identical physical gesture produces identical
/// joint coordinates on either hand -- gesture detection written for the right hand works
/// unchanged on the left. Concretely:
///
/// - `+X` points from the wrist toward the thumb metacarpal
/// - `+Y` points from the wrist toward the index metacarpal (orthogonalized against X)
/// - `+Z` is the palm normal; on the right hand this is `X x Y`, on the left hand the Z axis
///   is negated so both hands have Z pointing the same way relative to the palm
///
/// The left-hand mirror is only visible in `joint_positions`. `wrist_rotation` stays a proper
/// right-handed rotation. The `to_world` / `to_local` / `joint_world_position` helpers absorb
/// the mirror internally, so you never need to apply it yourself.
///
/// The math helpers require the `glam_conversion` feature.
#[derive(Debug, Clone, PartialEq)]
pub struct HandTrackingState {
    /// Bitfield of `HAND_FLAG_*` constants.
    pub flags: u8,
    /// World-space wrist position.
    pub wrist_position: math::Vec3,
    /// World-space wrist rotation.
    pub wrist_rotation: math::Quat,
    /// 26 joint positions in the coordinate space indicated by flags.
    pub joint_positions: [math::Vec3; 26],
}

impl HandTrackingState {
    /// True if tracking data is valid.
    pub fn is_active(&self) -> bool {
        self.flags & HAND_FLAG_ACTIVE != 0
    }

    /// True if joint positions are in wrist-local coordinates.
    pub fn is_local(&self) -> bool {
        self.flags & HAND_FLAG_LOCAL != 0
    }

    /// True if this is the left hand.
    pub fn is_left(&self) -> bool {
        self.flags & HAND_FLAG_LEFT != 0
    }
}

// Undo the left-hand Z mirror before rotating, so `wrist_rotation` stays a proper rotation.
// See [`HandTrackingState`] for why the local frame is mirrored on the left hand.
#[cfg(feature = "glam_conversion")]
#[inline]
fn unmirror_if_left(v: glam::Vec3, is_left: bool) -> glam::Vec3 {
    if is_left {
        glam::Vec3::new(v.x, v.y, -v.z)
    } else {
        v
    }
}

#[cfg(feature = "glam_conversion")]
impl HandTrackingState {
    /// Convert joint positions to world-space (in-place). No-op if already world.
    pub fn to_world(&mut self) {
        if !self.is_local() {
            return;
        }
        let is_left = self.is_left();
        let q: glam::Quat = self.wrist_rotation.into();
        let wrist: glam::Vec3 = self.wrist_position.into();
        for joint in &mut self.joint_positions {
            *joint = (wrist + q * unmirror_if_left(glam::Vec3::from(*joint), is_left)).into();
        }
        self.flags &= !HAND_FLAG_LOCAL;
    }

    /// Convert joint positions to wrist-local coordinates (in-place). No-op if already local.
    pub fn to_local(&mut self) {
        if self.is_local() {
            return;
        }
        let is_left = self.is_left();
        let inv_q = glam::Quat::from(self.wrist_rotation).conjugate();
        let wrist: glam::Vec3 = self.wrist_position.into();
        for joint in &mut self.joint_positions {
            *joint = unmirror_if_left(inv_q * (glam::Vec3::from(*joint) - wrist), is_left).into();
        }
        self.flags |= HAND_FLAG_LOCAL;
    }

    /// Get a joint's world-space position (without modifying the struct).
    pub fn joint_world_position(&self, index: usize) -> math::Vec3 {
        if !self.is_local() {
            return self.joint_positions[index];
        }
        let q: glam::Quat = self.wrist_rotation.into();
        let wrist: glam::Vec3 = self.wrist_position.into();
        let local = unmirror_if_left(
            glam::Vec3::from(self.joint_positions[index]),
            self.is_left(),
        );
        (wrist + q * local).into()
    }
}

/// Get hand tracking state. hand: 0=left, 1=right.
///
/// Returns `None` if no hand tracking data is available for the requested hand.
///
/// # Examples
/// ```no_run
/// # #[cfg(feature = "glam_conversion")] {
/// use oden_plugin_rs::input_api::hand_tracking_state;
///
/// if let Some(mut state) = hand_tracking_state(0) {
///     // Get a fingertip's world position
///     let _index_tip = state.joint_world_position(10);
///     // Or convert all joints to world-space in place
///     state.to_world();
/// }
/// # }
/// ```
pub fn hand_tracking_state(hand: i32) -> Option<HandTrackingState> {
    let globals = oden_global()?;

    if let Some(func) = globals.getHandTrackingState {
        let mut state = OdenHandTrackingState_s::default();
        if unsafe { func(hand, &mut state) } {
            Some(HandTrackingState {
                flags: state.flags,
                wrist_position: state.wrist_position,
                wrist_rotation: state.wrist_rotation,
                joint_positions: state.joint_positions,
            })
        } else {
            None
        }
    } else {
        panic!("This version of Oden is too old to have the hand_tracking_state function");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_state() -> HandTrackingState {
        let mut joints = [math::Vec3::new(); 26];
        joints[5] = math::Vec3::xyz(0.05, 0.0, 0.0);
        joints[10] = math::Vec3::xyz(0.0, 0.08, 0.0);
        joints[15] = math::Vec3::xyz(0.0, 0.0, 0.07);

        HandTrackingState {
            flags: HAND_FLAG_ACTIVE | HAND_FLAG_RIGHT | HAND_FLAG_LOCAL,
            wrist_position: math::Vec3::xyz(1.0, 2.0, 3.0),
            wrist_rotation: math::Quat::wxyz(1.0, 0.0, 0.0, 0.0),
            joint_positions: joints,
        }
    }

    #[test]
    fn flags_are_correct() {
        let state = make_test_state();
        assert!(state.is_active());
        assert!(!state.is_left());
        assert!(state.is_local());

        let left_state = HandTrackingState {
            flags: HAND_FLAG_ACTIVE | HAND_FLAG_LEFT | HAND_FLAG_LOCAL,
            ..make_test_state()
        };
        assert!(left_state.is_left());
        assert!(left_state.is_active());
        assert!(left_state.is_local());

        let inactive = HandTrackingState {
            flags: 0,
            ..make_test_state()
        };
        assert!(!inactive.is_active());
        assert!(!inactive.is_local());
    }

    #[test]
    fn hand_tracking_state_returns_none_without_oden() {
        assert!(hand_tracking_state(0).is_none());
        assert!(hand_tracking_state(1).is_none());
    }

    #[cfg(feature = "glam_conversion")]
    fn approx_eq(a: math::Vec3, b: math::Vec3, tol: f32) -> bool {
        (glam::Vec3::from(a) - glam::Vec3::from(b))
            .abs()
            .max_element()
            < tol
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn to_world_identity_rotation() {
        let mut state = make_test_state();
        assert!(state.is_local());

        state.to_world();

        assert!(!state.is_local());
        assert!(approx_eq(
            state.joint_positions[5],
            math::Vec3::xyz(1.05, 2.0, 3.0),
            1e-5
        ));
        assert!(approx_eq(
            state.joint_positions[10],
            math::Vec3::xyz(1.0, 2.08, 3.0),
            1e-5
        ));
        assert!(approx_eq(
            state.joint_positions[15],
            math::Vec3::xyz(1.0, 2.0, 3.07),
            1e-5
        ));
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn to_local_inverts_to_world() {
        let original = make_test_state();
        let mut state = original.clone();

        state.to_world();
        state.to_local();

        assert!(state.is_local());
        for (i, (got, expected)) in state
            .joint_positions
            .iter()
            .zip(original.joint_positions.iter())
            .enumerate()
        {
            assert!(
                approx_eq(*got, *expected, 1e-4),
                "joint {} mismatch: {:?} vs {:?}",
                i,
                got,
                expected
            );
        }
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn to_world_noop_if_already_world() {
        let mut state = make_test_state();
        state.to_world();
        let after_first = state.joint_positions;

        state.to_world();
        assert_eq!(state.joint_positions, after_first);
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn to_local_noop_if_already_local() {
        let state_before = make_test_state();
        let mut state = state_before.clone();

        state.to_local();
        assert_eq!(state.joint_positions, state_before.joint_positions);
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn joint_world_position_from_local() {
        let state = make_test_state();
        let world_pos = state.joint_world_position(5);
        assert!(approx_eq(world_pos, math::Vec3::xyz(1.05, 2.0, 3.0), 1e-5));
        assert!(state.is_local());
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn joint_world_position_from_world() {
        let mut state = make_test_state();
        state.to_world();
        let expected = state.joint_positions[10];
        let world_pos = state.joint_world_position(10);
        assert!(approx_eq(world_pos, expected, 1e-6));
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn round_trip_with_rotation() {
        // 90 deg around Y axis
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let mut state = make_test_state();
        state.wrist_rotation = math::Quat::wxyz(s, 0.0, s, 0.0);

        let original_joints = state.joint_positions;

        state.to_world();
        state.to_local();

        for (i, (got, expected)) in state
            .joint_positions
            .iter()
            .zip(original_joints.iter())
            .enumerate()
        {
            assert!(
                approx_eq(*got, *expected, 1e-4),
                "joint {} round-trip failed with rotation: {:?} vs {:?}",
                i,
                got,
                expected
            );
        }
    }

    #[cfg(feature = "glam_conversion")]
    #[test]
    fn left_hand_mirrors_z() {
        // Two hands with identical local coords should land on physically mirrored world positions
        // (mirror plane: the wrist's XY plane in world). With identity wrist rotation and identical
        // wrist position, that means right.world.z = wrist.z + local.z while left.world.z =
        // wrist.z - local.z -- the two are reflections of each other.
        let mut right = make_test_state();
        let mut left = HandTrackingState {
            flags: HAND_FLAG_ACTIVE | HAND_FLAG_LEFT | HAND_FLAG_LOCAL,
            ..make_test_state()
        };
        assert!(!right.is_left());
        assert!(left.is_left());

        right.to_world();
        left.to_world();

        // Joint 15 has local z = 0.07; only Z differs between hands.
        assert!(approx_eq(
            right.joint_positions[15],
            math::Vec3::xyz(1.0, 2.0, 3.07),
            1e-5
        ));
        assert!(approx_eq(
            left.joint_positions[15],
            math::Vec3::xyz(1.0, 2.0, 2.93),
            1e-5
        ));

        // Non-Z components are untouched for both hands.
        assert!(approx_eq(
            right.joint_positions[5],
            math::Vec3::xyz(1.05, 2.0, 3.0),
            1e-5
        ));
        assert!(approx_eq(
            left.joint_positions[5],
            math::Vec3::xyz(1.05, 2.0, 3.0),
            1e-5
        ));
        assert!(approx_eq(
            right.joint_positions[10],
            math::Vec3::xyz(1.0, 2.08, 3.0),
            1e-5
        ));
        assert!(approx_eq(
            left.joint_positions[10],
            math::Vec3::xyz(1.0, 2.08, 3.0),
            1e-5
        ));
    }
}
