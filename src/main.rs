use::std::fs;
use::std::io;
use::std::env;

mod downloader;
mod batch_down;
mod cmd_builder;

fn main() {
    let folders = ["./Music/Albums", "./Music/Singles", "./Music/Playlists"];

    for folder in folders {
        fs::create_dir_all(folder).expect(&format!("Помилка при створенні папки {}", folder));
    }
    // перевірка що ввів користувач при виклику, 1 якщо просто rmdl, 2 якщо 
    // з назвою файлу де є посилання для завантаження
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            loop {
                println!("\nВиберіть що потрібно завантажити:\n 1 - альбом\n 2 - плейлист\n 3 - одна пісня\n 4 - пісня по назві\n q - вихід");
        
                let mut input_text = String::new();
                
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("Error in reading input");
        
                match input_text.trim().chars().next() {
                    Some('1') => downloader::download_album(),
                    Some('2') => downloader::download_playlist(),
                    Some('3') => downloader::download_single_song(),
                    Some('4') => downloader::download_song_by_title(),
                    Some('q') => break,
                    Some (other_ch) => println!("Команди {} не існує", other_ch),
                    None => println!("Натиснуто Enter на порожньому місці!")
                }
            }
        }
        2 => {
            let file_path = &args[1];
            println!("Масове завантаження з файлу {}", file_path);
            match batch_down::parse_file(file_path) {
                Ok(tasks) => batch_down::download_from_txt(&tasks),
                Err(e) => eprintln!("Помилка при зчитуванні файлу '{file_path}': {e}"),
            }
        }
        _ => {
            println!("Помилка: забагато аргументів");
        }
    }

}

