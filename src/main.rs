use std::fs::File;

pub fn open_audio_file(file_path: &str) -> Result<File, std::io::Error> {
    let file = File::open(file_path)?;
    println!("Opening audio file: {}", file_path);
    Ok(file)
}

fn main() {
    println!("Hello, world!");
}