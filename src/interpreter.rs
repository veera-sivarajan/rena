use crate::err::RError;
use crate::expr::{BinaryExpr, Expr, NumberExpr, UnaryExpr};
use crate::token::TokenType;

pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
}

fn stringify(result: Value) -> String {
    match result {
        Value::Number(num) => format!("{}", num),
        Value::Bool(tof) => format!("{}", tof),
        Value::String(value) => format!("{}", value),
    }
}

pub fn interpret(expression: Expr) {
    match evaluate(expression) {
        Ok(value) => println!("{}", stringify(value)),
        Err(runtime_error) => println!("{}", runtime_error.to_string()),
    }
}

fn evaluate(expression: Expr) -> Result<Value, RError> {
    match expression {
        Expr::Binary(expr) => binary(expr),
        Expr::Unary(expr) => unary(expr),
        Expr::Number(expr) => number(expr),
        Expr::Boolean(expr) => boolean(expr),
        Expr::String(expr) => string(expr),
    }
}

fn unary(expression: UnaryExpr) -> Result<Value, RError> {
    let right = evaluate(*expression.right)?;
    match expression.oper.token_type {
        TokenType::Minus => match right {
            Value::Number(num) => Ok(Value::Number(-num)),
            _ => Err(RError::new(String::from("Operand should be a number."))),
        },
        TokenType::Bang => match right {
            Value::Bool(value) => Ok(Value::Bool(!value)),
            _ => Ok(Value::Bool(false)),
        },
        _ => Err(RError::new(String::from("Unknown unary operation"))),
    }
}

fn boolean(expression: bool) -> Result<Value, RError> {
    Ok(Value::Bool(expression))
}

fn string(expression: String) -> Result<Value, RError> {
    Ok(Value::String(expression))
}

fn number(expression: NumberExpr) -> Result<Value, RError> {
    Ok(Value::Number(expression.value))
}

fn binary(expression: BinaryExpr) -> Result<Value, RError> {
    let left = evaluate(*expression.left)?;
    let right = evaluate(*expression.right)?;

    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                TokenType::Plus => Ok(Value::Number(l + r)),
                TokenType::Minus => Ok(Value::Number(l - r)),
                TokenType::Slash => Ok(Value::Number(l / r)),
                TokenType::Star => Ok(Value::Number(l * r)),
                TokenType::Greater => Ok(Value::Bool(l > r)),
                TokenType::GreaterEqual => Ok(Value::Bool(l >= r)),
                TokenType::Less => Ok(Value::Bool(l < r)),
                TokenType::LessEqual => Ok(Value::Bool(l <= r)),
                _ => Err(RError::new(String::from("Unknown operation."))),
            }
        },
        (Value::Bool(l), Value::Bool(r)) => {
            match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => Err(RError::new(String::from("Unknown operation."))),
            }
        },
        (Value::String(l), Value::String(r)) => {
            match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l.eq(&r))),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => Err(RError::new(String::from("Unknown operation."))),
            }
        },
        _ => {
            match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(false)),
                TokenType::BangEqual => Ok(Value::Bool(true)),
                _ => {
                    let message = "Operands should be of same type.".to_string();
                    Err(RError::new(message))
                }
            }
        }
    }
}
