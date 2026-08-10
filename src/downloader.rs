use::std::fs;
use std::{io::Read, path::Path, process::Command};
use::std::io;


pub fn download_album() {
    let clean_url = input_url();
    let quality_mus = quality_check();

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
        .arg("--audio-quality").arg(&quality_mus)
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
    let quality_mus = quality_check();
    
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
        .arg("--audio-quality").arg(&quality_mus)
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
    let quality_mus = quality_check();
    println!("\n Завантаження розпочато...");
    
    // запуск основного завантаження    
    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--embed-thumbnail")
        .arg("--audio-format").arg("mp3")
        .arg("--embed-metadata") // теги
        .arg("--audio-quality").arg(&quality_mus)
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



          // код для кастомного завантаження //
// pub enum DownloadMode {
//     Album,
//     Playlist,
//     Single,
//     Custom(String), // збереження назви для нової папки яку введе користувач
// }


// impl DownloadMode {
//     // ф-я повертає готовий рядок для аргумента -o в yt-dlp
//     fn get_output_arg_o(&self) -> String { 
//         match self {
//             DownloadMode::Album => String::from("./Music/Albums/%(playlist_title)s/%(title)s.%(ext)s"),
//             DownloadMode::Playlist => String::from("./Music/Playlists/%(playlist_title)s/%(titles)s.%(ext)s"),
//             DownloadMode::Single => String::from("./Music/Singles/%(title)s.%(ext)s"),
//             DownloadMode::Custom(folder_name) => format!("./Music/{}/%(title)s.%(ext)s", folder_name),
//         }
//     }
// }

// struct DownloadTask { 
//     url: String,
//     quality: String, // тут потрібно дописати
// }


// impl DownloadTask { 
//     pub fn start_download(&self) {
//         // отримання правильного шляху з enum
//         let output_path = self.mode.get_output_arg_o();
//         println!("Запускаємо yt-dlp...");

//         let status = Command::new("yt-dlp")
//             .arg("-x")
//             .arg("--audio-format").arg("mp3")
//             .arg("--audio-quality").arg(&self.quality)
//             .arg("-o").arg(&output_path) // згенерований шлях
//             .arg(&self.url)              // посилання
//             .status()
//             .expect("Помилка запуску");
            
//         // далі має бути  перевірка status.success()
//     }
// }


// pub fn custom_download() {
//     // далі потрібно додати зчитування вводу користувача, ну і далі все що треба
// }
