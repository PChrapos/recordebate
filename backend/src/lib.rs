use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use config::get_config;
use postprocess::PostProcess;
use record_models::RecordModels;

mod api;
mod config;
mod postprocess;
mod record_models;
mod video;

pub fn main_loop(update_models_every_seconds: u64, postprocess_every_seconds: u64) {
    let mut record_models = RecordModels::new();
    let mut postprocess = PostProcess::new();

    let mut record_models_timer: u64 = update_models_every_seconds;
    let mut postprocess_timer: u64 = postprocess_every_seconds;

    let mut start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut config = get_config().unwrap();
    loop {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let time_diff = cur_time - start_time;
        start_time = cur_time;
        record_models_timer += time_diff;
        postprocess_timer += time_diff;

        if postprocess_timer >= postprocess_every_seconds {
            postprocess
                .process_all(&record_models.get_online_models(), config.remux)
                .ok();
            postprocess_timer -= postprocess_every_seconds;
        }
        if record_models_timer >= update_models_every_seconds {
            config = get_config().unwrap();
            record_models.update(&config);
            record_models_timer -= update_models_every_seconds;
        }

        thread::sleep(Duration::from_secs(5));
    }
}
