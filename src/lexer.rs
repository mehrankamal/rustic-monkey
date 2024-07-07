use crate::token::Token;
use crate::token::Token::*;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, Token> = {
        let mut keywords_lookup = HashMap::new();
        keywords_lookup.insert("fn".to_string(), Function);
        keywords_lookup.insert("let".to_string(), Let);
        keywords_lookup.insert("if".to_string(), If);
        keywords_lookup.insert("else".to_string(), Else);
        keywords_lookup.insert("return".to_string(), Return);
        keywords_lookup.insert("true".to_string(), True);
        keywords_lookup.insert("false".to_string(), False);
        keywords_lookup
    };
}

pub(crate) struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.consume_char();

        lexer
    }

    fn consume_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char(b'=') {
                    self.consume_char();
                    Eq
                } else {
                    Assign
                }
            }
            b'+' => Plus,
            b'-' => Minus,
            b'!' => {
                if self.peek_char(b'=') {
                    self.consume_char();
                    NotEq
                } else {
                    Bang
                }
            }
            b'*' => Asterisk,
            b'/' => Slash,
            b'<' => LT,
            b'>' => GT,
            b'(' => LParen,
            b')' => RParen,
            b'{' => LBrace,
            b'}' => RBrace,
            b',' => Comma,
            b';' => Semicolon,
            0 => Eof,

            _ => {
                if is_letter(self.ch) {
                    let ident_literal = self.consume_ident();
                    return lookup_ident(&ident_literal);
                } else if is_digit(self.ch) {
                    return self.consume_number();
                } else {
                    Illegal
                }
            }
        };

        self.consume_char();

        return tok;
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' {
            self.consume_char();
        }
    }

    fn consume_ident(&mut self) -> String {
        let current_position = self.position;
        while is_letter(self.ch) {
            self.consume_char()
        }

        return String::from(&self.input[current_position..self.position]);
    }

    fn consume_number(&mut self) -> Token {
        let start_position = self.position;

        while is_digit(self.ch) {
            self.consume_char()
        }

        Int(self.input[start_position..self.position]
            .parse::<i64>()
            .unwrap())
    }

    fn peek_char(&self, peek: u8) -> bool {
        if self.read_position >= self.input.len() {
            0 == peek
        } else {
            self.input.as_bytes()[self.read_position] == peek
        }
    }
}

fn lookup_ident(name: &String) -> Token {
    match KEYWORDS.get(name) {
        Some(keyword) => keyword.clone(),
        None => Ident(String::from(name))
    }
}

fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}

fn is_letter(ch: u8) -> bool {
    (ch >= b'a' && ch <= b'z') || (b'A' <= ch && ch <= b'Z') || (ch == b'_')
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::Lexer;
    use crate::token::Token::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let test_cases = vec![
            Let,
            Ident(String::from("five")),
            Assign,
            Int(5),
            Semicolon,
            Let,
            Ident(String::from("ten")),
            Assign,
            Int(10),
            Semicolon,
            Let,
            Ident(String::from("add")),
            Assign,
            Function,
            LParen,
            Ident(String::from("x")),
            Comma,
            Ident(String::from("y")),
            RParen,
            LBrace,
            Ident(String::from("x")),
            Plus,
            Ident(String::from("y")),
            Semicolon,
            RBrace,
            Semicolon,
            Let,
            Ident(String::from("result")),
            Assign,
            Ident(String::from("add")),
            LParen,
            Ident(String::from("five")),
            Comma,
            Ident(String::from("ten")),
            RParen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int(5),
            Semicolon,
            Int(5),
            LT,
            Int(10),
            GT,
            Int(5),
            Semicolon,
            If,
            LParen,
            Int(5),
            LT,
            Int(10),
            RParen,
            LBrace,
            Return,
            True,
            Semicolon,
            RBrace,
            Else,
            LBrace,
            Return,
            False,
            Semicolon,
            RBrace,
            Int(10),
            Eq,
            Int(10),
            Semicolon,
            Int(10),
            NotEq,
            Int(9),
            Semicolon,
            Eof,
        ];

        let mut lexer = Lexer::new(input);

        for test_case in test_cases {
            let token = lexer.next_token();

            assert_eq!(token, test_case)
        }
    }
}
