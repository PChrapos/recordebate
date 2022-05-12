use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use crate::{
    api::{download_stream, is_model_online},
    config::Config,
};

fn record_model(name: &str, tx: Sender<String>, rx_stop: &Receiver<()>, auto_convert: bool) {
    let save_folder = format!("../data/{name}");
    let result = download_stream(name, &save_folder, rx_stop, auto_convert);
    if let Err(err) = result {
        println!("Error while recording model {name}: {err:?}");
    }
    tx.send(name.to_string()).unwrap();
}

fn get_offline_models(config: &Config, online_models: &[&String]) -> Vec<String> {
    config
        .tracked_models
        .iter()
        .filter(|name| {
            !online_models
                .iter()
                .any(|online_model| online_model == name)
        })
        .map(|x| x.to_owned())
        .collect()
}

pub struct RecordModels {
    online_models: Vec<(String, JoinHandle<()>, Sender<()>)>,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl Default for RecordModels {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordModels {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            online_models: vec![],
            tx,
            rx,
        }
    }

    pub fn update(&mut self, config: &Config) {
        let offline_models = get_offline_models(config, &self.get_online_models());
        let mut new_online_models: Vec<(String, JoinHandle<()>, Sender<()>)> = offline_models
            .into_iter()
            .filter(|name| is_model_online(name))
            .map(|name| {
                let model_name = name.clone();
                let tx = self.tx.clone();
                let (tx_stop, rx_stop) = channel();
                let auto_convert = config.remux;
                let handle =
                    thread::spawn(move || record_model(&model_name, tx, &rx_stop, auto_convert));
                (name, handle, tx_stop)
            })
            .collect();
        for (new_online_model, _, _) in &new_online_models {
            println!("{new_online_model} is online");
        }
        self.online_models.append(&mut new_online_models);

        self.online_models.retain(|(online_model, _, tx_stop)| {
            if config.tracked_models.contains(online_model) {
                return true;
            }
            tx_stop.send(()).unwrap();
            println!("removed model {online_model}");
            false
        });

        while let Ok(offline_model) = self.rx.try_recv() {
            let index = self
                .online_models
                .iter()
                .position(|(name, _, _)| *name == offline_model);
            if let Some(index) = index {
                let (_, offline_model_handle, _) = self.online_models.remove(index);
                offline_model_handle.join().unwrap();
                println!("{offline_model} is offline");
            }
        }
    }

    pub fn get_online_models(&self) -> Vec<&String> {
        self.online_models.iter().map(|x| &x.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_offline_models() {
        let config = Config {
            tracked_models: ["model1", "model2", "model3"]
                .into_iter()
                .map(String::from)
                .collect(),
            ..Default::default()
        };

        let online_models = ["model2".to_string()];
        let online_models: Vec<&String> = online_models.iter().collect();

        let result = get_offline_models(&config, &online_models);

        assert_eq!(result, ["model1", "model3"])
    }
}
