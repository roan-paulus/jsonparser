#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    Comma,
    OpeningBrace,
    ClosingBrace,
    OpeningSquirly,
    ClosingSquirly,
    String,
    Bool,
    Number,
    Null,
}

pub type UnsignedInt = u32;

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: UnsignedInt,
    pub column: UnsignedInt,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            lexeme: self.lexeme.clone(),
            token_type: self.token_type.clone(),
            line: self.line,
            column: self.column,
        }
    }
}
