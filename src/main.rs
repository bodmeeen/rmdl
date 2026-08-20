use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;
use dirs::audio_dir;

mod downloader;
mod batch_down;
mod cmd_builder;
mod utils;

fn main() {
    let system_path: PathBuf = match audio_dir() {
        Some(base_music_path) => {
        println!("Системну папку знайдено: {:?}", base_music_path);
        
        let subfolders = ["Albums", "Singles", "Playlists"];
        
        for folder in subfolders {
            // клонування шляху для того щоб випадково не змінити його
            let mut target_path = base_music_path.clone();
            target_path.push(folder);
            
            if let Err(e) = fs::create_dir_all(&target_path) {
                eprintln!("Помилка при створенні папки {:?}: {}", target_path, e);
            }
        } 
        base_music_path
    } 
        None => {
        eprintln!("Помилка: не вдалося знайти системну папку для музики!");
        println!("Замість цього використовується локальна папка ./Music");
        PathBuf::from("./Music")
    }
};
    
    // перевірка що ввів користувач при виклику, 1 якщо просто rmdl, 2 якщо 
    // з назвою файлу де є посилання для завантаження
    // -- env::args() збирає аргументи тільки один раз на початку
    let args: Vec<String> = env::args().collect(); 
    match args.len() {
        1 => {
            loop {
                println!("\nВиберіть що потрібно завантажити:\n 1 - альбом\n 2 - плейлист\n 3 - одна пісня\n q - вихід");
        
                let mut input_text = String::new();
                
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("Error in reading input");

                match input_text.trim().chars().next() {
                    Some('1') => downloader::download_collection(&system_path, "Albums"),
                    Some('2') => downloader::download_collection(&system_path, "Playlists"),
                    Some('3') => downloader::download_single_song(&system_path),
                    Some('q') => break,
                    Some (other_ch) => println!("Команди {} не існує", other_ch),
                    None => println!("Натиснуто Enter на порожньому місці!")
                }
            }
        }
        2 => {
            let file_path = &args[1];
            let mut final_path = file_path.clone();
            if !std::path::Path::new(&final_path).exists() { // перевірка поточної папки
                // якщо файлу немає в поточній папці(де відкрито термінал), то шукається Стільниця
                if let Some(mut desktop) = dirs::desktop_dir() { 
                    desktop.push(file_path); // додавання назву файлу до шляху Стільниці
                    if desktop.exists() {
                        // якщо файл є, то перетворити з PathBuf назад в String
                        final_path = desktop.to_string_lossy().to_string();
                    }
                }
            }
            println!("Масове завантаження з файлу {}", final_path);
            match batch_down::parse_file(&final_path) {
                Ok(tasks) => batch_down::download_from_txt(&tasks, &system_path),
                Err(e) => eprintln!("Помилка при зчитуванні файлу '{file_path}': {e}"),
            }
        }
        _ => {
            println!("Помилка: забагато аргументів");
        }
    }

}

