use crate::types::{Token, TokenType, UnsignedInt};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(UnsignedInt),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn print(&self) {
        match self {
            Self::Null => println!("null"),
            Self::Bool(b) => println!("{b}"),
            Self::Number(n) => println!("{n}"),
            Self::String(s) => println!("{s}"),
            _ => todo!("Implement other variants"),
        }
    }
}

type Tokens = Vec<Token>;
type ValueResult<'a> = Result<Value, ParseError<'a>>;

#[derive(Debug)]
pub enum ParseError<'a> {
    BadToken(&'a Token),
    EndOfFile,
}

pub struct Parser<'a> {
    cursor: usize,
    tokens: &'a Tokens,
}

impl<'t> Parser<'t> {
    pub fn parse(tokens: &Tokens) -> ValueResult {
        let mut parser = Parser { cursor: 0, tokens };

        let result = parser.value();
        if let Err(result) = &result {
            match result {
                ParseError::BadToken(token) => result,
                ParseError::EndOfFile => result,
            };
        };

        Ok(result.unwrap())
    }

    fn value(&'t mut self) -> ValueResult {
        let token = match self.advance() {
            Some(token) => token,
            None => return Err(ParseError::EndOfFile),
        };

        match token.token_type {
            TokenType::Null => Ok(Value::Null),
            TokenType::Bool => {
                let boolean: bool = token
                    .lexeme
                    .parse()
                    .expect("Should contain a valid boolean");
                Ok(Value::Bool(boolean))
            }
            TokenType::Number => {
                let number: UnsignedInt = token
                    .lexeme
                    .parse()
                    .expect("Token should contain valid number");
                Ok(Value::Number(number))
            }
            TokenType::OpeningBrace => Ok(self.array()?),
            _ => todo!("Implement the other tokens"),
        }
    }

    fn array(&'t mut self) -> ValueResult {
        let token = match self.advance() {
            Some(token) => token,
            None => return Err(ParseError::EndOfFile),
        };

        let mut array = Vec::new();

        match token.token_type {
            TokenType::ClosingBrace => (),
            TokenType::Comma | TokenType::OpeningBrace | TokenType::ClosingSquirly => {
                return Err(ParseError::BadToken(token));
            }
            _ => array.push(self.value()?),
        }

        Ok(Value::Array(array))
    }

    fn advance(&mut self) -> Option<&'t Token> {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan;

    #[test]
    fn only_one_null() {
        assert_eq!(test::parse("null"), Value::Null);
    }

    #[test]
    fn only_one_bool() {
        assert_eq!(test::parse("true"), Value::Bool(true));
        assert_eq!(test::parse("false"), Value::Bool(false));
    }

    #[test]
    fn only_one_number() {
        assert_eq!(test::parse("123456789"), Value::Number(123456789));
    }

    #[test]
    fn empty_array() {
        assert_eq!(test::parse("[]"), Value::Array(Vec::new()));
    }

    #[test]
    fn array_with_one_null_value() {
        assert_eq!(test::parse("[null]"), Value::Array(vec![Value::Null]));
    }

    mod test {
        use super::*;

        pub fn parse(s: &str) -> Value {
            Parser::parse(&create_tokens(s.to_string())).unwrap()
        }

        fn create_tokens(s: String) -> Tokens {
            let scanner = scan::Scanner::new(scan::ScannerErrorHandler::new());
            match scanner.scan(s) {
                Ok(tokens) => tokens,
                Err(_) => panic!("Should be a valid string"),
            }
        }
    }
}
