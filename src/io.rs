use std::fs;
use std::io::Write;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Something went wrong reading the file")
}

pub fn write_to_file(file_path: &str, text: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(text.as_bytes())?;

    Ok(())
}
