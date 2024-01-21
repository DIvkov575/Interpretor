#[derive(Debug, PartialEq)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Symbol(String),
    Dot,
    Text(String),
    Quote,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SourcePos {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub pos: SourcePos,
    pub token: TokenType,
}


