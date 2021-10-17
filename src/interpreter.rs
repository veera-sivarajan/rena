use crate::err::RError;
use crate::token::Token;
use crate::expr::{BinaryExpr, Expr, UnaryExpr, VariableExpr};
use crate::stmt::{Stmt, VarStmt, PrintStmt, ExpressionStmt};
use crate::token::TokenType;
use crate::environment::Environment;

use float_eq::{float_eq, float_ne};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
}

pub struct Interpreter {
    memory: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { memory: Environment::new() }
    }

    fn stringify(&self, result: Value) -> String {
        match result {
            Value::Number(num) => format!("{}", num),
            Value::Bool(tof) => format!("{}", tof),
            Value::String(value) => value,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            match self.execute(statement) {
                Ok(()) => {},
                Err(runtime_error) => println!("{}", runtime_error.to_string()),
            }
        }
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), RError> {
        match statement {
            Stmt::Print(stmt) => Ok(self.print(stmt)?),
            Stmt::Var(stmt) => Ok(self.var(stmt)?), 
            Stmt::Expression(stmt) => Ok(self.expression(stmt)?),
        }
    }

    fn var(&mut self, decl: VarStmt) -> Result<(), RError> {
        if let Some(init) = decl.init {
            let value = self.evaluate(*init)?;
            self.memory.define(decl.name.lexeme, Some(value));
            Ok(())
        } else {
            self.memory.define(decl.name.lexeme, None);
            Ok(())
        }
    }

    fn print(&self, stmt: PrintStmt) -> Result<(), RError> { 
        let value = self.evaluate(*stmt.expr)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn expression(&self, stmt: ExpressionStmt) -> Result<(), RError> {
        let _value = self.evaluate(*stmt.expr)?;
        Ok(())
    }

    fn evaluate(&self, expression: Expr) -> Result<Value, RError> {
        match expression {
            Expr::Number(expr) => Ok(Value::Number(expr.value)),
            Expr::String(expr) => Ok(Value::String(expr)),
            Expr::Boolean(expr) => Ok(Value::Bool(expr)),
            Expr::Unary(expr) => self.unary(expr),
            Expr::Binary(expr) => self.binary(expr),
            Expr::Variable(expr) => self.variable(expr),
        }
    }

    fn variable(&self, expression: VariableExpr) -> Result<Value, RError> {
        self.look_up(expression.name)
    }

    fn look_up(&self, name: Token) -> Result<Value, RError> {
        match self.memory.fetch(name.lexeme) {
            None => Err(RError::new(String::from("Undeclared variable."))),
            Some(variable) => match variable {
                None => Err(RError::new("Uninitialized variable.".to_string())),
                Some(value) => Ok(value.clone()),
            }
        }
    }

    fn unary(&self, expression: UnaryExpr) -> Result<Value, RError> {
        let right = self.evaluate(*expression.right)?;
        match expression.oper.token_type {
            TokenType::Minus => match right {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => Err(RError::new(String::from("Operand not a number."))),
            },
            TokenType::Bang => match right {
                Value::Bool(value) => Ok(Value::Bool(!value)),
                _ => Ok(Value::Bool(false)),
            },
            _ => Err(RError::new(String::from("Unknown unary operation"))),
        }
    }

    fn division(&self, left: f64, right: f64) -> Result<Value, RError> {
        if right == 0.0 {
            Err(RError::new(String::from("Division by zero not allowed")))
        } else {
            Ok(Value::Number(left / right))
        }
    }

    fn binary(&self, expression: BinaryExpr) -> Result<Value, RError> {
        let left = self.evaluate(*expression.left)?;
        let right = self.evaluate(*expression.right)?;

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => {
                        let result = float_eq!(l, r, ulps <= 10); 
                        Ok(Value::Bool(result))
                    }
                    TokenType::BangEqual => {
                        let result = float_ne!(l, r, ulps <= 10); 
                        Ok(Value::Bool(result))
                    }
                    TokenType::Plus => Ok(Value::Number(l + r)),
                    TokenType::Minus => Ok(Value::Number(l - r)),
                    TokenType::Slash => self.division(l, r),
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
                        let message = "Operands should be of same type.";
                        Err(RError::new(message.to_string()))
                    }
                }
            }
        }
    }
}
