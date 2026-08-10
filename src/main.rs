use::std::fs;
use::std::io;
use::std::env;

mod downloader;
mod batch_down;

fn main() {
    // перевірка що ввів користувач в термінал, 1 якщо просто rmdl, 2 якщо 
    // з назвою файлу де є посилання для завантаження
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            todo!()
        }
        2 => {
            let file_path = &args[1];
            println!("Масове завантаження з файлу {}", file_path);
            match batch_down::file_read(file_path) {
                Ok(()) => println!("Файл успішно опрацьовано!"),
                Err(e) => eprintln!("Помилка при зчитуванні файлу '{file_path}': {e}"),
            }
            // тут потрібно викликати ф-ю для завантаження з файлу
        }
        _ => {
            println!("Помилка: забагато аргументів");
        }
    }
    let folders = ["./Music/Albums", "./Music/Singles", "./Music/Playlists"];

    for folder in folders {
        fs::create_dir_all(folder).expect(&format!("Помилка при створенні папки {}", folder));
    }
    loop {
        println!("\nВиберіть що потрібно завантажити:\n 1 - альбом\n 2 - плейлист\n 3 - одна пісня\n 4 - пісня по назві\n 5 - кастомне завантаження (не працює)\n q - вихід");

        let mut input_text = String::new();
        
        io::stdin()
            .read_line(&mut input_text)
            .expect("Error in reading input");

        match input_text.trim().chars().next() {
            Some('1') => downloader::download_album(),
            Some('2') => downloader::download_playlist(),
            Some('3') => downloader::download_single_song(),
            Some('4') => downloader::download_song_by_title(),
            // Some('5') => downloader::custom_download(),
            Some('q') => break,
            Some (other_ch) => println!("Команди {} не існує", other_ch),
            None => println!("Натиснуто Enter на порожньому місці!")
        }
    }
}

