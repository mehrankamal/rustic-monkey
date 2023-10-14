mod ast;

use std::fmt::format;
use crate::lexer::Lexer;
use crate::parser::ast::{Ident, Let, Program, Stmt};
use crate::token::Token;

type ParseError = String;

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_tok: Token,
    peek_tok: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            l: lexer,

            cur_tok: Token::Illegal,
            peek_tok: Token::Illegal,
        };

        parser.next_token();
        parser.next_token();
        
        return parser;
    }


    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();

        while self.cur_tok != Token::Eof {
            let stmt = self.parse_stmt()?;
            program.statements.push(stmt);

            self.next_token();
        }

        Ok(program)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.cur_tok {
            Token::Let => Ok(self.parse_let_stmt()?),
            _ => Err("Parsing failed".to_string()),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.next_token();

        let mut ident_name = "".to_string();

        match self.cur_tok.clone() {
            Token::Ident(name) => { ident_name = name },
            other => return Err(format!("{} is not an identifier", other))
        };

        self.next_token();
        self.expect_peek(&Token::Assign)?;

        while !self.curr_tok_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::Let(Let{ ident: Ident{ name: ident_name }, expr: None}))
    }

    fn curr_tok_is(&self, tok: &Token) -> bool {
        self.cur_tok == *tok
    }

    fn expect_peek(&mut self, tok: &Token) -> Result<(), ParseError>{
        if self.cur_tok == *tok {
            self.next_token();
            Ok(())
        } else {
            let e = format!("expected token: {} got: {}", tok, self.cur_tok);
            Err(e)
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::lexer::Lexer;
    use crate::parser::ast::{Stmt};
    use crate::parser::{Parser};

    #[test]
    fn test_let_statement() {
        let input = r#"
        let x = 10;
        let y = 100;
        let foo = 838383;"#;

        let l = Lexer::new(input);
        let mut parser = Parser::new(l);

        let program = parser.parse_program();

        assert_eq!(None, program.as_ref().err());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        let expected_idents = ["x", "y", "foo"];
        for (idx, stmt) in program.statements.iter().enumerate() {
            assert!(matches!(stmt, Stmt::Let(let_stmt) if let_stmt.ident.name == expected_idents[idx]));
        }
    }
}