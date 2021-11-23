use crate::token::Token;
use crate::expr::Expr;

pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
}

pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct ExpressionStmt {
    pub expr: Box<Expr>,
}

