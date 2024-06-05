// use crate::types::{Token, TokenType, UnsignedInt};
// use std::{collections::HashMap, slice::Iter};
//
// enum Value {
//     Null,
//     Bool(bool),
//     Number(UnsignedInt),
//     String(String),
//     Array(Vec<Value>),
//     Object(HashMap<String, Value>),
// }
//
// fn parse(tokens: Vec<Token>) -> Json {
//     let mut tokens = tokens.iter();
//
//     while let Some(token) = tokens.next() {
//         match token.token_type {
//             TokenType::String => return Json::String(token.lexeme.clone()),
//             _ => return Json::Object,
//         }
//     }
//
//     Json::Number
// }
