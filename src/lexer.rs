use crate::token::Token;
use crate::token::Token::{Assign, Comma, Illegal, LBrace, LParen, Plus, RBrace, RParen, Semicolon};

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
        let tok = match self.ch {
            b'=' => Assign,
            b'+' => Plus,
            b'(' => LParen,
            b')' => RParen,
            b'{' => LBrace,
            b'}' => RBrace,
            b',' => Comma,
            b';' => Semicolon,
            _ => Illegal,
        };

        self.read_char();

        return tok
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let test_cases = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for test_case in test_cases {
            let token = lexer.next_token();

            assert_eq!(token, test_case)
        }

    }
}