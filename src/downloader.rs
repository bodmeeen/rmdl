use::std::fs;
use std::{process::Command};
use::std::io;

use crate::cmd_builder::{build_base_command, get_folder_title};

pub const ERR_START_YTDLP: &str = "Не вдалося запустити yt-dlp";

pub fn download_album() {
    let clean_url = input_url();
    let folder_name = get_folder_title(&clean_url);

    let mut cmd = build_base_command(&clean_url);
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


pub fn download_playlist() {
    let clean_url = input_url();
    let folder_name = get_folder_title(&clean_url);

    let mut cmd = build_base_command(&clean_url);
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


pub fn download_single_song() {
    let clean_url = input_url();
    let mut cmd = build_base_command(&clean_url);
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


pub fn download_song_by_title() {
    println!("Введіть повну назву пісні: ");

    let mut song_name: String = String::new();
        io::stdin()
            .read_line(&mut song_name)
            .expect("Error in reading input");
    
    let quality_mus = quality_check();
    
    let clean_name = song_name.trim();
    
    println!("\n Завантаження розпочато... "); 

    // запуск основного завантаження    
    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--embed-thumbnail")
        .arg("--audio-format").arg("mp3")
        .arg("--embed-metadata") // теги
        .arg("--audio-quality").arg(&quality_mus)
        .arg("--no-playlist") // завантажити тільки цю пісню якщо посилання на альбом чи плейлист
        .arg("-o").arg(format!("./Music/Singles/%(title)s.%(ext)s")) // папка та чиста назва
        .arg(format!("ytmsearch1:{}", clean_name))
        .status()
        .expect(ERR_START_YTDLP);

    if status.success() {
        println!("Успішно завантажено!");
        println!("Шлях до папки: ./Music/Singles");
        let files = show_music_files("Music/Singles");
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
            .expect("Помилка в читанні рядка");
        url.trim().to_string()
}


pub fn quality_check() -> String{
    let mut quality_str = String::new();
    loop {
        println!("Вкажіть бажану якість музики(0-9, де 0 - найбільша)");
        let mut inp: String = String::new();
            io::stdin()
                .read_line(&mut inp)
                .expect("Помилка в читанні рядка");

        match inp.trim().parse::<u8>() {
            Ok(inp) => {
                if inp <= 9 {
                    quality_str = inp.to_string();
                    break;
                } else {
                    println!("Помилка: введена цифра {} більше за 9, спробуйте ще раз", inp);
                }
            }
            Err(_) => {
                println!("Помилка: ви ввели не цифру, спрoбуйте ще раз");
            }
        }
    }
    quality_str
}