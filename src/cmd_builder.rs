use std::{process::Command};

use crate::downloader::{quality_check};

pub fn build_base_command(clean_url: &String) -> Command {
    let quality_mus = quality_check();
    let mut cmd = Command::new("yt-dlp");

    cmd.arg("-x")
        .arg("--embed-thumbnail")
        .arg("--audio-format").arg("mp3")
        .arg("--embed-metadata")
        .arg("--audio-quality").arg(&quality_mus)
        .arg(&clean_url);

    cmd
}

pub fn get_folder_title(clean_url: &String) -> String {
    // запис назви альбому який створиться
    let yt_output = Command::new("yt-dlp")
        .arg("--print")
        .arg("%(playlist_title)s")
        .arg("--playlist-items").arg("1")
        .arg(&clean_url)
        .output()
        .expect("Не вдалося запустити yt-dlp");

    // перетворення виводу в текст
    let full_output = String::from_utf8_lossy(&yt_output.stdout).trim().to_string();

    // береться тільки перший рядок з того, що вивів yt-dlp
    let folder_name = full_output.lines().next().unwrap_or("").trim().to_string();

    folder_name
}