use crate::expr::Expr;
use crate::token::Token;
pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    Let(LetStmt),
    If(IfStmt),
}

pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Option<Stmt>>,
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
