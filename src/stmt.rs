pub struct PrintStmt {
    pub expression: Box<Expr>,
}

pub enum Stmt {
    Print(PrintStmt),
}
