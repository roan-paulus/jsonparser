#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::prelude::*;

mod scan;

fn main() {
    let content = read_json_file("test.json");
    let mut scanner = scan::Scanner::new();
    let tokens = scanner.scan(content);

    for token in tokens {
        println!("{:?}", token);
    }
}

fn read_json_file(file_path: &str) -> String {
    let mut json_file = File::open(file_path).expect("File couldnt be opened!");
    let mut content = String::new();
    let _ = json_file.read_to_string(&mut content);

    return content;
}
