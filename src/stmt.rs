use crate::expr::Expr;
use crate::token::Token;
#[derive(Clone)]
pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    Let(LetStmt),
    If(IfStmt),
    While(WhileStmt),
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}

#[derive(Clone)]
pub struct LetStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}

#[derive(Clone)]
pub struct PrintStmt {
    pub expr: Box<Expr>,
}

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expr: Box<Expr>,
}

#[derive(Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}
