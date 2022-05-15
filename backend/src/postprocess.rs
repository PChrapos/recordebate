use std::{
    error::Error,
    fs,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::video::{convert_to_mp4, create_thumbnail};

pub struct PostProcess {
    tx: Sender<String>,
    rx: Receiver<String>,
    current_videos_postprocessing: Vec<String>,
}

impl Default for PostProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl PostProcess {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            rx,
            current_videos_postprocessing: vec![],
        }
    }

    pub fn process_all(
        &mut self,
        ignored_models: &[&String],
        remux: bool,
    ) -> Result<(), Box<dyn Error>> {
        while let Ok(done_file) = self.rx.try_recv() {
            let index = self
                .current_videos_postprocessing
                .iter()
                .position(|file| *file == done_file)
                .unwrap();
            self.current_videos_postprocessing.remove(index);
            println!("done postprocessing {done_file}");
        }

        let dir_entries = fs::read_dir("../data")?;
        for dir in dir_entries {
            let dir = dir?;
            if !dir.file_type()?.is_dir() {
                continue;
            }
            if ignored_models.contains(&&dir.file_name().to_str().unwrap().to_string()) {
                continue;
            }
            let entries = fs::read_dir(dir.path())?;
            for entry in entries {
                let entry = entry?;
                if !entry.file_type()?.is_file() {
                    continue;
                }
                let entry_path = entry.path();
                let input_file = entry_path.to_str().unwrap().to_string();
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".mp4") && !entry_path.with_extension("png").exists() {
                    println!("creating thumbnail for {input_file}");
                    create_thumbnail(entry_path.to_str().unwrap()).ok();
                }
                if !file_name.ends_with(".ts") {
                    continue;
                }
                if self.current_videos_postprocessing.contains(&input_file) {
                    continue;
                }
                self.current_videos_postprocessing
                    .push(input_file.to_string());
                println!("post processing {input_file}");
                let tx = self.tx.clone();
                thread::spawn(move || {
                    create_thumbnail(&input_file).ok();
                    convert_to_mp4(&input_file, remux).ok();
                    tx.send(input_file).ok();
                });
            }
        }
        Ok(())
    }
}
