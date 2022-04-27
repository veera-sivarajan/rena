use crate::expr::Expr;
use crate::token::Token;
#[derive(Clone)]
pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunStmt),
    Return(ReturnStmt),
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub init: Option<Expr>,
}

#[derive(Clone)]
pub struct PrintStmt {
    pub expr: Expr,
}

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

#[derive(Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone)]
pub struct FunStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}
    
