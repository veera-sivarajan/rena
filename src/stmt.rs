use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    Let(LetStmt),
}

pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}

pub struct LetStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct ExpressionStmt {
    pub expr: Box<Expr>,
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}
