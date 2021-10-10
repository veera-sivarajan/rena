use crate::expr::{Expr, BinaryExpr, NumberExpr, UnaryExpr};
use crate::token::TokenType;

pub enum Value {
    Number(f64),
    Bool(bool),
    String(String), 
    Error(String),
}

fn stringify(result: Value) -> String {
    match result {
        Value::Number(num) => format!("{}", num),
        Value::Bool(tof) => format!("{}", tof),
        Value::String(value) => format!("{}", value),
        Value::Error(message) => format!("Error: {}", message),
    }
}

pub fn interpret(expression: Expr) {
    let result = evaluate(expression);
    println!("{}", stringify(result));
}

fn evaluate(expression: Expr) -> Value {
    match expression {
        Expr::Binary(expr) => intpt_binary(expr),
        Expr::Unary(expr)  => intpt_unary(expr),
        Expr::Number(expr) => intpt_number(expr),
        Expr::Boolean(expr) => intpt_boolean(expr),
        Expr::String(expr) => intpt_string(expr),
    }
}

fn intpt_unary(expression: UnaryExpr) -> Value {
    let right = evaluate(*expression.right);
    match expression.oper.token_type {
        TokenType::Minus => {
            match right {
                Value::Number(num) => Value::Number(-num),
                _ => Value::Error(String::from("Operand should be a number.")),
            }
        },
        TokenType::Bang =>{
            match right {
                Value::Bool(value) => Value::Bool(!value),
                _ => Value::Bool(false),
            }
        },
        _ => Value::Error(String::from("Unknown unary operation")),
    }
}

fn intpt_boolean(expression: bool) -> Value {
    Value::Bool(expression)
}

fn intpt_string(expression: String) -> Value {
    Value::String(expression)
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
                _ => Value::Error(String::from("Unknown binary operation")),
            }
        },
        _ => Value::Error(String::from("Operands should be numbers.")),
    }
}
    
