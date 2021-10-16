use crate::err::RError;
use crate::expr::{BinaryExpr, Expr, UnaryExpr};
use crate::stmt::{Stmt, VarStmt, PrintStmt, ExpressionStmt};
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
        Value::String(value) => value,
    }
}

pub fn interpret(statements: Vec<Stmt>) {
    for statement in statements {
        match execute(statement) {
            Ok(()) => {},
            Err(runtime_error) => println!("{}", runtime_error.to_string()),
        }
    }
}

fn execute(statement: Stmt) -> Result<(), RError> {
    match statement {
        Stmt::Print(stmt) => Ok(print(stmt)?),
        _ => Ok(()),
    }
}

// fn _var(decl: Stmt) {
//     if let Some(init) = decl.init {
//         let value = evaluate(decl.init)?;
//         // TODO: store name and value in environment (hash map)
//     } else {
//         // TODO: Store name and none in environment
//     }
// }

fn print(stmt: PrintStmt) -> Result<(), RError> { 
    let value = evaluate(*stmt.expr)?;
    println!("{}", stringify(value));
    Ok(())
}

fn evaluate(expression: Expr) -> Result<Value, RError> {
    match expression {
        Expr::Number(expr) => Ok(Value::Number(expr.value)),
        Expr::String(expr) => Ok(Value::String(expr)),
        Expr::Boolean(expr) => Ok(Value::Bool(expr)),
        Expr::Unary(expr) => unary(expr),
        Expr::Binary(expr) => binary(expr),
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

fn division(left: f64, right: f64) -> Result<Value, RError> {
    if right == 0.0 {
        Err(RError::new(String::from("Division by zero not allowed")))
    } else {
        Ok(Value::Number(left / right))
    }
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
                TokenType::Slash => division(l, r),
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
