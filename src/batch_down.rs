use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::cmd_builder::{build_base_command, get_folder_title};
use crate::downloader::{print_files_list, show_music_files, ERR_START_YTDLP};


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
                println!("Знайдено текст {} - {}", method, url); // test
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


pub fn download_from_txt(file_content: &[(u8, String)]) {
    println!("Отримано {} завдань", file_content.len());

    for (index, (method, url)) in file_content.iter().enumerate() {
        println!(" [{},{}] Оброблення методу {} для {}", index + 1, file_content.len(), method, url);

        match method {
            1 => {
                println!("Завантаження як альбом");
                let folder_name = get_folder_title(&url);

                // запуск основного завантаження    
                let mut cmd = build_base_command(&url);
                cmd.arg("-o").arg(format!("./Music/Albums/%(playlist_title)s/%(title)s.%(ext)s"));

                let status = cmd.status().expect(ERR_START_YTDLP);

                if status.success() {
                    println!("Успішно завантажено!");
                    println!("Шлях до папки: {}", folder_name);
                    let path_for_files = format!("./Music/Albums/{}", folder_name);
                    let files = show_music_files(&path_for_files);
                        print_files_list(&files);
                } else {
                    println!("Виникла помилка під час завантаження");
                }
            }

            2 => {
                println!("Завантаження як плейлист");
                let folder_name = get_folder_title(&url);

                let mut cmd = build_base_command(&url);
                cmd.arg("-o").arg(format!("./Music/Playlists/%(playlist_title)s/%(title)s.%(ext)s"));

                let status = cmd.status().expect(ERR_START_YTDLP);

                if status.success() {
                    println!("Успішно завантажено!");
                    println!("Шлях до папки: {}", folder_name);
                    let path_for_files = format!("./Music/Playlists/{}", folder_name);
                    let files = show_music_files(&path_for_files);
                    print_files_list(&files);
                } else {
                    println!("Виникла помилка під час завантаження");
                }
            }

            3 => {
                println!("Завантаження як сингл");
                let mut cmd = build_base_command(&url);
                cmd.arg("--no-playlist");
                cmd.arg("-o").arg(format!("./Music/Singles/%(title)s.%(ext)s"));

                let status = cmd.status().expect(ERR_START_YTDLP);

                println!("\n Завантаження розпочато...");

                if status.success() {
                    println!("Успішно завантажено!");
                    println!("Шлях до папки: ./Music/Singles");
                    let files = show_music_files("Music/Singles");
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