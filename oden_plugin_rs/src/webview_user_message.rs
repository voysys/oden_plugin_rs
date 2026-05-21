//! Message passing between plugins and JavaScript in Webviews

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Message passing between plugins and JavaScript in Webviews
///
/// # Example Usage (Rust side):
/// ```
/// use oden_plugin_rs::webview_user_message::WebviewUserMessage;
/// use serde_json::json;
///
/// // Create a sample message
/// let message = WebviewUserMessage {
///     name: "vehicle_state".to_string(),
///     payload: Some(json!({"speed": 65, "fuel": 75})),
/// };
///
/// // In actual code, you would send this message via a channel
/// // For example, if you have a sender from named_mpmc_channel:
/// // sender.try_send(message).ok();
/// ```
///
/// # Webview side, this example uses React:
/// ```javascript
/// useEffect(() => {
///     const layoutClient = window.odenLayoutClient;
///
///     const handleMessage = (message) => {
///         console.log(message);
///     };
///
///     layoutClient.registerUserMessageCallback("vehicle_state", handleMessage);
///
///     return () => {
///         layoutClient.unregisterUserMessageCallback("vehicle_state", handleMessage);
///     };
/// }, []);
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WebviewUserMessage {
    /// The name identifier of the message channel.
    pub name: String,

    /// The JSON payload containing the message data.
    pub payload: Option<Value>,
}
