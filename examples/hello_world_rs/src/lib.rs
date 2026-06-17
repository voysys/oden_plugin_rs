use oden_plugin_rs::{
    math::{Vec3, Vec4},
    register_plugin, GuiParams, InitParams, OdenPlugin, ShutdownParams, UpdateParams,
};

struct State {
    _dummy: bool,
}

impl OdenPlugin for State {
    fn init(_api: &InitParams) -> Self {
        State { _dummy: false }
    }

    fn shutdown(self, _api: &ShutdownParams) {}

    fn update(&mut self, api: &UpdateParams) {
        api.im_draw_add_text(
            "Hello World!",
            Vec3::xyz(-1.0, 0.0, -2.0),
            Vec3::xyz(0.0, 0.3, 0.0),
            Vec3::xyz(1.0, 0.0, 0.0),
            Vec4::rgba(0.8, 0.8, 0.8, 1.0),
            0,
        );
    }

    fn gui(&mut self, api: &GuiParams) {
        api.label("Hello World!");
    }
}

register_plugin!(
    "Hello World from Rust",
    "185d4603-e803-4fa2-9f56-266782cf2e98",
    State
);
