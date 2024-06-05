use crate::types::{Token, TokenType, UnsignedInt};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(UnsignedInt),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn parse(tokens: Vec<Token>) -> Value {
    let mut tokens = tokens.iter().peekable();

    let json = None;

    while let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::String => return Value::String(token.lexeme.clone()),
            _ => return Value::Null,
        }
    }

    json.expect("Json may not be None")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_string() {
        let tokens = vec![Token {
            lexeme: "string".to_string(),
            token_type: TokenType::String,
            line: 1,
            column: 1,
        }];

        let json = parse(tokens);

        assert_eq!(json, Value::String("string".to_string()));
    }
}
