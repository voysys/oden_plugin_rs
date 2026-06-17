use std::time::Instant;

use oden_plugin_rs::{
    BusClient, GuiParams, InitParams, OdenPlugin, ShutdownParams, Subscription, UpdateParams, log,
    register_plugin,
};
use serde_json::json;
use webview_shared::WEBVIEW_UUID;

const WEBVIEW_BUS_UUID: oden_plugin_rs::math::Uuid =
    oden_plugin_rs::math::Uuid::parse(WEBVIEW_UUID);

struct State {
    start_time: Instant,
    bus_client: BusClient,
    subscription: Subscription,
}

impl OdenPlugin for State {
    fn init(api: &InitParams) -> Self {
        let bus_client = BusClient::open(api, WEBVIEW_BUS_UUID);
        let subscription = bus_client.subscribe(&["example_from_webview"]);
        State {
            start_time: Instant::now(),
            bus_client,
            subscription,
        }
    }

    fn shutdown(self, _api: &ShutdownParams) {}

    fn update(&mut self, _api: &UpdateParams) {
        while let Some(msg) = self.subscription.try_recv() {
            log::info!(
                "Webview message: {} {:?}",
                msg.name,
                String::from_utf8_lossy(&msg.payload)
            );
        }
    }

    fn gui(&mut self, api: &GuiParams) {
        if api.button("Send Test Message") {
            let duration = self.start_time.elapsed().as_millis();

            let payload =
                serde_json::to_vec(&json!({"time_since_start_ms": duration})).unwrap_or_default();

            log::info!("Sending example_to_webview ({} ms)", duration);
            self.bus_client.send("example_to_webview", &payload).ok();
        }
    }
}

register_plugin!(
    "Webview Messaging Example",
    "c9644797-0eeb-4083-9ab9-abfe10a07625",
    State
);
