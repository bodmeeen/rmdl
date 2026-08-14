use std::fs;
use std::io;
use std::path::PathBuf;

use crate::cmd_builder::{build_base_command, get_folder_title};

pub const ERR_START_YTDLP: &str = "Не вдалося запустити yt-dlp";

pub fn download_album(system_path: &PathBuf) {
    let clean_url = input_url();
    let folder_name = get_folder_title(&clean_url);

    let mut cmd = build_base_command(&clean_url);
    cmd.arg("-o").arg(format!("{}/Albums/%(playlist_title)s/%(title)s.%(ext)s", system_path.display()));
    
    println!("\n Завантаження розпочато...");
    let status = cmd.status().expect(ERR_START_YTDLP);

    
    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: {}", folder_name);
        let path_for_files = format!("{}/Albums/{}", system_path.display(), folder_name);
        let files = show_music_files(&path_for_files);
        print_files_list(&files);
    } else {
        println!("Виникла помилка під час завантаження");
    }
}


pub fn download_playlist(system_path: &PathBuf) {
    let clean_url = input_url();
    let folder_name = get_folder_title(&clean_url);

    let mut cmd = build_base_command(&clean_url);
    cmd.arg("-o").arg(format!("{}/Playlists/%(playlist_title)s/%(title)s.%(ext)s", system_path.display()));

    println!("\n Завантаження розпочато...");
    let status = cmd.status().expect(ERR_START_YTDLP);


    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: {}", folder_name);
        let path_for_files = format!("{}/Playlists/{}", system_path.display(), folder_name);
        let files = show_music_files(&path_for_files);
        print_files_list(&files);
    } else {
        println!("Виникла помилка під час завантаження");
    }
}


pub fn download_single_song(system_path: &PathBuf) {
    let clean_url = input_url();
    let mut cmd = build_base_command(&clean_url);
    cmd.arg("--no-playlist");
    cmd.arg("-o").arg(format!("{}/Singles/%(title)s.%(ext)s", system_path.display()));

    println!("\n Завантаження розпочато...");
    let status = cmd.status().expect(ERR_START_YTDLP);


    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: ./Music/Singles"); //тут теж потрібно system_path поставити
        // let temp_path = &system_path.display();
        let files = show_music_files(&format!("{}/Singles", system_path.display()));
        print_files_list(&files);
    } else {
        println!("Виникла помилка під час завантаження");
    }
}   


pub fn show_music_files(new_folder: &str) -> Vec<String> {
    let path = format!("{}", new_folder);
    let mut file_names: Vec<String> = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()){
                    file_names.push(file_name.to_string());
                }
            }
        }
    } else {
        println!("Помилка: папку '{}' не знайдено", path);
    }
    file_names
}

// отримання зрізу (slice) замість посилання на Vec
pub fn print_files_list(files: &[String]) {
    println!("Усі файли: ");
    for (index, name) in files.iter().enumerate() {
        println!("{} -> {}", index + 1, name);
    }
}


pub fn input_url() -> String{
    println!("Вставте посилання: ");
        let mut url: String = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Помилка в читанні рядка");
        url.trim().to_string()
}