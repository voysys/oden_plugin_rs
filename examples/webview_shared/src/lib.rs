//! Constants shared between the webview plugin and other plugins.

/// UUID of the webview plugin. It registers its global entity under this UUID
/// and publishes its message bus under it; clients pass it to
/// `oden_plugin_rs::BusClient::open`.
pub const WEBVIEW_UUID: &str = "f6599e51-0ea3-46c1-9907-0e7d0334a807";
