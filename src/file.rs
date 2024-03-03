use std::fs;
use std::io::Write;
use std::path::Path;

pub fn open_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file") as String
}

pub fn write_string_to_file(file_path: &str, contents: &String) -> std::io::Result<()> {
    let folder_path = Path::new(file_path).parent().unwrap();
    fs::create_dir_all(folder_path).expect("Failed to create directory");
    let mut file = fs::File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn path_exists(path: &String) -> bool {
    Path::new(&path).exists()
}
