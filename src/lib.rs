use std::fs;
use std::io::Read;

pub fn read_json_file(file_path: &str) -> String {
    let mut json_file = fs::File::open(file_path).expect("File couldnt be opened!");
    let mut content = String::new();
    let _ = json_file.read_to_string(&mut content);

    content
}
