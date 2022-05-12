use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{create_dir_all, File, OpenOptions},
    io::Read,
    path::Path,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    pub tracked_models: Vec<String>,
    pub remux: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            tracked_models: vec![],
            remux: true,
        }
    }

    pub fn write(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)?;
        serde_json::to_writer(&file, self)?;
        Ok(())
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_file = "../config/config.json";
    let config_path = Path::new(config_file);
    if !config_path.exists() {
        let parent_dir = config_path.parent().unwrap();
        create_dir_all(parent_dir)?;
        Config::new().write(config_file)?;
    }
    let mut file = File::open(config_file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}
