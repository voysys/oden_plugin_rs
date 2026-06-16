use std::time::Instant;

use oden_plugin_rs::{
    log,
    named_mpmc_channel::{NamedMpmcChannelExt, Receiver, Sender},
    register_plugin,
    webview_user_message::WebviewUserMessage,
    GuiParams, InitParams, OdenPlugin, ShutdownParams, UpdateParams,
};
use serde_json::json;

struct State {
    start_time: Instant,
    sender: Sender<WebviewUserMessage>,
    receiver: Receiver<WebviewUserMessage>,
}

impl OdenPlugin for State {
    fn init(api: &InitParams) -> Self {
        State {
            start_time: Instant::now(),
            sender: api.named_mpmc_channel_tx("user_message_oden_to_webview", 128),
            receiver: api.named_mpmc_channel_rx("user_message_webview_to_oden", 128),
        }
    }

    fn shutdown(self, _api: &ShutdownParams) {}

    fn update(&mut self, _api: &UpdateParams) {
        while let Ok(webview_message) = self.receiver.try_recv() {
            log::info!("Webview Message: {webview_message:?}");
        }
    }

    fn gui(&mut self, api: &GuiParams) {
        if api.button("Send Test Message") {
            let duration = self.start_time.elapsed().as_millis();

            let message = WebviewUserMessage {
                name: "test_message".to_string(),
                payload: Some(json!({"time_since_start_ms": duration})),
            };

            log::info!("Sending: {message:?}");

            if let Err(err) = self.sender.try_send(message) {
                log::error!("Failed to send test message: {err}");
            }
        }
    }
}

register_plugin!(
    "Webview Messaging Example",
    "c9644797-0eeb-4083-9ab9-abfe10a07625",
    State
);
