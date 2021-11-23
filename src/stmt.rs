use crate::token::Token;
use crate::expr::Expr;
<<<<<<< HEAD
=======

>>>>>>> stream

pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
<<<<<<< HEAD
}

pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
=======
    Var(VarStmt),
>>>>>>> stream
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct ExpressionStmt {
    pub expr: Box<Expr>,
<<<<<<< HEAD
=======
}

pub struct VarStmt {
    pub name: Token,
    pub init: Option<Box<Expr>>,
>>>>>>> stream
}
    
