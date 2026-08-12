use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{io::Read, path::Path, process::Command};

use crate::downloader::{print_files_list, show_music_files};


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
                println!("Знайдено текст {} - {}", method, url); // замість цього виклик ф-ї
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
                let yt_output = Command::new("yt-dlp")
                    .arg("--print")
                    .arg("%(playlist_title)s")
                    .arg("--playlist-items").arg("1")
                    .arg(&url)
                    .output()
                    .expect("Не вдалося запустити yt-dlp");

                    println!("\n Завантаження розпочато... ");

                // перетворення виводу в текст
                let full_output = String::from_utf8_lossy(&yt_output.stdout).trim().to_string();

                    // береться тільки перший рядок з того, що вивів yt-dlp
                let folder_name = full_output.lines().next().unwrap_or("").trim().to_string();

                // запуск основного завантаження    
                let status = Command::new("yt-dlp")
                    .arg("-x")
                    .arg("--embed-thumbnail")
                    .arg("--audio-format").arg("mp3")
                    .arg("--embed-metadata")
                    .arg("-o").arg(format!("./Music/Albums/%(playlist_title)s/%(title)s.%(ext)s"))
                    .arg(&url)
                    .status()
                    .expect("Помилка у завантаженні");

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
                let yt_output = Command::new("yt-dlp")
                    .arg("--print")
                    .arg("%(playlist_title)s")
                    .arg("--playlist-items").arg("1")
                    .arg(&url)
                    .output()
                    .expect("Не вдалося запустити yt-dlp");

                println!("\n Завантаження розпочато... ");

                // перетворення виводу в текст
                let full_output = String::from_utf8_lossy(&yt_output.stdout).trim().to_string();

                // береться тільки перший рядок з того, що вивів yt-dlp
                let folder_name = full_output.lines().next().unwrap_or("").trim().to_string();

                // запуск основного завантаження    
                let status = Command::new("yt-dlp")
                    .arg("-x")
                    .arg("--embed-thumbnail")
                    .arg("--audio-format").arg("mp3")
                    .arg("--embed-metadata") // теги
                    .arg("-o").arg(format!("./Music/Playlists/%(playlist_title)s/%(title)s.%(ext)s")) // папка та чиста назва
                    .arg(&url)
                    .status()
                    .expect("Помилка у завантаженні");

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
                let status = Command::new("yt-dlp")
                    .arg("-x")
                    .arg("--embed-thumbnail")
                    .arg("--audio-format").arg("mp3")
                    .arg("--embed-metadata")
                    .arg("--no-playlist")
                    .arg("-o").arg(format!("./Music/Singles/%(title)s.%(ext)s"))
                    .arg(&url)
                    .status()
                    .expect("Помилка у завантаженні");

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