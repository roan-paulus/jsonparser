#![allow(dead_code, unused_variables)]

use std::process;

use parser::Parser;

mod parser;
mod scan;
mod types;

fn main() {
    let content = jsonparser::read_json_file("test.json");

    let scanner = scan::Scanner::new(scan::ScannerErrorHandler::new());

    let tokens = match scanner.scan(content) {
        Ok(tokens) => tokens,
        Err(error_handler) => {
            error_handler.print_errors();
            process::exit(1);
        }
    };

    tokens.iter().for_each(|token| println!("{:?}", token));

    let jvalue = match Parser::parse(&tokens) {
        Ok(jvalue) => jvalue,
        Err(err) => panic!("{:?}", err),
    };

    jvalue.print();
}
