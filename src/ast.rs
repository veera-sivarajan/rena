// NOTE This module is not used by the project anymore
// but leaving it here assuming I'd need it in the future
use crate::expr::Expr;

pub fn print_ast(expr: &Expr) {
    match expr {
        Expr::Number(float) => println!("{}", float),
        Expr::String(str_value) => println!("{}", str_value),
        Expr::Boolean(value) => println!("{}", value),
        Expr::Binary(b_value) => println!("{}", b_value),
        Expr::Unary(u_value) => println!("{}", u_value),
        Expr::Variable(u_value) => println!("{}", u_value),
        Expr::Assign(value) => println!("{}", value),
    }
}
