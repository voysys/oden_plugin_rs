# Webview Messaging Example

This is an example of how to do two-way messaging with JavaScript on a website running in the WebView.

## Building

Rust Plugin: `cargo build`

React app: Go to `example_react` and run `npm install`.

## Running

- Go to `example_react` and run `npm run dev`
- Copy the build artifact under `target/debug/` or `target/release` to the OdenVR directory.
- Run OdenVR and go to `localhost:3000` in the WebView and open `example.vproj`.
