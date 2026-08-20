use std::path::PathBuf;

use crate::cmd_builder::{build_base_command, get_folder_title};
use crate::utils::{show_music_files, print_files_list, input_url};

pub const ERR_START_YTDLP: &str = "Не вдалося запустити yt-dlp";

pub fn download_collection(system_path: &PathBuf, type_folder_name: &str) {
    let clean_url = input_url();
    let folder_name = get_folder_title(&clean_url);

    let mut cmd = build_base_command(&clean_url);
    cmd.arg("-o")
        .arg(format!("{}/{}/%(playlist_title)s/%(playlist_index)02d - %(title)s.%(ext)s", system_path.display(), type_folder_name));

    println!("\n Завантаження розпочато...");
    let status = cmd.status().expect(ERR_START_YTDLP);

    
    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: {}", folder_name);
        let path_for_files = format!("{}/{}/{}", system_path.display(), type_folder_name, folder_name);
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