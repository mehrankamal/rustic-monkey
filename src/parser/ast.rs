use std::collections::hash_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Program(Program),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub(crate) fn new() -> Program {
        let mut program = Program{
            statements: vec![]
        };

        program
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let(Let),
    Return(Option<Expr>),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    pub ident: Ident,

    // TODO: Remove Option once expression parsing is done.
    pub expr: Option<Expr>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(Ident),
    IntLiteral(i64),
    PrefixExpr{
        expr: Box<Expr>,
        operator: Option<PrefixOperator>,
    },
    InfixExpr{
        left: Box<Expr>,
        right: Box<Expr>,
        operator: InfixOperator,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefixOperator {
    Not,
    Negate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InfixOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String
}


#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ExprPrecedence {
    LOW,
    EQUALS, // ==
    COMPARE, // > or <
    SUM, // +
    PRODUCT, // *
    PREFIX, // -X or !X
    CALL,
}

