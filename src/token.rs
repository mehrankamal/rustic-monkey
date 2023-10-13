
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(i64),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let
}