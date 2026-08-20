use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use crate::cmd_builder::{build_base_command, get_folder_title};
use crate::downloader::ERR_START_YTDLP;
use crate::utils::{show_music_files, print_files_list};


pub fn parse_file(file_path: &str) -> io::Result<Vec<(u8, String)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?; // зчитування з перевіркою на помилки
        let line = line.trim();

        if line.is_empty() { continue; }

        if let Some((method_str, url_str)) = line.split_once(' ') {
            if let Ok(method) = method_str.parse::<u8>() { // парсинг методу завантаження яке прописано в файлі у число
                let url = url_str.trim_matches('"').to_string();
                println!("Знайдено посилання {} - {}", method, url); // test
                tasks.push((method, url)); // зберігання в список
            }
            else {
                eprintln!("Помилка парсингу методу завантаження у рядку {}: {}",index + 1, method_str);
            }
        }
        else {
            eprintln!("Некоректний формат у рядку {}: {}", index + 1, line);
        }
    }
    Ok(tasks)
}


pub fn download_from_txt(file_content: &[(u8, String)], system_path: &PathBuf) {
    println!("Отримано {} завдань", file_content.len());

    for (index, (method, url)) in file_content.iter().enumerate() {
        println!(" [{},{}] Оброблення методу {} для {}", index + 1, file_content.len(), method, url);

        match method {
            1 => {
                println!("\nЗавантаження як альбом\n");
                let folder_name = get_folder_title(&url);

                // запуск основного завантаження    
                let mut cmd = build_base_command(&url);
                cmd.arg("-o").arg(format!("{}/Albums/%(playlist_title)s/%(playlist_index)02d - %(title)s.%(ext)s", system_path.display()));

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

            2 => {
                println!("\nЗавантаження як плейлист\n");
                let folder_name = get_folder_title(&url);

                let mut cmd = build_base_command(&url);
                cmd.arg("-o").arg(format!("{}/Playlists/%(playlist_title)s/%(playlist_index)02d - %(title)s.%(ext)s", system_path.display()));

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

            3 => {
                println!("\nЗавантаження як сингл\n");
                let mut cmd = build_base_command(&url);
                cmd.arg("-o").arg(format!("{}/Singles/%(title)s.%(ext)s", system_path.display()));

                let status = cmd.status().expect(ERR_START_YTDLP);

                if status.success() {
                    println!("Успішно завантажено!");
                    println!("Шлях до папки: ./Music/Singles");
                    let files = show_music_files(&format!("{}/Singles", system_path.display()));
                    print_files_list(&files);
                } else {
                    println!("Виникла помилка під час завантаження");
                }
            }

            _ => {
                eprintln!("Невідомий метод завантаження");
            }
        }
    }
}