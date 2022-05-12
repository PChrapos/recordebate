use std::{error::Error, fs, path::Path, process::Command};

pub fn convert_to_mp4(input_file: &str, remux: bool) -> Result<(), Box<dyn Error>> {
    let tmp_output_file = Path::new(input_file).with_extension("tmp.mp4");
    let output_file = Path::new(input_file).with_extension("mp4");
    fs::remove_file(&output_file).ok();
    fs::remove_file(&tmp_output_file).ok();
    if Path::new(input_file).metadata()?.len() == 0 {
        fs::remove_file(&input_file).ok();
        return Err("Invalid input file".into());
    }

    if remux {
        Command::new("ffmpeg")
            .arg("-i")
            .arg(input_file)
            .args(["-map", "0", "-map", "-0:d", "-c", "copy"])
            .arg(&tmp_output_file)
            .output()?;
    } else {
        Command::new("ffmpeg")
            .arg("-i")
            .arg(input_file)
            .arg(&tmp_output_file)
            .output()?;
    }

    fs::rename(tmp_output_file, output_file)?;
    fs::remove_file(input_file).ok();
    Ok(())
}

pub fn create_thumbnail(input_file: &str) -> Result<(), Box<dyn Error>> {
    let output_file = Path::new(input_file).with_extension("png");
    fs::remove_file(&output_file).ok();

    let output_data = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input_file,
        ])
        .output()?
        .stdout;
    let output = String::from_utf8(output_data)?;
    let video_duration = output.split('.').next().unwrap().parse::<i32>()?;
    let thumbnail_time = video_duration / 2;
    let formatted_time = format!("{thumbnail_time}.000");
    Command::new("ffmpeg")
        .args(["-ss", &formatted_time, "-i", input_file, "-vframes", "1"])
        .arg(output_file)
        .output()?;
    Ok(())
}
