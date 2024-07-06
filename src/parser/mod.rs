mod ast;

use std::collections::HashMap;

use ast::InfixOperator;

use crate::lexer::Lexer;
use crate::parser::ast::{Expr, ExprPrecedence, Ident, Let, PrefixOperator, Program, Stmt};
use crate::token::Token;

type ParseError = String;

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_tok: Token,
    peek_tok: Token,
}

lazy_static! {
    static ref PRECEDENCES: HashMap<Token, ExprPrecedence> = {
        let mut map = HashMap::new();
        map.insert(Token::Eq, ExprPrecedence::EQUALS);
        map.insert(Token::NotEq, ExprPrecedence::EQUALS);
        map.insert(Token::LT, ExprPrecedence::COMPARE);
        map.insert(Token::GT, ExprPrecedence::COMPARE);
        map.insert(Token::Plus, ExprPrecedence::SUM);
        map.insert(Token::Minus, ExprPrecedence::SUM);
        map.insert(Token::Slash, ExprPrecedence::PRODUCT);
        map.insert(Token::Asterisk, ExprPrecedence::PRODUCT);
        map.insert(Token::LParen, ExprPrecedence::CALL);
        map
    };
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
            _ => Ok(self.parse_expression_stmt()?),
        }
    }

    fn parse_expression_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.parse_expression(ExprPrecedence::LOW)?;

        if self.peek_tok_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::Expr(expression))
    }

    fn parse_expression(&mut self, precedence: ExprPrecedence) -> Result<Expr, ParseError> {

        let mut left = self.parse_prefix_expression()?;

        while !self.peek_tok_is(&Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left = self.parse_infix_expression(left)?;
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expr, ParseError> {
        match self.cur_tok.clone() {
            Token::Ident(name) => Ok(self.parse_ident_expr(name.clone())?),
            Token::Int(value) => Ok(Expr::IntLiteral(value)),
            Token::Bang => self.parse_bang_expr(),
            Token::Minus => self.parse_negate_expr(),
            _ => Err(format!("No Prefix parse function registered for {}", self.cur_tok))
        }
    }

    fn parse_infix_expression(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let operator = match self.cur_tok {
            Token::Eq => InfixOperator::Equals,
            Token::NotEq => InfixOperator::NotEquals,
            Token::GT => InfixOperator::GreaterThan,
            Token::LT => InfixOperator::LessThan,
            Token::Plus => InfixOperator::Add,
            Token::Minus => InfixOperator::Sub,
            Token::Slash => InfixOperator::Div,
            Token::Asterisk => InfixOperator::Mul,
            _ => Err(format!("No Infix parse function registered for {}", self.cur_tok))?
        };

        let precedence = self.curr_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(Expr::InfixExpr { left: Box::new(left), right: Box::new(right), operator:  operator})

    }

    fn parse_negate_expr(&mut self) -> Result<Expr, ParseError> {
        let operator = PrefixOperator::Negate;
        self.next_token();
        let right_expression = self.parse_expression(ExprPrecedence::PREFIX)?;

        Ok(Expr::PrefixExpr {
            expr: Box::new(right_expression),
            operator: Some(operator),
        })
    }

    fn parse_bang_expr(&mut self) -> Result<Expr, ParseError> {
        let operator = PrefixOperator::Not;
        self.next_token();
        let right_expression = self.parse_expression(ExprPrecedence::PREFIX)?;

        Ok(Expr::PrefixExpr {
            expr: Box::new(right_expression),
            operator: Some(operator),
        })
    }

    fn parse_ident_expr(&mut self, ident_name: String) -> Result<Expr, ParseError> {
        Ok(Expr::Ident(Ident { name: ident_name}))
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

    fn peek_tok_is(&self, tok: &Token) -> bool {
        self.peek_tok == *tok
    }

    fn peek_precedence(&self) -> ExprPrecedence {
        match PRECEDENCES.get(&self.peek_tok) {
            Some(precedence) => precedence.clone(),
            None => ExprPrecedence::LOW,
        }
    }

    fn curr_precedence(&self) -> ExprPrecedence {
        match PRECEDENCES.get(&self.cur_tok) {
            Some(precedence) => precedence.clone(),
            None => ExprPrecedence::LOW,
        }
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
    use crate::parser::ast::{Expr, PrefixOperator, Stmt};
    use crate::parser::Parser;

    use super::ast::InfixOperator;

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

    #[test]
    fn test_identifier_expressions() {
        let input = r#"foobar; gmail;"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_eq!(false, program.is_err(), "Error occurred while parsing, got error: {}", program.err().unwrap());

        let program = program.unwrap();

        let expected_idents = ["foobar", "gmail"];
        assert_eq!(expected_idents.len(), program.statements.len());

        for (idx, statement) in program.statements.iter().enumerate() {
            assert!(matches!(statement, Stmt::Expr(expr) if
                                                        matches!(&expr, Expr::Ident(ident) if
                                                                    *ident.name == expected_idents[idx].to_string())));
        }
    }

    #[test]
    fn test_integer_literal_expressions() {
        let input = r#"10; 11;"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_eq!(false, program.is_err(), "Error occurred while parsing, got error: {}", program.err().unwrap());

        let program = program.unwrap();

        let expected_ints: [i64; 2] = [10, 11];
        assert_eq!(expected_ints.len(), program.statements.len());

        for (idx, statement) in program.statements.iter().enumerate() {
            assert!(matches!(statement, Stmt::Expr(expr) if
                                                        matches!(&expr, Expr::IntLiteral(number) if
                                                                    *number == expected_ints[idx])));
        }
    }

    struct PrefixParseTestCase<'a> {
        input: &'a str,
        operator: PrefixOperator,
        literal_value: i64,
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let mut test_cases:Vec<PrefixParseTestCase> = Vec::new();

        test_cases.push(PrefixParseTestCase{input: "!5", operator: PrefixOperator::Not, literal_value: 5});
        test_cases.push(PrefixParseTestCase{input: "-5", operator: PrefixOperator::Negate, literal_value: 5});

        for test_case in test_cases {
            let l = Lexer::new(test_case.input);
            let mut p = Parser::new(l);

            let program = p.parse_program();

            assert_eq!(false, program.is_err(), "Error occurred while parsing, got error: {}", program.err().unwrap());

            let program = program.unwrap();

            assert_eq!(1, program.statements.len());


            assert!(
                matches!(program.statements[0].clone(), Stmt::Expr(expr)
                    if matches!(&expr, Expr::PrefixExpr { expr: prefix_expr, operator: oper }
                        if matches!(oper, Some(operator)
                            if *operator == test_case.operator)
                        && matches!(**(prefix_expr), Expr::IntLiteral(literal_value)
                            if (literal_value == test_case.literal_value))
                    )
                )
            );
        }
    }

    struct InfixParseTestCase<'a> {
        input: &'a str,
        left_value: i64,
        operator: InfixOperator,
        right_value: i64,
    }

    #[test]
    fn test_parse_infix_expsressions() {
        let mut test_cases:Vec<InfixParseTestCase> = Vec::new();
        test_cases.push(InfixParseTestCase{input: "5 + 5", left_value: 5, operator: InfixOperator::Add, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 - 5", left_value: 5, operator: InfixOperator::Sub, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 * 5", left_value: 5, operator: InfixOperator::Mul, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 / 5", left_value: 5, operator: InfixOperator::Div, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 == 5", left_value: 5, operator: InfixOperator::Equals, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 != 5", left_value: 5, operator: InfixOperator::NotEquals, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 < 5", left_value: 5, operator: InfixOperator::LessThan, right_value: 5});
        test_cases.push(InfixParseTestCase{input: "5 > 5", left_value: 5, operator: InfixOperator::GreaterThan, right_value: 5});

        for test_case in test_cases {
            let lexer = Lexer::new(test_case.input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program().unwrap();

            assert_eq!(1, program.statements.len());

            dbg!(test_case.input);
            dbg!(&program);

            let stmt = program.statements[0].clone();
            assert!(matches!(stmt, Stmt::Expr(expr) if
                matches!(&expr, Expr::InfixExpr { left: left_expr, operator: oper, right: right_expr }
                    if matches!(**left_expr, Expr::IntLiteral(left_value)
                        if left_value == test_case.left_value)
                    && matches!(**right_expr, Expr::IntLiteral(right_value)
                        if right_value == test_case.right_value)
                    && *oper == test_case.operator
                )
            ));
            dbg!(&"Passed");
        }
    }
}