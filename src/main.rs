#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::process::ExitCode;

mod scan;

fn main() -> ExitCode {
    let content = read_json_file("test.json");

    let scanner = scan::Scanner::new(scan::ScannerErrorHandler::new());

    let tokens = match scanner.scan(content) {
        Ok(tokens) => tokens,
        Err(error_handler) => {
            error_handler.print_errors();
            return ExitCode::FAILURE;
        }
    };

    for token in tokens {
        println!("{:?}", token);
    }

    ExitCode::SUCCESS
}

fn read_json_file(file_path: &str) -> String {
    let mut json_file = File::open(file_path).expect("File couldnt be opened!");
    let mut content = String::new();
    let _ = json_file.read_to_string(&mut content);

    return content;
}
