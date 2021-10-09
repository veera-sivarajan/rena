use crate::expr::{Expr, BinaryExpr, NumberExpr};
use crate::token::TokenType;

pub enum Value {
    Number(f64),
    Bool(bool),
}

fn stringify(result: Value) -> String {
    match result {
        Value::Number(num) => format!("{}", num),
        Value::Bool(tof) => format!("{}", tof),
    }
}

pub fn interpret(expression: Expr) {
    let result = evaluate(expression);
    println!("{}", stringify(result));
}

fn evaluate(expression: Expr) -> Value {
    match expression {
        Expr::Binary(expr) => intpt_binary(expr),
        // Expr::Unary(expr)  => intpt_unary(expr),
        Expr::Number(expr) => intpt_number(expr),
        // Expr::Boolean(expr) => intpt_boolean(expr),
        // Expr::String(expr) => intpt_string(expr),
        _ => Value::Number(0.0),
    }
}

fn intpt_number(expression: NumberExpr) -> Value {
    Value::Number(expression.value)
}

fn intpt_binary(expression: BinaryExpr) -> Value {
    let left = evaluate(*expression.left);
    let right = evaluate(*expression.right);

    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            match expression.oper.token_type {
                TokenType::Plus => {
                    let sum = l + r;
                    Value::Number(sum)
                },
                TokenType::Minus => {
                    let diff = l - r;
                    Value::Number(diff)
                },
                TokenType::Slash => {
                    let quotient = l / r;
                    Value::Number(quotient)
                },
                TokenType::Star => {
                    let prod = l * r;
                    Value::Number(prod)
                },
                _ => Value::Number(0.0),
            }
        },
        _ => Value::Number(0.0),
    }
}
    
