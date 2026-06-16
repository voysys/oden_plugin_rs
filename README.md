# oden_plugin_rs

Rust wrapper for the [Oden](https://docs.voysys.dev) plugin C API. Write Oden
plugins in Rust against a safe, idiomatic API.

API documentation:
<https://voysys.github.io/oden_plugin_rs/latest/oden_plugin_rs/index.html>
(`/latest/` always points at the newest release; older versions live under
`/<version>/`).

## Usage

Add the crate to your plugin's `Cargo.toml`. Pin a release tag for a stable
build, or track `master` for the latest:

```toml
# Pinned to a release
oden_plugin_rs = { git = "https://github.com/voysys/oden_plugin_rs", tag = "v48.0" }

# Latest
oden_plugin_rs = { git = "https://github.com/voysys/oden_plugin_rs", branch = "master" }
```

The `Properties` derive macro lives in the companion `oden_plugin_rs-derive`
crate in this repository:

```toml
oden_plugin_rs-derive = { git = "https://github.com/voysys/oden_plugin_rs", tag = "v48.0" }
```

## Quick start

A plugin implements the [`OdenPlugin`] trait and registers itself with
`register_plugin!`. The API methods are inherent methods on the `*Params` value
passed to each callback, so you call them directly on `api` — you do not need to
import the `*Api` traits.

```rust
use oden_plugin_rs::{
    math::{Vec3, Vec4},
    register_plugin, GuiParams, InitParams, OdenPlugin, ShutdownParams, UpdateParams,
};

struct State;

impl OdenPlugin for State {
    fn init(_api: &InitParams) -> Self {
        State
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
```

See the [`hello_world_rs`](examples/hello_world_rs) example for the full crate,
and [`examples/`](examples) for more.

[`OdenPlugin`]: https://voysys.github.io/oden_plugin_rs/latest/oden_plugin_rs/trait.OdenPlugin.html

## Features

| Feature | Description |
| --- | --- |
| `glam_conversion` | Conversions to/from `glam` types |
| `nalgebra_conversion` | Conversions to/from `nalgebra` types |
| `glam_nalgebra` | Both of the above plus cross conversions |
| `serialize` | `serde` `Serialize`/`Deserialize` for API types |
| `oden_egui` | `egui` integration (OpenGL painter via `glow`) |
| `webview_user_message` | Webview user-message helpers |
| `strum_ex` | `strum` helpers for generated enums |
| `mock` | `mockall` mocks of the API traits, for testing plugins |
| `cuda` | CUDA-compatible layouts for `glam`/`nalgebra` |

## Examples

The [`examples/`](examples) folder contains complete plugins you can build and
load in Oden:

| Example | Demonstrates |
| --- | --- |
| [`hello_world_rs`](examples/hello_world_rs) | The smallest possible plugin |
| [`example_plugin_rs`](examples/example_plugin_rs) | Properties, rendering and image handling |
| [`example_plugin_webview_messaging`](examples/example_plugin_webview_messaging) | Webview user-message passing with a React frontend |
| [`statistics_logger`](examples/statistics_logger) | Logging statistics to CSV with the `Properties` derive |

Each example is its own standalone crate that depends on `oden_plugin_rs` by
relative path, so they build straight from a checkout of this repository.

## License

Licensed under the [0BSD](https://opensource.org/licenses/0BSD) license. See
[LICENSE](LICENSE).
