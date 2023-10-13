use std::collections::HashMap;
use crate::token::Token;
use crate::token::Token::{Assign, Asterisk, Bang, Comma, Function, GT, Ident, Illegal, Int, LBrace, Let, LParen, LT, Minus, Plus, RBrace, RParen, Semicolon, Slash};

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
         let mut lexer =  Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let tok = match self.ch {
            b'=' => Assign,
            b'+' => Plus,
            b'-' => Minus,
            b'!' => Bang,
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

            _ => {
                if is_letter(self.ch) {
                    let ident_literal = self.read_ident();
                    return lookup_ident(&ident_literal)
                } else if is_digit(self.ch) {
                    return self.read_number()
                } else {
                    Illegal
                }
            },
        };

        self.read_char();

        return tok
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let current_position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }

        return String::from(&self.input[current_position..self.position])
    }

    fn read_number(&mut self) -> Token {
        let start_position = self.position;

        while is_digit(self.ch) {
            self.read_char()
        }

        Int(self.input[start_position..self.position].parse::<i64>().unwrap())
    }
}

fn lookup_ident(name: &String) -> Token {
    let mut keywords_lookup: HashMap<String, Token> = HashMap::new();
    keywords_lookup.insert("fn".to_string(), Function);
    keywords_lookup.insert("let".to_string(), Let);


    if keywords_lookup.contains_key(name) {
        keywords_lookup[name].clone()
    } else {
        Ident(String::from(name))
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
    use crate::token::Token::{Assign, Asterisk, Bang, Comma, Function, GT, Ident, Int, LBrace, Let, LParen, LT, Minus, Plus, RBrace, RParen, Semicolon, Slash};

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;"#;

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
            Semicolon
        ];

        let mut lexer = Lexer::new(input);

        for test_case in test_cases {
            let token = lexer.next_token();

            assert_eq!(token, test_case)
        }

    }
}