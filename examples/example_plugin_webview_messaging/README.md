# Webview Messaging Example

This is an example of round-tripping a message between an Oden plugin and JavaScript
running in the WebView, on top of the generic `oden_plugin_rs::message_bus` primitive.

The webview plugin (UUID `f6599e51-0ea3-46c1-9907-0e7d0334a807`, exposed as
`webview_shared::WEBVIEW_UUID`) owns the bus. This example plugin opens it
as a client, subscribes to one downstream message, and sends one upstream message:

- **Plugin → WebView**: `example_to_webview` carries `{"time_since_start_ms": <ms>}`.
  Triggered by the "Send Test Message" button in the plugin GUI.
- **WebView → Plugin**: `example_from_webview` carries `{"ts": <Date.now()>}`.
  Triggered by the "Send example_from_webview" button in the React demo. The plugin
  logs the message via `log::info!`.

## Building

Rust Plugin: `cargo build`

React app: Go to `../webview/example_react` and run `npm install`.

## Running

- Go to `../webview/example_react` and run `npm run dev`.
- Copy the build artifact under `target/debug/` or `target/release` into the OdenVR
  directory.
- Run OdenVR and point the WebView plugin at `localhost:3000`.
