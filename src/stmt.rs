use crate::token::Token;
use crate::expr::Expr;


pub enum Stmt {
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Var(VarStmt),
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct ExpressionStmt {
    pub expr: Box<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
}
