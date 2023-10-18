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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String
}


pub enum ExprPrecedence {
    LOW,
    EQUALS, // ==
    COMPARE, // > or <
    SUM, // +
    PRODUCT, // *
    PREFIX, // -X or !X
    CALL,
}

