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
    line: u16,
    column: u16,
}

pub struct Scanner {
    line: u16,
    column: u16,
}

impl Scanner {
    pub fn new() -> Self {
        return Self { line: 1, column: 1 };
    }

    pub fn scan(&mut self, chars: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = chars.chars();

        // Try to tokenize a character or sequence of characters
        while let Some(c) = chars.next() {
            let token: Option<Token> = match c {
                ',' => Some(Token {
                    lexeme: c.to_string(),
                    token_type: TokenType::Comma,
                    line: self.line,
                    column: self.column,
                }),
                '[' => Some(Token {
                    lexeme: c.to_string(),
                    token_type: TokenType::OpeningBrace,
                    line: self.line,
                    column: self.column,
                }),
                ']' => Some(Token {
                    lexeme: c.to_string(),
                    token_type: TokenType::ClosingBrace,
                    line: self.line,
                    column: self.column,
                }),
                '{' => Some(Token {
                    lexeme: c.to_string(),
                    token_type: TokenType::OpeningSquirly,
                    line: self.line,
                    column: self.column,
                }),
                '}' => Some(Token {
                    lexeme: c.to_string(),
                    token_type: TokenType::ClosingSquirly,
                    line: self.line,
                    column: self.column,
                }),
                '"' => Some(Token {
                    lexeme: self.json_string(&mut chars),
                    token_type: TokenType::String,
                    line: self.line,
                    column: self.column,
                }),
                char if char.is_alphanumeric() => Some(self.json_alphanumeric(&mut chars, char)),
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    None
                }
                _ => {
                    self.column += 1;
                    None
                }
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

    fn json_string(&mut self, chars: &mut Chars) -> String {
        let mut result_string = String::new();
        loop {
            self.column += 1;
            match chars.next() {
                Some('"') => {
                    break;
                }
                Some('\n') => {
                    panic!(
                        "Unterminated string literal at line: {} column: {}",
                        self.line, self.column
                    );
                }
                Some(char) => result_string.push(char),
                None => panic!("Nothing found"),
            }
        }
        result_string
    }

    fn json_alphanumeric(&mut self, chars: &mut Chars, starting_char: char) -> Token {
        let mut result_string = String::new();
        result_string.push(starting_char);

        // Build result_string up by consuming all alphanumeric characters
        loop {
            self.column += 1;
            match chars.next() {
                Some(c) if c.is_alphanumeric() => result_string.push(c),
                Some(_) | None => break,
            }
        }

        // Match with the correct keyword and return the appropriate Token
        return match result_string.as_str() {
            "true" | "false" => Token {
                lexeme: result_string,
                token_type: TokenType::Bool,
                line: self.line,
                column: self.column,
            },
            s if s.chars().all(char::is_numeric) => Token {
                lexeme: result_string,
                token_type: TokenType::Number,
                line: self.line,
                column: self.column,
            },
            _ => panic!(
                "'{result_string}' is of unknown keyword at line: {}, col: {}",
                self.line,
                self.column - result_string.len() as u16
            ),
        };
    }
}
