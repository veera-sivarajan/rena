use crate::expr::Expr;

pub fn print_ast(expr: &Expr) {
    match expr {
        Expr::Number(float) => println!("{}", float),
        Expr::String(str_value) => println!("{}", str_value),
        Expr::Boolean(value) => println!("{}", value),
        Expr::Binary(b_value) => println!("{}", b_value),
        Expr::Unary(u_value) => println!("{}", u_value),
    }
}
