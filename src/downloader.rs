use::std::fs;
use std::{io::Read, path::Path, process::Command};
use::std::io;

pub fn download_album() {
    let clean_url = input_url();
    
    // запис назви альбому який створиться
    let yt_output = Command::new("yt-dlp")
    .arg("--print")
    .arg("%(playlist_title)s")
    .arg("--playlist-items").arg("1")
    .arg(&clean_url)
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
    .arg("-o").arg(format!("./Music/Albums/%(playlist_title)s/%(title)s.%(ext)s")) // папка та чиста назва
    .arg(&clean_url)
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


pub fn download_playlist() {
    let clean_url = input_url();
    
    // запис назви альбому який створиться
    let yt_output = Command::new("yt-dlp")
    .arg("--print")
    .arg("%(playlist_title)s")
    .arg("--playlist-items").arg("1")
    .arg(&clean_url)
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
    .arg(&clean_url)
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


pub fn download_single_song() {
    let clean_url = input_url();
    
    println!("\n Завантаження розпочато... "); 
    
    // запуск основного завантаження    
    let status = Command::new("yt-dlp")
    .arg("-x")
    .arg("--embed-thumbnail")
    .arg("--audio-format").arg("mp3")
    .arg("--embed-metadata") // теги
    .arg("--no-playlist") // завантажити тільки цю пісню якщо посилання на альбом чи плейлист
    .arg("-o").arg(format!("./Music/Singles/%(title)s.%(ext)s")) // папка та чиста назва
    .arg(&clean_url)
    .status()
    .expect("Помилка у завантаженні");

    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: ./Music/Singles");
        let files = show_music_files("Singles");
        print_files_list(&files);
    } else {
        println!("Виникла помилка під час завантаження");
    }
}   


pub fn download_song_by_title() {
    println!("Введіть повну назву пісні: ");

    let mut song_name: String = String::new();
        io::stdin()
            .read_line(&mut song_name)
            .expect("Error in reading input");
        
    let clean_name = song_name.trim();
    
    println!("\n Завантаження розпочато... "); 

    // запуск основного завантаження    
    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--embed-thumbnail")
        .arg("--audio-format").arg("mp3")
        .arg("--embed-metadata") // теги
        .arg("--no-playlist") // завантажити тільки цю пісню якщо посилання на альбом чи плейлист
        .arg("-o").arg(format!("./Music/Singles/%(title)s.%(ext)s")) // папка та чиста назва
        .arg(format!("ytsearch1:{}", clean_name))
        .status()
        .expect("Помилка у завантаженні");

    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: ./Music/Singles");
        let files = show_music_files("Singles");
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


pub fn print_files_list(files: &[String]) {// отримання зрізу (slice) замість посилання на Vec
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
            .expect("Error in reading input");
        url.trim().to_string()
}