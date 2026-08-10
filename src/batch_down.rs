use std::fs::File;
use std::io::{self, BufRead, BufReader};


    // потрібно змінити, щоб повертався Vec<u8, String> і далі передавати це
pub fn file_read(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() { continue; }

        if let Some((method_str, url_str)) = line.split_once(' ') {
            if let Ok(method) = method_str.parse::<u8>() { // парсинг методу завантаження яке прописано в файлі у число
                let url = url_str.trim_matches('"');
                println!("Знайдено текст {} - {}", method, url); // замість цього виклик ф-ї
                todo!()   // тут потрібно викликати завантажувач який ще потрібно дописати
            }
            else {
                eprintln!("Помилка парсингу методу завантаження у рядку {}: {}",index + 1, method_str);
            }
        }
        else {
            eprintln!("Некоректний формат у рядку {}: {}", index + 1, line);
        }
    }
    Ok(())
}