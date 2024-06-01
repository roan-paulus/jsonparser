use std::str::Chars;

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

type UnsignedInt = u16;

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    line: UnsignedInt,
    column: UnsignedInt,
}

struct ScannerError {
    line: UnsignedInt,
    column: UnsignedInt,
    message: String,
}

pub struct ScannerErrorHandler {
    errors: Vec<ScannerError>,
}

impl ScannerErrorHandler {
    pub fn new() -> Self {
        return Self { errors: Vec::new() };
    }

    pub fn has_errored(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print_errors(&self) {
        for error in self.errors.iter() {
            println!("{} at {}:{}", error.message, error.line, error.column);
        }
    }

    fn add_error(&mut self, message: String, location: (UnsignedInt, UnsignedInt)) {
        let (line, column) = location;
        self.errors.push(ScannerError {
            line,
            column,
            message,
        });
    }
}

pub struct Scanner {
    line: UnsignedInt,
    column: UnsignedInt,
    error_handler: ScannerErrorHandler,
}

impl Scanner {
    pub fn new(error_handler: ScannerErrorHandler) -> Self {
        return Self {
            line: 1,
            column: 1,
            error_handler,
        };
    }

    pub fn scan(mut self, chars: String) -> Result<Vec<Token>, ScannerErrorHandler> {
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
                '"' => {
                    let s = self.json_string(&mut chars);
                    match s {
                        Ok(lexeme) => Some(Token {
                            lexeme,
                            token_type: TokenType::String,
                            line: self.line,
                            column: self.column,
                        }),
                        Err(message) => {
                            self.error_handler.add_error(message, self.position());
                            self.synch_after_newline(&mut chars);
                            None
                        }
                    }
                }
                char if char.is_alphanumeric() => {
                    let bool_or_number = match self.json_alphanumeric(&mut chars, char) {
                        Ok(bool_or_number) => Some(bool_or_number),
                        Err(message) => {
                            self.error_handler.add_error(message, self.position());
                            self.synch_after_newline(&mut chars);
                            None
                        }
                    };
                    bool_or_number
                }
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

        if self.error_handler.has_errored() {
            return Err(self.error_handler);
        }
        Ok(tokens)
    }

    fn json_string(&mut self, chars: &mut Chars) -> Result<String, String> {
        let mut result_string = String::new();
        let err_msg = String::from("Unterminated string");
        loop {
            self.column += 1;
            match chars.next() {
                Some('"') => {
                    return Ok(result_string);
                }
                Some('\n') => {
                    return Err(err_msg);
                }
                Some(char) => result_string.push(char),
                None => {
                    return Err(err_msg);
                }
            }
        }
    }

    fn json_alphanumeric(
        &mut self,
        chars: &mut Chars,
        starting_char: char,
    ) -> Result<Token, String> {
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
            "true" | "false" => Ok(Token {
                lexeme: result_string,
                token_type: TokenType::Bool,
                line: self.line,
                column: self.column,
            }),
            s if s.chars().all(char::is_numeric) => Ok(Token {
                lexeme: result_string,
                token_type: TokenType::Number,
                line: self.line,
                column: self.column,
            }),
            _ => Err(format!("'{result_string}' is of unknown keyword")),
        };
    }

    fn position(&self) -> (UnsignedInt, UnsignedInt) {
        (self.line, self.column)
    }

    fn synch_after_newline(&mut self, chars: &mut Chars) {
        while let Some(c) = chars.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_json_file;

    fn setup() -> Scanner {
        Scanner::new(ScannerErrorHandler::new())
    }

    fn base_check_no_errors(json: String) {
        let scanner = setup();

        let result = scanner.scan(json);
        match result {
            Ok(tokens) => true,
            Err(error_handler) => {
                error_handler.print_errors();
                panic!("Invalid input");
            }
        };
    }

    #[test]
    fn all_valid_tokens() {
        base_check_no_errors(read_json_file("scan_test_data/all_valid_tokens.json"));
    }

    #[test]
    #[should_panic]
    fn unterminated_str() {
        base_check_no_errors("\"This is an unterminated string\n".to_string());
    }

    #[test]
    fn correct_lineno_for_unterminted_str() {
        let scanner = setup();
        let result = scanner.scan(read_json_file(
            "scan_test_data/error_at_correct_location.json",
        ));

        let error_handler = result.unwrap_err();

        let first_error = &error_handler.errors[0];

        error_handler.print_errors();
        assert_eq!(first_error.line, 4);
        assert_eq!(first_error.column, 38);
    }
}
