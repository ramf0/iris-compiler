#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum TokenValue<'a> {
    StringLiteral(&'a str),
    
    Identifier(&'a str),
    LParen,
    RParen,
    
    Eof,
    NewLine,
    Illegal,
}
#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub value: TokenValue<'a>,
    pub line: usize,
    pub column: usize,
}
impl<'a> Token<'a> {
    pub fn new(value: TokenValue<'a>, line: usize, column: usize) -> Token<'a> {
        Token {
            value: value,
            line: line,
            column: column,
        }
    }
}