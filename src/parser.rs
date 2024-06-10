use crate::types::{Token, TokenType, UnsignedInt};
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(PartialEq, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(UnsignedInt),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug)]
pub enum ParseError {
    NoTokens,
    UnknownToken,
    UnexpectedToken(String),
}

type ValueResult = Result<Value, ParseError>;
type Tokens<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(tokens: Vec<Token>) -> ValueResult {
    let tokens: Tokens = tokens.iter().peekable();
    value(tokens)
}

const VALID_TOKEN_MSG: &str = "Tokenized value should be valid";

fn value(mut tokens: Tokens) -> ValueResult {
    if let Some(token) = tokens.next() {
        match token.token_type {
            // Scalar json types
            TokenType::Null => return Ok(Value::Null),
            TokenType::Bool => {
                let parsed: bool = token.lexeme.parse().expect(VALID_TOKEN_MSG);
                return Ok(Value::Bool(parsed));
            }
            TokenType::Number => {
                let parsed: UnsignedInt = token.lexeme.parse().expect(VALID_TOKEN_MSG);
                return Ok(Value::Number(parsed));
            }
            TokenType::String => return Ok(Value::String(token.lexeme.clone())),

            // Compound json types
            TokenType::OpeningBrace => return array(tokens),

            _ => return Err(ParseError::UnknownToken),
        };
    }
    Err(ParseError::NoTokens)
}

fn array(mut tokens: Tokens) -> ValueResult {
    // array = '[' value (, value)* ']'
    let mut json_array: Vec<Value> = Vec::new();

    if let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::ClosingBrace => {
                tokens.next();
                return Ok(Value::Array(json_array));
            }
            TokenType::OpeningSquirly => {
                return Err(ParseError::UnexpectedToken(token.lexeme.clone()))
            }
            _ => json_array.push(value(tokens)?),
        }
    }
    Err(ParseError::NoTokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a json array with optional tokens between the brackets.
    ///
    /// This serves as a quik way to create arrays for testing purposes.
    /// If no inner tokens are requires pass 'Vec::New' as an argument.
    ///
    ///
    /// # Example
    ///
    /// [ token1, token2, token3 ]
    fn generate_array<F>(inner_tokens: F) -> Vec<Token>
    where
        F: FnOnce() -> Vec<Token>,
    {
        vec![Token {
            lexeme: "[".to_string(),
            token_type: TokenType::OpeningBrace,
            line: 1,
            column: 1,
        }]
        .into_iter()
        .chain(inner_tokens())
        .chain(vec![Token {
            lexeme: "]".to_string(),
            token_type: TokenType::ClosingBrace,
            line: 1,
            column: 2,
        }])
        .collect::<Vec<Token>>()
    }

    /// Returns a token with TokenType::Null
    fn null_token() -> Token {
        Token {
            lexeme: String::from("null"),
            token_type: TokenType::Null,
            line: 1,
            column: 1,
        }
    }

    #[test]
    fn parses_json_string() -> Result<(), ParseError> {
        let tokens = vec![Token {
            lexeme: "string".to_string(),
            token_type: TokenType::String,
            line: 1,
            column: 1,
        }];

        let json = parse(tokens)?;

        if json == Value::String("string".to_string()) {
            return Ok(());
        }
        Err(ParseError::UnknownToken)
    }

    #[test]
    fn parses_json_null() -> Result<(), ParseError> {
        let tokens = vec![Token {
            lexeme: "null".to_string(),
            token_type: TokenType::Null,
            line: 1,
            column: 1,
        }];
        let json = parse(tokens)?;

        if json == Value::Null {
            return Ok(());
        }
        Err(ParseError::UnknownToken)
    }

    #[test]
    fn parses_unsigned_integer() {
        let tokens = vec![Token {
            lexeme: "123456789".to_string(),
            token_type: TokenType::Number,
            line: 1,
            column: 1,
        }];

        let json = parse(tokens).unwrap();
        assert_eq!(json, Value::Number(123456789));
    }

    #[test]
    fn parses_single_bool_value() {
        let tokens_t = vec![Token {
            lexeme: "true".to_string(),
            token_type: TokenType::Bool,
            line: 1,
            column: 1,
        }];
        let tokens_f = vec![Token {
            lexeme: "false".to_string(),
            token_type: TokenType::Bool,
            line: 1,
            column: 1,
        }];

        let json_t = parse(tokens_t).unwrap();
        let json_f = parse(tokens_f).unwrap();

        assert_eq!(json_t, Value::Bool(true));
        assert_eq!(json_f, Value::Bool(false));
    }

    #[test]
    fn parses_empty_array() -> Result<(), String> {
        let tokens = generate_array(Vec::new);
        let json = parse(tokens).expect("Input should be a valid list of tokens");
        match json {
            Value::Array(_) => Ok(()),
            _ => Err(String::from("Not an array")),
        }
    }

    #[test]
    fn parses_array_with_one_value() {
        let tokens = generate_array(|| vec![null_token()]);
        let json_value = parse(tokens).unwrap();
        match json_value {
            Value::Array(v) => assert_eq!(*v.first().unwrap(), Value::Null),
            v => panic!("Not an array, value: {:?}", v),
        }
    }
}
