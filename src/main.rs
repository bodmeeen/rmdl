use::std::fs;
use::std::io;

mod downloader;

fn main() {
    let folders = ["./Music/Albums", "./Music/Singles", "./Music/Playlists"];

    for folder in folders {
        fs::create_dir_all(folder).expect(&format!("Помилка при створенні папки {}", folder));
    }
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
            Some(other_ch) => println!("Команди {} не існує", other_ch),
            None => println!("Натиснуто Enter на порожньому місці!")
        }
    }
}

