use std::{iter::Enumerate, str::Chars};

#[derive(Eq, PartialEq, Debug)]
pub enum TokenType {
    Comma,
    OpeningBrace,
    ClosingBrace,
    OpeningSquirly,
    ClosingSquirly,
    String,
    Bool,
    Number,
}

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    position: usize,
}

pub fn tokenize(chars: String) -> Vec<Token> {
    // TODO: Display line where error happened
    //
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = chars.chars().enumerate();

    // Try to tokenize a character or sequence of characters
    while let Some((i, c)) = chars.next() {
        let token: Option<Token> = match c {
            ',' => Some(Token {
                lexeme: c.to_string(),
                token_type: TokenType::Comma,
                position: i,
            }),
            '[' => Some(Token {
                lexeme: c.to_string(),
                token_type: TokenType::OpeningBrace,
                position: i,
            }),
            ']' => Some(Token {
                lexeme: c.to_string(),
                token_type: TokenType::ClosingBrace,
                position: i,
            }),
            '{' => Some(Token {
                lexeme: c.to_string(),
                token_type: TokenType::OpeningSquirly,
                position: i,
            }),
            '}' => Some(Token {
                lexeme: c.to_string(),
                token_type: TokenType::ClosingSquirly,
                position: i,
            }),
            '"' => Some(Token {
                lexeme: json_string(&mut chars),
                token_type: TokenType::String,
                position: i,
            }),
            char if char.is_alphanumeric() => Some(json_alphanumeric(&mut chars, char, i)),
            _ => None,
        };

        // Add token to tokens
        match token {
            None => (),
            Some(t) => {
                tokens.push(t);
            }
        }
    }

    tokens
}

fn json_string(chars: &mut Enumerate<Chars>) -> String {
    let mut result_string = String::new();
    loop {
        match chars.next() {
            Some((_, '"')) => {
                break;
            }
            Some((i, '\n')) => {
                panic!("Unterminated string literal at position: {i}");
            }
            Some((_, char)) => result_string.push(char),
            None => panic!("Nothing found"),
        }
    }
    result_string
}

fn json_alphanumeric(chars: &mut Enumerate<Chars>, starting_char: char, position: usize) -> Token {
    let mut result_string = String::new();
    result_string.push(starting_char);

    // Build result_string up by consuming all alphanumeric characters
    loop {
        match chars.next() {
            Some((_, c)) if c.is_alphanumeric() => result_string.push(c),
            Some((_, _)) | None => break,
        }
    }

    // Match with the correct keyword and return the appropriate Token
    return match result_string.as_str() {
        "true" | "false" => Token {
            lexeme: result_string,
            token_type: TokenType::Bool,
            position,
        },
        s if s.chars().all(char::is_numeric) => Token {
            lexeme: result_string,
            token_type: TokenType::Number,
            position,
        },
        _ => panic!("'{result_string}' is of unknown keyword"),
    };
}
