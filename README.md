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
| `webview_user_message` | Legacy webview user-message JSON types; superseded by the message bus |
| `strum_ex` | `strum` helpers for generated enums |
| `mock` | `mockall` mocks of the API traits, for testing plugins |
| `cuda` | CUDA-compatible layouts for `glam`/`nalgebra` |

## Webview messaging

Plugins exchange named messages with the Oden WebView plugin through the
message bus in `oden_plugin_rs::message_bus` (`BusClient`, `Subscription`, and
friends are re-exported at the crate root). The WebView plugin owns the bus and
publishes it under this owner UUID:

```rust
use oden_plugin_rs::math::Uuid;

const WEBVIEW_UUID: Uuid = Uuid::parse("f6599e51-0ea3-46c1-9907-0e7d0334a807");
```

`Uuid::parse` is a `const fn`, so a mistyped literal fails at compile time.

`BusClient::open()` connects to the bus, creating it if no other plugin has
dialed that UUID yet, so you can open it once in `init()` and subscribe right
away — messages broadcast before the WebView plugin starts simply wait for it:

```rust
fn init(api: &InitParams) -> Self {
    let bus_client = BusClient::open(api, WEBVIEW_UUID);
    let subscription = bus_client.subscribe(&["my_message_from_webview"]);
    Self { bus_client, subscription }
}

fn update(&mut self, api: &UpdateParams) {
    while let Some(msg) = self.subscription.try_recv() {
        log::info!("{}: {}", msg.name, String::from_utf8_lossy(&msg.payload));
    }
}
```

Subscriptions receive the messages a web page sends with
`sendNamedUserMessage(name, payload)`, matched by exact name.
`BusClient::send(name, payload)` goes the other way: the WebView plugin
forwards the payload to JavaScript callbacks registered with
`registerUserMessageCallback(name, cb)`. Payloads are opaque bytes on the bus,
but messages sent toward the webview must be valid JSON (or empty) — the
WebView plugin discards anything else. Besides `try_recv()`,
subscriptions offer `recv_timeout(Duration)` and `recv_async()`.

The bus outlives every plugin: when the WebView plugin restarts or
hot-reloads, your `BusClient` and `Subscription` stay valid — subscriptions
keep receiving once the new owner is up, and sends made in the meantime wait
in a bounded queue. There is no liveness state to poll and no reconnect logic
to write.

## License

Licensed under the [0BSD](https://opensource.org/licenses/0BSD) license. See
[LICENSE](LICENSE).
