# oden_plugin_rs

Rust wrapper for the [Oden](https://oden.voysys.dev) plugin C API. Write Oden
plugins in Rust against a safe, idiomatic API.

API documentation: <https://voysys.github.io/oden_plugin_rs/>

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

## License

Licensed under the [0BSD](https://opensource.org/licenses/0BSD) license. See
[LICENSE](LICENSE).
