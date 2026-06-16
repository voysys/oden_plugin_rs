#![allow(missing_docs)]

use crate::plugin_h::{
    OdenEventType_e_OdenEventTypeChar, OdenEventType_e_OdenEventTypeFramebufferResize,
    OdenEventType_e_OdenEventTypeKey, OdenEventType_e_OdenEventTypeMouseButton,
    OdenEventType_e_OdenEventTypeScroll, OdenEventType_e_OdenEventTypeShutdown,
    OdenEventType_e_OdenEventTypeTouchInput, OdenEventType_e_OdenEventTypeWindowFocus,
    OdenEventType_e_OdenEventTypeWindowPos, OdenEventType_e_OdenEventTypeWindowResize,
    OdenTouchInputAction_e_Move, OdenTouchInputAction_e_Press, OdenTouchInputAction_e_Release,
    OdenWindowEvent, OdenWindowEvent_s__bindgen_ty_1,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEvent {
    pub key: i32,
    pub scancode: i32,
    pub actions: i32,
    pub mods: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseButtonEvent {
    pub key: i32,
    pub actions: i32,
    pub mods: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollEvent {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct CharEvent {
    pub c: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct FramebufferResizeEvent {
    pub framebuffer_size: (i32, i32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowResizeEvent {
    pub window_size: (i32, i32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowPosEvent {
    pub window_position: (i32, i32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowFocusEvent {
    pub is_focused: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TouchInputAction {
    Press,
    Move,
    Release,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchInputEvent {
    pub touch_id: i32,
    pub action: TouchInputAction,
    pub pos: (f32, f32),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowEvent {
    KeyEvent(KeyEvent),
    MouseButtonEvent(MouseButtonEvent),
    ScrollEvent(ScrollEvent),
    CharEvent(CharEvent),
    FramebufferResizeEvent(FramebufferResizeEvent),
    WindowResizeEvent(WindowResizeEvent),
    WindowPosEvent(WindowPosEvent),
    WindowFocusEvent(WindowFocusEvent),
    TouchInputEvent(TouchInputEvent),
    ShutdownEvent,
}

impl From<&OdenWindowEvent> for WindowEvent {
    #[allow(non_upper_case_globals)]
    fn from(event: &OdenWindowEvent) -> Self {
        unsafe {
            match event.type_ {
                OdenEventType_e_OdenEventTypeKey => Self::KeyEvent(KeyEvent {
                    key: event.__bindgen_anon_1.k.key,
                    scancode: event.__bindgen_anon_1.k.scancode,
                    actions: event.__bindgen_anon_1.k.actions,
                    mods: event.__bindgen_anon_1.k.mods,
                }),
                OdenEventType_e_OdenEventTypeMouseButton => {
                    Self::MouseButtonEvent(MouseButtonEvent {
                        key: event.__bindgen_anon_1.mb.key,
                        actions: event.__bindgen_anon_1.mb.actions,
                        mods: event.__bindgen_anon_1.mb.mods,
                    })
                }
                OdenEventType_e_OdenEventTypeScroll => Self::ScrollEvent(ScrollEvent {
                    x: event.__bindgen_anon_1.s.x,
                    y: event.__bindgen_anon_1.s.y,
                }),
                OdenEventType_e_OdenEventTypeChar => Self::CharEvent(CharEvent {
                    c: event.__bindgen_anon_1.c.c,
                }),
                OdenEventType_e_OdenEventTypeFramebufferResize => {
                    Self::FramebufferResizeEvent(FramebufferResizeEvent {
                        framebuffer_size: (
                            event.__bindgen_anon_1.fr.framebufferSize.x(),
                            event.__bindgen_anon_1.fr.framebufferSize.y(),
                        ),
                    })
                }
                OdenEventType_e_OdenEventTypeWindowResize => {
                    Self::WindowResizeEvent(WindowResizeEvent {
                        window_size: (
                            event.__bindgen_anon_1.wr.windowSize.x(),
                            event.__bindgen_anon_1.wr.windowSize.y(),
                        ),
                    })
                }
                OdenEventType_e_OdenEventTypeWindowPos => Self::WindowPosEvent(WindowPosEvent {
                    window_position: (
                        event.__bindgen_anon_1.wp.windowPosition.x(),
                        event.__bindgen_anon_1.wp.windowPosition.y(),
                    ),
                }),
                OdenEventType_e_OdenEventTypeWindowFocus => {
                    Self::WindowFocusEvent(WindowFocusEvent {
                        is_focused: event.__bindgen_anon_1.wf.isFocused,
                    })
                }
                OdenEventType_e_OdenEventTypeTouchInput => Self::TouchInputEvent(TouchInputEvent {
                    touch_id: event.__bindgen_anon_1.t.touchId,
                    action: match event.__bindgen_anon_1.t.action {
                        OdenTouchInputAction_e_Press => TouchInputAction::Press,
                        OdenTouchInputAction_e_Move => TouchInputAction::Move,
                        OdenTouchInputAction_e_Release => TouchInputAction::Release,
                        _ => panic!("Unknown touch action {}", event.__bindgen_anon_1.t.action),
                    },
                    pos: (
                        event.__bindgen_anon_1.t.pos.x(),
                        event.__bindgen_anon_1.t.pos.y(),
                    ),
                }),
                OdenEventType_e_OdenEventTypeShutdown => Self::ShutdownEvent,
                _ => panic!("Unknown event type {}", event.type_),
            }
        }
    }
}

impl From<&WindowEvent> for OdenWindowEvent {
    #[allow(non_upper_case_globals)]
    fn from(event: &WindowEvent) -> Self {
        unsafe {
            match event {
                WindowEvent::KeyEvent(KeyEvent {
                    key,
                    scancode,
                    actions,
                    mods,
                }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.k.key = *key;
                    payload.k.scancode = *scancode;
                    payload.k.actions = *actions;
                    payload.k.mods = *mods;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeKey,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::MouseButtonEvent(MouseButtonEvent { key, actions, mods }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.mb.key = *key;
                    payload.mb.actions = *actions;
                    payload.mb.mods = *mods;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeMouseButton,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::ScrollEvent(ScrollEvent { x, y }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.s.x = *x;
                    payload.s.y = *y;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeScroll,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::CharEvent(CharEvent { c }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.c.c = *c;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeChar,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::FramebufferResizeEvent(FramebufferResizeEvent {
                    framebuffer_size,
                }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.fr.framebufferSize.d[0] = framebuffer_size.0;
                    payload.fr.framebufferSize.d[1] = framebuffer_size.1;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeFramebufferResize,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::WindowResizeEvent(WindowResizeEvent { window_size }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.wr.windowSize.d[0] = window_size.0;
                    payload.wr.windowSize.d[1] = window_size.1;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeWindowResize,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::WindowPosEvent(WindowPosEvent { window_position }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.wp.windowPosition.d[0] = window_position.0;
                    payload.wp.windowPosition.d[1] = window_position.1;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeWindowPos,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::WindowFocusEvent(WindowFocusEvent { is_focused }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.wf.isFocused = *is_focused;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeWindowFocus,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::TouchInputEvent(TouchInputEvent {
                    touch_id,
                    action,
                    pos,
                }) => {
                    let mut payload = OdenWindowEvent_s__bindgen_ty_1::default();
                    payload.t.touchId = *touch_id;
                    payload.t.action = match action {
                        TouchInputAction::Press => OdenTouchInputAction_e_Press,
                        TouchInputAction::Move => OdenTouchInputAction_e_Move,
                        TouchInputAction::Release => OdenTouchInputAction_e_Release,
                    };
                    payload.t.pos.d[0] = pos.0;
                    payload.t.pos.d[1] = pos.1;

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeTouchInput,
                        __bindgen_anon_1: payload,
                    }
                }
                WindowEvent::ShutdownEvent => {
                    let payload = OdenWindowEvent_s__bindgen_ty_1::default();

                    OdenWindowEvent {
                        type_: OdenEventType_e_OdenEventTypeShutdown,
                        __bindgen_anon_1: payload,
                    }
                }
            }
        }
    }
}
