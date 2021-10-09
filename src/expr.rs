use crate::token::{Token};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Number(NumberExpr),
    Boolean(bool),
    String(String),
}

#[derive(Clone, Debug)]
pub struct GroupExpr {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub oper: Token,
    pub right: Box<Expr>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.oper.lexeme, self.left, self.right)
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub oper: Token,
    pub right: Box<Expr>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", self.oper.lexeme, self.right)
    }
}

#[derive(Clone, Debug)]
pub struct NumberExpr {
    pub value: f64,
}

impl fmt::Display for NumberExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Expr::Binary(expr)  => expr.fmt(f),
            Expr::Unary(expr)   => expr.fmt(f),
            Expr::Number(expr)  => expr.fmt(f),
            Expr::Boolean(expr) => write!(f, "{}", expr),
            Expr::String(expr)  => write!(f, "{}", expr),
        }
    }
}
