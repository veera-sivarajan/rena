use crate::expr::{Expr, BinaryExpr, NumberExpr, UnaryExpr, VariableExpr,
                  GroupExpr};
use crate::token::TokenType;
use crate::err::LoxError;
use crate::stmt::{Stmt, PrintStmt, ExpressionStmt, VarStmt};
use crate::environment::Environment;
use crate::token::Token;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String), 
}


pub struct Interpreter {
    memory: Environment, // 
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { memory: Environment::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?
        }
        Ok(())
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Print(stmt) => Ok(self.print(stmt)?),
            Stmt::Expression(stmt) => Ok(self.expression(stmt)?),
            Stmt::Var(stmt) => Ok(self.var(stmt)?),
        }
    }

    fn stringify(&self, result: Value) -> String {
        match result {
            Value::Number(num) => format!("{}", num),
            Value::Bool(tof) => format!("{}", tof),
            Value::String(value) => format!("{}", value),
        }
    }

    fn print(&self, stmt: PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(*stmt.expr)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn expression(&self, stmt: ExpressionStmt) -> Result<(), LoxError> {
        let _value = self.evaluate(*stmt.expr)?;
        Ok(())
    }

    fn var(&mut self, decl: VarStmt) -> Result<(), LoxError> {
        if let Some(init) = decl.init {
            let value = self.evaluate(*init)?;
            self.memory.define(decl.name.lexeme, Some(value));
            Ok(())
        } else {
            self.memory.define(decl.name.lexeme, None);
            Ok(())
        }
    }

    fn evaluate(&self, expression: Expr) -> Result<Value, LoxError> {
        match expression {
            Expr::Number(expr) => Ok(Value::Number(expr.value)),
            Expr::String(expr) => Ok(Value::String(expr)),
            Expr::Boolean(expr) => Ok(Value::Bool(expr)),
            Expr::Unary(expr) => self.unary(expr),
            Expr::Binary(expr) => self.binary(expr),
            Expr::Variable(expr) => self.variable(expr),
            Expr::Group(expr) => self.group(expr),
            Expr::Assign(expr) => unreachable!(),
        }
    }

    fn group(&self, expression: GroupExpr) -> Result<Value, LoxError> {
        self.evaluate(*expression.expr)
    }

    fn unary(&self, expression: UnaryExpr) -> Result<Value, LoxError> {
        let right = self.evaluate(*expression.right)?;
        match expression.oper.token_type {
            TokenType::Minus => match right {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => Err(LoxError::new(String::from("Operand not a number.")))
            },
            TokenType::Bang => match right {
                Value::Bool(value) => Ok(Value::Bool(!value)),
                _ => Ok(Value::Bool(false)),
            },
            _ => Err(LoxError::new(String::from("Unknown unary operation."))),
        }
    }

    fn variable(&self, expression: VariableExpr) -> Result<Value, LoxError> {
        self.look_up(expression.name)
    }

    fn look_up(&self, name: Token) -> Result<Value, LoxError> {
        match self.memory.fetch(name.lexeme) {
            None => Err(LoxError::new(String::from("Undeclared variable."))),
            Some(variable) => match variable {
                Some(value) => Ok(value.clone()),
                None => {
                    Err(LoxError::new("Uninitialized variable.".to_string()))
                }
            }
        }
    }

    fn binary(&self, expression: BinaryExpr) -> Result<Value, LoxError> {
        let left = self.evaluate(*expression.left)?;
        let right = self.evaluate(*expression.right)?;

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
                    _ => Err(LoxError::new(String::from("Unknown operation."))),
                }
            }, 
            (Value::Bool(l), Value::Bool(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                    TokenType::BangEqual => Ok(Value::Bool(l != r)),
                    _ => Err(LoxError::new(String::from("Unknown operation."))),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => Ok(Value::Bool(l.eq(&r))),
                    TokenType::BangEqual => Ok(Value::Bool(l.ne(&r))),
                    _ => Err(LoxError::new(String::from("Unknown operation."))),
                }
            }
            _ => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => Ok(Value::Bool(false)),
                    TokenType::BangEqual => Ok(Value::Bool(true)),
                    _ => {
                        let message = "Operand should be of same type.";
                        Err(LoxError::new(message.to_string()))
                    }
                }
            }
        }
    }
}
