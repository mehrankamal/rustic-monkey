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
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    pub ident: Ident,
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

