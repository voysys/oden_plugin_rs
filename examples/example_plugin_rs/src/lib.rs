use oden_plugin_rs::{
    async_com_channel_api::{
        async_com_channel_add_message_to_send_queue, async_com_channel_flush_message_queue,
        AsyncReceiveAB,
    },
    glam,
    log::{error, info},
    math::{Vec2, Vec2i, Vec3},
    plugin_h::OdenRotation,
    register_plugin, zerocopy,
    zerocopy::{AsBytes, FromBytes},
    GuiParams, InitParams, LinkError, OdenPlugin, ShutdownParams, UpdateParams,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

fn link_stats_example(api: &GuiParams, entity: Option<&str>, index: i32) -> Result<(), LinkError> {
    let link_stats = api.link_stats_bytes_transferred(entity, index)?;

    api.label(&format!(
        "Recv: {}, sent: {}",
        link_stats.bytesReceived, link_stats.bytesSent,
    ));

    Ok(())
}

fn network_examples(api: &GuiParams, entity: Option<&str>, index: i32) -> Result<(), LinkError> {
    let mut p2p_enabled = api.link_p2p_enabled(entity, index)?;
    if api.checkbox("Enabled", &mut p2p_enabled) {
        api.set_link_p2p_enabled(entity, index, p2p_enabled)?;
    }

    let p2p_status = api.link_p2p_status(entity, index)?;
    api.label(&p2p_status.to_string());

    let mut p2p_stun_server = api.link_p2p_stun_server(entity, index)?;
    if api.input_text("STUN Server", &mut p2p_stun_server) {
        api.set_link_p2p_stun_server(entity, index, &p2p_stun_server)?;
    }

    Ok(())
}

#[derive(AsBytes, FromBytes, Debug)]
#[repr(C)]
struct LargeDataTest {
    data: [u8; 3072],
}

impl Default for LargeDataTest {
    fn default() -> Self {
        Self { data: [0; 3072] }
    }
}

#[derive(Default, AsBytes, FromBytes, Debug)]
#[repr(C)]
struct HelloMessageA {
    data: u32,
    data2: LargeDataTest,
}

#[derive(Default, AsBytes, FromBytes, Debug)]
#[repr(C)]
struct HelloMessageB {
    data: u32,
    data2: u32,
}

#[repr(C)]
#[derive(Debug, AsBytes)]
struct GpsData {
    sat_in_use: u32,
    speed_mps: f32,
    latitude: f64,
    longitude: f64,
}

struct AsyncTestStorage {
    message_a: HelloMessageA,
    message_b: HelloMessageB,
    last_received_timestamp: Instant,
}

fn async_thread_fn(
    run: Arc<AtomicBool>,
    run_async_test: Arc<AtomicBool>,
    receiver: Arc<AtomicBool>,
    async_storage: Arc<Mutex<AsyncTestStorage>>,
) {
    let mut message_counter_a = 0;
    let mut message_counter_b = 10000;

    while run.load(Ordering::SeqCst) {
        if run_async_test.load(Ordering::SeqCst) {
            if receiver.load(Ordering::SeqCst) {
                match AsyncReceiveAB::<HelloMessageA, HelloMessageB>::async_com_channel_receive(
                    "HelloMessageA",
                    "HelloMessageB",
                    Duration::from_millis(100),
                ) {
                    Ok(AsyncReceiveAB::A(message)) => {
                        let mut async_storage = async_storage.lock().unwrap();
                        async_storage.last_received_timestamp = Instant::now();
                        async_storage.message_a = message;
                    }
                    Ok(AsyncReceiveAB::B(message)) => {
                        let mut async_storage = async_storage.lock().unwrap();
                        async_storage.last_received_timestamp = Instant::now();
                        async_storage.message_b = message;
                    }
                    _ => (),
                }
            } else {
                let message_a = HelloMessageA {
                    data: message_counter_a,
                    data2: Default::default(),
                };

                let message_b = HelloMessageB {
                    data: message_counter_a,
                    data2: message_counter_b,
                };

                let gps_data = GpsData {
                    sat_in_use: 125,
                    speed_mps: 32.0,
                    latitude: 123.0,
                    longitude: 0.5,
                };

                message_counter_a = message_counter_a.wrapping_add(1);
                message_counter_b = message_counter_b.wrapping_add(1);

                async_com_channel_add_message_to_send_queue("HelloMessageA", &message_a);
                async_com_channel_add_message_to_send_queue("HelloMessageB", &message_b);

                async_com_channel_add_message_to_send_queue("GPS", &gps_data);

                async_com_channel_flush_message_queue();

                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

struct State {
    show_img: bool,
    run: Arc<AtomicBool>,
    run_async_test: Arc<AtomicBool>,
    async_storage: Arc<Mutex<AsyncTestStorage>>,
    receiver: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
    reset_view: bool,
    selected_entity: String,
}

impl OdenPlugin for State {
    fn init(api: &InitParams) -> Self {
        let oden_img = include_bytes!("../oden.png");

        let oden_img = image::load_from_memory(oden_img)
            .map(|img| img.into_rgba8())
            .unwrap();

        api.upload_image(1, 256, 256, &oden_img);

        let run = Arc::new(AtomicBool::new(true));
        let run_async_test = Arc::new(AtomicBool::new(true));
        let receiver = Arc::new(AtomicBool::new(true));
        let async_storage = Arc::new(Mutex::new(AsyncTestStorage {
            message_a: HelloMessageA::default(),
            message_b: HelloMessageB::default(),
            last_received_timestamp: Instant::now(),
        }));

        let thread_handle = {
            let run = run.clone();
            let run_async_test = run_async_test.clone();
            let receiver = receiver.clone();
            let async_storage = async_storage.clone();

            Some(
                thread::Builder::new()
                    .name("Async Thread".to_string())
                    .spawn(move || async_thread_fn(run, run_async_test, receiver, async_storage))
                    .unwrap(),
            )
        };

        State {
            show_img: true,
            run,
            run_async_test,
            receiver,
            async_storage,
            thread_handle,
            reset_view: false,
            selected_entity: String::new(),
        }
    }

    fn shutdown(self, _api: &ShutdownParams) {}

    fn update(&mut self, api: &UpdateParams) {
        {
            let mat = glam::Mat4::from_rotation_z(-2.570_796_3);

            api.im_draw_push_matrix(mat.into());

            if self.show_img {
                api.im_draw_add_image(Vec3::xyz(0.0, 0.0, -2.0), Vec2::xy(1.0, 1.0), 1);
            }

            api.im_draw_pop_matrix();
        }

        if self.reset_view {
            api.reset_view();
            self.reset_view = false;
        }
    }

    fn gui(&mut self, api: &GuiParams) {
        if let Some(_guard) = api.tree_node("Update Functions") {
            if api.button("Reset View") {
                self.reset_view = true;
            }
        }

        if let Some(_tree) = api.tree_node("Streamer Functions") {
            api.input_text("Entity", &mut self.selected_entity);

            if api.button("Adjust Alignment Pos") {
                let pos = Vec2i::xy(50, 50);
                if let Err(err) = api.set_output_alignment_position(&self.selected_entity, pos) {
                    error!("{err:?}");
                }
            }

            if api.button("Adjust Alignment Scale") {
                let scale = 0.5;
                if let Err(err) = api.set_output_alignment_scale(&self.selected_entity, scale) {
                    error!("{err:?}");
                }
            }

            if api.button("Adjust Alignment Rot") {
                let rot = OdenRotation::OdenRotation90;
                if let Err(err) = api.set_output_alignment_rotation(&self.selected_entity, rot) {
                    error!("{err:?}");
                }
            }
        }

        if let Some(_guard) = api.tree_node("Scene Functions") {
            api.label("Raw Recording:");

            if api.raw_record_is_running() {
                if api.button("Stop Raw Record") {
                    api.raw_record_stop()
                }
            } else if api.button("Start Raw Record")
                && api.set_raw_record_entity_ring_buffer_size("My Video Entity Name", 2048)
                && api.raw_record_entity_ring_buffer_enable("My Video Entity Name")
            {
                api.raw_record_start()
            }

            if let Some(id) = api.selected_scene_id() {
                api.label(&format!("Current Scene ID: {id}"));

                if api.button("Export Current Scene Full") {
                    if let Ok(scene_string) = api.scene_to_string(Some(id), true) {
                        info!("{scene_string}");
                    }
                }

                if api.button("Export Current Scene Minimal") {
                    if let Ok(scene_string) = api.scene_to_string(Some(id), false) {
                        info!("{scene_string}");
                    }
                }
            }

            {
                let scene_ids = api.all_scene_ids();

                api.label("Available Scenes:");
                for id in scene_ids {
                    if let Some(name) = api.scene_name(id) {
                        api.label(&format!("{name} (id: {id})"));
                        api.same_line();
                        if api.button(&format!("Go To {name}!")) && !api.set_selected_scene(id) {
                            error!("Could not set scene");
                        }
                    }
                }
            }
        }

        if let Some(_guard) = api.tree_node("Entity Functions") {
            for (i, entity_uuid) in api
                .all_entity_uuids()
                .unwrap_or_default()
                .into_iter()
                .enumerate()
            {
                let name = api
                    .entity_name(&entity_uuid.to_string())
                    .unwrap_or_default();
                let enabled = api
                    .entity_enabled(&entity_uuid.to_string())
                    .unwrap_or_default();

                api.label(&format!("Entity {i}: Name: {name}, Enabled: {enabled}"));
            }
        }

        if let Some(_guard) = api.indent(40.0) {
            api.label("Indented label");
        }

        if let Some(_guard) = api.tree_node("Network Functions") {
            let mut remote_streamer_entity = api
                .read_string("remote_streamer_entity_id")
                .unwrap_or_default();

            if api.input_text("Entity ID", &mut remote_streamer_entity) {
                api.write_string("remote_streamer_entity_id", &remote_streamer_entity);
            }

            let remote_streamer_entity = if remote_streamer_entity.is_empty() {
                None
            } else {
                Some(remote_streamer_entity)
            };

            if api.button("Add Link") {
                if let Err(err) = api.add_link(remote_streamer_entity.as_deref()) {
                    error!("Error adding link: {:?}", err);
                }
            }

            if let Ok(link_count) = api.link_count(remote_streamer_entity.as_deref()) {
                let links: Vec<String> = (0..link_count).map(|i| i.to_string()).collect();
                let links: Vec<&str> = links.iter().map(|s| &**s).collect();

                let mut current_item = 0;
                if api.combo("Remove Link", &mut current_item, &links) {
                    if let Err(err) =
                        api.remove_link(remote_streamer_entity.as_deref(), current_item)
                    {
                        error!("Error removing link with index {current_item}: {err:?}");
                    }
                }

                if link_count > 0 {
                    if let Some(_tree) = api.tree_node("P2P") {
                        if let Err(err) =
                            network_examples(api, remote_streamer_entity.as_deref(), 0)
                        {
                            api.label(&format!("{err:?}"));
                        }
                    }

                    if let Some(_tree) = api.tree_node("Link Stats") {
                        if let Err(err) =
                            link_stats_example(api, remote_streamer_entity.as_deref(), 0)
                        {
                            api.label(&format!("{err:?}"));
                        }
                    }
                }
            }
        }

        if let Some(_guard) = api.tree_node("Async Test") {
            {
                let mut run_async_test = self.run_async_test.load(Ordering::SeqCst);
                if api.checkbox("Run", &mut run_async_test) {
                    self.run_async_test.store(run_async_test, Ordering::SeqCst);
                }
            }

            {
                let mut receiver = self.receiver.load(Ordering::SeqCst);
                if api.checkbox("Receiver", &mut receiver) {
                    self.receiver.store(receiver, Ordering::SeqCst);
                }

                if self.receiver.load(Ordering::SeqCst) {
                    let async_storage = self.async_storage.lock().unwrap();

                    api.label(&format!(
                        "Time since last message: {} ms",
                        async_storage.last_received_timestamp.elapsed().as_millis(),
                    ));

                    api.label(&format!("{:?}", async_storage.message_a));
                    api.label(&format!("{:?}", async_storage.message_b));
                }
            }
        }

        if api.button("Toggle test img") {
            self.show_img = !self.show_img;
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.run.store(false, Ordering::SeqCst);

        if let Some(thread) = self.thread_handle.take() {
            thread.join().unwrap();
        }
    }
}

register_plugin!(
    "Example Rust",
    "25d0135f-51e3-418f-a049-2ae36f87905b",
    State
);
