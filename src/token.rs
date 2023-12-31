use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(i64),
    True,
    False,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Eq,
    NotEq,

    LT,
    GT,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    If,
    Else,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TokenType({:?})", self)
    }
}