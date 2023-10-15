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
            Token::Return => Ok(self.parse_return_stmt()?),
            _ => Err("Parsing failed".to_string()),
        }
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {

        // TODO: To parse expression for return stmt once done with parsing them.
        while !self.curr_tok_is(&Token::Semicolon) {
            self.next_token();
        }


        Ok(Stmt::Return(None))
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.next_token();

        let ident = match self.cur_tok.clone() {
            Token::Ident(name) => Ok(Ident{ name }),
            other=> Err(format!("Expected Ident token, got {}", other))
        }?;

        self.next_token();
        self.expect_peek(Token::Assign)?;

        // TODO: To be wired with Expression parsing once done.
        while !self.curr_tok_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::Let(Let{ ident, expr: None}))
    }

    fn curr_tok_is(&self, tok: &Token) -> bool {
        self.cur_tok == *tok
    }

    fn expect_peek(&mut self, tok: Token) -> Result<(), ParseError>{
        return if self.cur_tok == tok {
            self.next_token();
            Ok(())
        } else {
            let e = format!("expected peek token: {} got: {}", tok, self.cur_tok.clone());
            Err(e)
        };
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

        assert_eq!(false, program.is_err(), "Error occurred while parsing, got error: {}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        let expected_idents = ["x", "y", "foo"];
        for (idx, stmt) in program.statements.iter().enumerate() {
            assert!(matches!(stmt, Stmt::Let(let_stmt) if let_stmt.ident.name == expected_idents[idx]));
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
        return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_eq!(false, program.is_err());

        let program = program.unwrap();
        assert_eq!(3, program.statements.len());

        for (_, stmt) in program.statements.iter().enumerate() {
            assert!(matches!(stmt, Stmt::Return(_)))
        }
    }
}