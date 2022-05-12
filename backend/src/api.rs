use std::{
    error::Error,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::mpsc::Receiver,
    thread,
    time::Duration,
};

use chrono::Local;
use m3u8_rs::Playlist;
use serde::Deserialize;

use crate::video::convert_to_mp4;

pub fn is_model_online(name: &str) -> bool {
    let request_url = format!("https://roomimg.stream.highwebmedia.com/stream?room={name}");
    let res = reqwest::blocking::get(request_url);
    if let Ok(res) = res {
        res.status().as_u16() == 200
    } else {
        false
    }
}

pub fn download_stream(
    name: &str,
    save_folder: &str,
    rx_stop: &Receiver<()>,
    auto_convert: bool,
) -> Result<(), Box<dyn Error>> {
    let date = Local::now();
    let mut path = PathBuf::from(save_folder);
    path.push(date.format("%Y_%m_%d_%H_%M_%S.ts").to_string());
    create_dir_all(path.parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;

    let mut downloaded_urls: Vec<String> = vec![];

    let mut chunklist = String::new();
    while is_model_online(name) {
        if let Ok(()) = rx_stop.try_recv() {
            break;
        }
        let segments = if let Ok(segments) = get_media_segments(&chunklist) {
            segments
        } else {
            chunklist = get_chunklist(name)?;
            continue;
        };
        let new_segments: Vec<String> = segments
            .into_iter()
            .filter(|x| !downloaded_urls.contains(x))
            .collect();
        for segment in new_segments {
            let data = reqwest::blocking::get(&segment)?.bytes()?;
            file.write_all(&data)?;
            downloaded_urls.push(segment);
        }
        thread::sleep(Duration::from_secs(1));
    }
    if auto_convert {
        convert_to_mp4(path.to_str().unwrap(), true).ok();
    }

    Ok(())
}

#[derive(Deserialize)]
struct GetEdgeHlsUrlAjax {
    url: String,
}

fn get_playlist_url(name: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let params = [("room_slug", name), ("bandwidth", "high")];
    let res = client
        .post("https://chaturbate.com/get_edge_hls_url_ajax/")
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&params)
        .send()?;
    let result: GetEdgeHlsUrlAjax = res.json()?;
    Ok(result.url)
}

fn get_chunklist(name: &str) -> Result<String, Box<dyn Error>> {
    let url: String = get_playlist_url(name)?;
    let data = reqwest::blocking::get(&url)?.bytes()?;
    let result = m3u8_rs::parse_playlist(&data);
    if let Result::Ok((_, Playlist::MasterPlaylist(pl))) = result {
        let best_variant = pl
            .variants
            .iter()
            .max_by_key(|x| x.bandwidth.parse::<i32>().unwrap())
            .unwrap();
        let path = PathBuf::from(url);
        let chunklist = format!(
            "{}/{}",
            path.parent().unwrap().to_str().unwrap(),
            &best_variant.uri
        );
        Ok(chunklist)
    } else {
        Err("Invalid Playlist".into())
    }
}

fn get_media_segments(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = reqwest::blocking::get(url)?.bytes()?;
    let result = m3u8_rs::parse_playlist(&data);
    if let Result::Ok((_, Playlist::MediaPlaylist(pl))) = result {
        let path = PathBuf::from(url);
        let base_url = path.parent().unwrap().to_str().unwrap();
        let segments: Vec<String> = pl
            .segments
            .iter()
            .map(|x| format!("{}/{}", base_url, x.uri))
            .collect();
        Ok(segments)
    } else {
        Err("Could not get media segments".into())
    }
}
