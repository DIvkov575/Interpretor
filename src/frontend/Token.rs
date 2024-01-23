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

impl SourcePos {
    fn new(line: u32, column: u32) -> SourcePos {
        SourcePos { line, column }
    }
}


#[derive(Debug, PartialEq)]
pub struct Token {
    pub pos: SourcePos,
    pub token: TokenType,
}


