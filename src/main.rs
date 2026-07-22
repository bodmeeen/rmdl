use::std::fs;
use std::{io::Read, path::Path, process::Command};
use::std::io;

fn main() {
    let dir_path = "./Music/";
    if !Path::new(dir_path).exists() {
        match fs::create_dir(dir_path) {
            Ok(_) => println!("Створено папку {}", dir_path),
            Err(e) => eprintln!("Помилка при створенні папки: {}", e)
        }
    }

    println!("\nВиберіть дію:\n 1 - альбом (з обкладинкою, окрема папка, mp3, без зайвих символів)
                \n 2 - одна пісня (те ж саме що і в альбомі, тільки без окремої папки)");

    let mut input_text = String::new();
    
    io::stdin()
        .read_line(&mut input_text)
        .expect("Error in reading input");

    match input_text.trim().chars().next() {
        Some('1') => download_album(),
        Some('2') => todo!(),
        // Some('q') => break,
        Some(other_ch) => println!("Команди {} не існує", other_ch),
        None => println!("Натиснуто Enter на порожньому місці!")
    }
    
}

fn download_album(){
    println!("Вставте посилання на альбом: ");
    let mut url: String = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Error in reading input");
    
    let clean_url = url.trim();

    println!("\n Завантаження почато... ");

    // запис назви альбому який створиться
    let yt_output = Command::new("yt-dlp")
        .arg("--print")
        .arg("%(playlist_title)s")
        .arg("--playlist-items").arg("1")
        .arg(clean_url)
        .output()
        .expect("Не вдалося запустити yt-dlp");

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
        .arg("-o").arg(format!("./Music/%(playlist_title)s/%(title)s.%(ext)s")) // папка та чиста назва
        .arg(clean_url)
        .status()
        .expect("Помилка у завантаженні");


    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: {}", folder_name);
        let files = show_music_files(&folder_name);
        print_files_list(&files);
    } else {
        println!("Виникла помилка під час завантаження");
    }
}

fn show_music_files(new_folder: &str) -> Vec<String> {
    let path = format!("./Music/{}", new_folder);
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

fn print_files_list(files: &[String]) {// отримання зрізу (slice) замість посилання на Vec
    println!("Усі файли: ");
    for (index, name) in files.iter().enumerate() {
        println!("{} -> {}", index + 1, name);
    }
}