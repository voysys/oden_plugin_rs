use csv::{self, Writer};
use oden_plugin_rs::{
    log::error, register_plugin, GuiParams, InitParams, OdenPlugin, ShutdownParams, UpdateParams,
};
use oden_plugin_rs_derive::Properties;
use serde::Serialize;
use std::{error::Error, fs::File, ops::Add, path::Path, time::Instant};

fn create_writer(
    save_path_in: &str,
    file_name: &str,
    project_path: Option<String>,
) -> Option<csv::Writer<File>> {
    let save_path = if save_path_in.is_empty() {
        if let Some(path) = project_path {
            path
        } else {
            "".to_string()
        }
    } else {
        save_path_in.to_string()
    };
    match csv::Writer::from_path(
        &Path::new(&format!(
            "{save_path}/{file_name}_{}",
            chrono::Utc::now().to_rfc3339().replace(':', "-")
        ))
        .with_extension("csv"),
    ) {
        Ok(writer) => Some(writer),
        Err(err) => {
            error!("Cannot create writer: {err}");
            None
        }
    }
}

#[derive(Serialize)]
struct OdenBasicStatisticsDef {
    current_time_s: f64,
    channel_usage_fraction: f32,
    packet_loss_count: f32,
    kb_in_flight: f32,
    bandwidth_megabit: f32,
    round_trip_time_ms: f32,
}

fn write_basic_stats(
    remote_streamer_entity: &str,
    writer: &mut Writer<File>,
    api: &UpdateParams,
) -> Result<(), Box<dyn Error>> {
    let stats = api
        .basic_statistics(remote_streamer_entity)
        .ok_or("Can't find remote streamer entity")?;

    writer.serialize(OdenBasicStatisticsDef {
        current_time_s: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("!")
            .as_millis() as f64
            / 1000.0,
        channel_usage_fraction: stats.channelUsage,
        packet_loss_count: stats.packetLoss,
        kb_in_flight: stats.kiloBytesInFlight,
        bandwidth_megabit: stats.bandwidthMbit,
        round_trip_time_ms: stats.roundTripTimeMs,
    })?;

    writer.flush()?;

    Ok(())
}

#[derive(Serialize)]
struct OdenPerFrameStatisticsDef {
    frame_emit_time_ns: i64,
    channel_usage_fraction: f32,
    lost_packets: i32,
    frame_number_wrapping: i32,
    first_packet_time_ns: i64,
    last_packet_time_ns: i64,
}

fn write_extended_stats(
    stats: &[oden_plugin_rs::plugin_h::OdenStreamStatistics],
    writer: &mut Writer<File>,
) -> Result<(), Box<dyn Error>> {
    for stat in stats {
        let stat = OdenPerFrameStatisticsDef {
            frame_emit_time_ns: stat.frameEmitTime,
            channel_usage_fraction: stat.channelUsage,
            lost_packets: stat.lostPackets,
            frame_number_wrapping: stat.frameNo,
            first_packet_time_ns: stat.firstPacketTime,
            last_packet_time_ns: stat.lastPacketTime,
        };
        writer.serialize(stat)?;
    }

    writer.flush()?;

    Ok(())
}

#[derive(Properties)]
struct State {
    #[property(default = false)]
    basic_stats_active: bool,

    #[property(default = false)]
    extended_stats_active: bool,

    #[property(default = "")]
    save_path: String,

    #[property(default = "Streamer")]
    remote_streamer_entity_name: String,

    basic_stats_writer: Option<csv::Writer<File>>,
    timer: Instant,

    extended_stats_writer: Option<csv::Writer<File>>,
    last_extended_frame_emit_time: i64,
}

impl OdenPlugin for State {
    fn init(_api: &InitParams) -> Self {
        State {
            basic_stats_active: false,
            extended_stats_active: false,
            save_path: "".to_string(),
            remote_streamer_entity_name: "Streamer".to_string(),
            basic_stats_writer: None,
            timer: Instant::now(),
            last_extended_frame_emit_time: -1,
            extended_stats_writer: None,
        }
    }

    fn shutdown(self, _api: &ShutdownParams) {}

    fn update(&mut self, api: &UpdateParams) {
        let interval = std::time::Duration::from_millis(100);
        if self.timer.elapsed() > interval {
            while self.timer.elapsed() > interval {
                self.timer = self.timer.add(interval);

                if self.basic_stats_active {
                    if self.basic_stats_writer.is_none() {
                        self.basic_stats_writer =
                            create_writer(&self.save_path, "statistics_basic", api.project_path());
                    }

                    if let Some(basic_stats_writer) = &mut self.basic_stats_writer {
                        match write_basic_stats(
                            &self.remote_streamer_entity_name,
                            basic_stats_writer,
                            api,
                        ) {
                            Ok(_) => {}
                            Err(err) => error!("Error writing stats to csv: {err}"),
                        }
                    }
                } else {
                    self.basic_stats_writer = None;
                }
            }

            if self.extended_stats_active {
                if self.extended_stats_writer.is_none() {
                    self.extended_stats_writer =
                        create_writer(&self.save_path, "statistics_per_frame", api.project_path());
                }

                if let Some(extended_stats_writer) = &mut self.extended_stats_writer {
                    if let Some(stats) =
                        api.remote_streamer_statistics(&self.remote_streamer_entity_name)
                    {
                        if !stats.is_empty() {
                            let mut start_index = 0;
                            for (i, stream_stats) in stats.iter().enumerate() {
                                if stream_stats.frameEmitTime == self.last_extended_frame_emit_time
                                {
                                    start_index = i + 1;
                                    break;
                                }
                            }

                            if start_index != stats.len() {
                                self.last_extended_frame_emit_time =
                                    stats.last().unwrap().frameEmitTime;

                                match write_extended_stats(
                                    &stats[start_index..],
                                    extended_stats_writer,
                                ) {
                                    Ok(_) => {}
                                    Err(err) => error!("Error writing stats to csv: {err}"),
                                }
                            }
                        }
                    }
                }
            } else {
                self.extended_stats_writer = None;
            }
        }
    }

    fn gui(&mut self, _api: &GuiParams) {}
}

register_plugin!(
    "Statistics Logger",
    "5f74d482-930b-4284-a11f-8ecf11b807bd",
    State
);
