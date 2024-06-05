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

pub type UnsignedInt = u16;

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: UnsignedInt,
    pub column: UnsignedInt,
}
