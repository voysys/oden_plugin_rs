use super::{painter, shader_version, Painter};
use crate::{
    log, plugin_h, DrawParams, InitAndGraphicsApi, UpdateParams, WindowEvent, PLUGIN_MOD_ALT,
    PLUGIN_MOD_CONTROL, PLUGIN_MOD_SHIFT, PLUGIN_MOD_SUPER,
};
use egui::{
    vec2, Event, Key, Modifiers, PointerButton, Pos2, RawInput, Rect, ViewportIdMap, ViewportInfo,
};
use shader_version::ShaderVersion;
use std::{mem, rc::Rc, time::Instant};

pub struct OdenEgui {
    pub egui_ctx: egui::Context,
    pub painter: painter::Painter,

    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    pixels_per_point: f32,
    last_mouse_pos: Pos2,
    time: Instant,
    gui_visible: bool,
    was_updated: bool,
}

impl OdenEgui {
    pub fn new(api: &impl InitAndGraphicsApi) -> Option<Self> {
        let oden_gl = api.gl_loader()?;

        let gl = Rc::new(unsafe {
            glow::Context::from_loader_function_cstr(move |s| oden_gl(s.as_ptr()))
        });

        let painter = Painter::new(gl, "", Some(ShaderVersion::Gl140))
            .map_err(|error| {
                log::error!("error occurred in initializing painter:\n{}", error);
            })
            .unwrap();

        let egui_ctx = egui::Context::default();

        Some(Self {
            egui_ctx,
            painter,
            shapes: Default::default(),
            textures_delta: Default::default(),
            pixels_per_point: Default::default(),
            last_mouse_pos: Pos2::default(),
            time: Instant::now(),
            gui_visible: true,
            was_updated: false,
        })
    }

    pub fn update(&mut self, api: &UpdateParams, run_ui: impl FnMut(&egui::Context)) {
        self.was_updated = true;

        self.shapes.clear();
        self.textures_delta = Default::default();

        if self.egui_ctx.is_pointer_over_area() {
            api.inhibit_mouse_look();
        }

        let viewport = api.viewport_info();

        let mouse_over_gui = api.is_over_gui().unwrap_or(false);

        let offset = if api.gui_visible() {
            self.gui_visible = true;
            (400.0, 24.0)
        } else {
            self.gui_visible = false;
            (0.0, 0.0)
        };

        let screen_rect = Rect::from_min_max(
            Pos2::new(0.0 + offset.0, 0.0),
            Pos2::new(
                (viewport.width_px as f32 / viewport.content_scale.x()) + offset.0,
                (viewport.height_px as f32 / viewport.content_scale.y()) + offset.1,
            ),
        );

        let raw_input = if !mouse_over_gui {
            let mouse_pos = api.mouse_pos();

            let mouse_pos = Pos2 {
                x: mouse_pos.x() * viewport.width_px as f32 / viewport.content_scale.x() + offset.0,
                y: mouse_pos.y() * viewport.height_px as f32 / viewport.content_scale.y(),
            };

            let mut events = api
                .window_events()
                .into_iter()
                .filter_map(|event| match event {
                    WindowEvent::KeyEvent(key_event) => {
                        map_key(key_event.key).map(|key| Event::Key {
                            key,
                            physical_key: None,
                            pressed: key_event.actions == plugin_h::PLUGIN_KEY_ACTION_PRESS as i32,
                            repeat: key_event.actions == plugin_h::PLUGIN_KEY_ACTION_REPEAT as i32,
                            modifiers: process_mods(key_event.mods),
                        })
                    }
                    WindowEvent::MouseButtonEvent(mouse_event) => {
                        Some(Event::PointerButton {
                            pos: mouse_pos,
                            button: match mouse_event.key as u32 {
                                plugin_h::PLUGIN_MOUSE_BUTTON_LEFT => PointerButton::Primary,
                                plugin_h::PLUGIN_MOUSE_BUTTON_RIGHT => PointerButton::Secondary,
                                plugin_h::PLUGIN_MOUSE_BUTTON_MIDDLE => PointerButton::Middle,
                                _ => {
                                    PointerButton::Primary // TODO! look into this
                                }
                            },
                            pressed: api.mouse_pressed_since_last_swap(),
                            modifiers: process_mods(mouse_event.mods),
                        })
                    }
                    WindowEvent::ScrollEvent(scroll_event) => Some(Event::Scroll(vec2(
                        scroll_event.x as f32 * 10.0,
                        scroll_event.y as f32 * 10.0,
                    ))),
                    WindowEvent::CharEvent(char_event) => {
                        Some(Event::Text((char_event.c as u8 as char).to_string()))
                    }
                    WindowEvent::FramebufferResizeEvent(_) => None,
                    WindowEvent::WindowResizeEvent(_) => None,
                    WindowEvent::WindowPosEvent(_) => None,
                    WindowEvent::WindowFocusEvent(_) => None,
                    WindowEvent::TouchInputEvent(_) => None,
                    WindowEvent::ShutdownEvent => None,
                })
                .collect::<Vec<_>>();

            if self.last_mouse_pos != mouse_pos {
                events.push(Event::PointerMoved(mouse_pos));
            }

            self.last_mouse_pos = mouse_pos;

            RawInput {
                viewport_id: self.egui_ctx.viewport_id(),
                viewports: {
                    let mut viewports = ViewportIdMap::default();
                    viewports.insert(
                        self.egui_ctx.viewport_id(),
                        ViewportInfo {
                            native_pixels_per_point: Some(viewport.content_scale.x()),
                            ..Default::default()
                        },
                    );
                    viewports
                },
                screen_rect: Some(screen_rect),
                events,
                time: Some(self.time.elapsed().as_secs_f64()),
                ..Default::default()
            }
        } else {
            RawInput {
                viewport_id: self.egui_ctx.viewport_id(),
                viewports: {
                    let mut viewports = ViewportIdMap::default();
                    viewports.insert(
                        self.egui_ctx.viewport_id(),
                        ViewportInfo {
                            native_pixels_per_point: Some(viewport.content_scale.x()),
                            ..Default::default()
                        },
                    );
                    viewports
                },
                screen_rect: Some(screen_rect),
                time: Some(self.time.elapsed().as_secs_f64()),
                ..Default::default()
            }
        };

        let egui::FullOutput {
            platform_output: _,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = self.egui_ctx.run(raw_input, run_ui);

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);
        self.pixels_per_point = pixels_per_point;
    }

    pub fn draw(&mut self, api: &DrawParams) {
        let was_updated = mem::replace(&mut self.was_updated, false);

        if !was_updated {
            return;
        }

        let (mut width, mut height) = api.target_size();

        if self.gui_visible {
            height -= (24.0 * self.pixels_per_point) as i32;
        };

        width = width.max(1);
        height = height.max(1);

        let mut textures_delta = self.textures_delta.clone();

        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(id, &image_delta);
        }

        let clipped_primitives = self
            .egui_ctx
            .tessellate(self.shapes.clone(), self.pixels_per_point);

        let dimensions: [u32; 2] = [width as u32, height as u32];

        self.painter.paint_primitives(
            dimensions,
            self.egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );

        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id);
        }
    }

    pub fn destroy(&mut self) {
        self.painter.destroy();
    }
}

impl Drop for OdenEgui {
    fn drop(&mut self) {
        // !IMPORTANT: The painter needs to be properly destroyed in order to not leak objects
        self.painter.destroy();
    }
}

fn map_key(key: i32) -> Option<Key> {
    Some(match key as u32 {
        plugin_h::PLUGIN_KEY_DOWN => Key::ArrowDown,
        plugin_h::PLUGIN_KEY_LEFT => Key::ArrowLeft,
        plugin_h::PLUGIN_KEY_RIGHT => Key::ArrowRight,
        plugin_h::PLUGIN_KEY_UP => Key::ArrowUp,
        plugin_h::PLUGIN_KEY_ESCAPE => Key::Escape,
        plugin_h::PLUGIN_KEY_TAB => Key::Tab,
        plugin_h::PLUGIN_KEY_BACKSPACE => Key::Backspace,
        plugin_h::PLUGIN_KEY_ENTER => Key::Enter,
        plugin_h::PLUGIN_KEY_SPACE => Key::Space,
        plugin_h::PLUGIN_KEY_INSERT => Key::Insert,
        plugin_h::PLUGIN_KEY_DELETE => Key::Delete,
        plugin_h::PLUGIN_KEY_HOME => Key::Home,
        plugin_h::PLUGIN_KEY_END => Key::End,
        plugin_h::PLUGIN_KEY_PAGE_UP => Key::PageUp,
        plugin_h::PLUGIN_KEY_PAGE_DOWN => Key::PageDown,
        plugin_h::PLUGIN_KEY_MINUS => Key::Minus,
        plugin_h::PLUGIN_KEY_EQUAL => Key::Equals,
        plugin_h::PLUGIN_KEY_0 => Key::Num0,
        plugin_h::PLUGIN_KEY_1 => Key::Num1,
        plugin_h::PLUGIN_KEY_2 => Key::Num2,
        plugin_h::PLUGIN_KEY_3 => Key::Num3,
        plugin_h::PLUGIN_KEY_4 => Key::Num4,
        plugin_h::PLUGIN_KEY_5 => Key::Num5,
        plugin_h::PLUGIN_KEY_6 => Key::Num6,
        plugin_h::PLUGIN_KEY_7 => Key::Num7,
        plugin_h::PLUGIN_KEY_8 => Key::Num8,
        plugin_h::PLUGIN_KEY_9 => Key::Num9,
        plugin_h::PLUGIN_KEY_A => Key::A,
        plugin_h::PLUGIN_KEY_B => Key::B,
        plugin_h::PLUGIN_KEY_C => Key::C,
        plugin_h::PLUGIN_KEY_D => Key::D,
        plugin_h::PLUGIN_KEY_E => Key::E,
        plugin_h::PLUGIN_KEY_F => Key::F,
        plugin_h::PLUGIN_KEY_G => Key::G,
        plugin_h::PLUGIN_KEY_H => Key::H,
        plugin_h::PLUGIN_KEY_I => Key::I,
        plugin_h::PLUGIN_KEY_J => Key::J,
        plugin_h::PLUGIN_KEY_K => Key::K,
        plugin_h::PLUGIN_KEY_L => Key::L,
        plugin_h::PLUGIN_KEY_M => Key::M,
        plugin_h::PLUGIN_KEY_N => Key::N,
        plugin_h::PLUGIN_KEY_O => Key::O,
        plugin_h::PLUGIN_KEY_P => Key::P,
        plugin_h::PLUGIN_KEY_Q => Key::Q,
        plugin_h::PLUGIN_KEY_R => Key::R,
        plugin_h::PLUGIN_KEY_S => Key::S,
        plugin_h::PLUGIN_KEY_T => Key::T,
        plugin_h::PLUGIN_KEY_U => Key::U,
        plugin_h::PLUGIN_KEY_V => Key::V,
        plugin_h::PLUGIN_KEY_W => Key::W,
        plugin_h::PLUGIN_KEY_X => Key::X,
        plugin_h::PLUGIN_KEY_Y => Key::Y,
        plugin_h::PLUGIN_KEY_Z => Key::Z,
        plugin_h::PLUGIN_KEY_F1 => Key::F1,
        plugin_h::PLUGIN_KEY_F2 => Key::F2,
        plugin_h::PLUGIN_KEY_F3 => Key::F3,
        plugin_h::PLUGIN_KEY_F4 => Key::F4,
        plugin_h::PLUGIN_KEY_F5 => Key::F5,
        plugin_h::PLUGIN_KEY_F6 => Key::F6,
        plugin_h::PLUGIN_KEY_F7 => Key::F7,
        plugin_h::PLUGIN_KEY_F8 => Key::F8,
        plugin_h::PLUGIN_KEY_F9 => Key::F9,
        plugin_h::PLUGIN_KEY_F10 => Key::F10,
        plugin_h::PLUGIN_KEY_F11 => Key::F11,
        plugin_h::PLUGIN_KEY_F12 => Key::F12,
        plugin_h::PLUGIN_KEY_F13 => Key::F13,
        plugin_h::PLUGIN_KEY_F14 => Key::F14,
        plugin_h::PLUGIN_KEY_F15 => Key::F15,
        plugin_h::PLUGIN_KEY_F16 => Key::F16,
        plugin_h::PLUGIN_KEY_F17 => Key::F17,
        plugin_h::PLUGIN_KEY_F18 => Key::F18,
        plugin_h::PLUGIN_KEY_F19 => Key::F19,
        plugin_h::PLUGIN_KEY_F20 => Key::F20,
        _ => {
            return None;
        }
    })
}

/// Oden Plugin modifiers to Egui modifiers
fn process_mods(mods: i32) -> Modifiers {
    let mut modifier = Modifiers::default();

    if (mods & PLUGIN_MOD_CONTROL as i32) != 0 {
        modifier.ctrl = true;
    }

    if (mods & PLUGIN_MOD_ALT as i32) != 0 {
        modifier.alt = true;
    }

    if (mods & PLUGIN_MOD_SUPER as i32) != 0 {
        modifier.command = true;
    }

    if (mods & PLUGIN_MOD_SHIFT as i32) != 0 {
        modifier.shift = true;
    }

    modifier
}
