use crate::environment::Environment;
use crate::err::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GroupExpr, UnaryExpr,
    VariableExpr,
};
use crate::functions::{Callable, Function};
use crate::stmt::{
    BlockStmt, ExpressionStmt, FunStmt, IfStmt, PrintStmt, ReturnStmt,
    Stmt, VarStmt, WhileStmt,
};
use crate::token::{Token, TokenType};

use float_eq::{float_eq, float_ne};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    String(String),
    Function(Function),
}

pub struct Interpreter {
    pub memory: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: Environment::new(),
        }
    }

    pub fn interpret(
        &mut self,
        statements: &[Stmt],
    ) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Print(stmt) => self.print(stmt),
            Stmt::Expression(stmt) => self.expression(stmt),
            Stmt::Var(stmt) => self.var(stmt),
            Stmt::Block(stmt) => {
                self.block(&stmt.statements,
                           Environment::with_enclosing(self.memory.frame_list.clone()))
            }
            Stmt::If(stmt) => self.execute_if(stmt),
            Stmt::While(stmt) => self.execute_while(stmt),
            Stmt::Function(stmt) => self.fun_decl(stmt),
            Stmt::Return(stmt) => self.execute_return(stmt),
        }
    }

    fn execute_return(
        &mut self,
        stmt: &ReturnStmt,
    ) -> Result<(), LoxError> {
        if let Some(ref v) = stmt.value {
            let value = self.evaluate(v)?;
            Err(LoxError::Return(value))
        } else {
            // return keyword followd by no expression
            // e.g `return;`
            Err(LoxError::Return(Value::Nil))
        }
    }

    fn fun_decl(&mut self, statement: &FunStmt) -> Result<(), LoxError> {
        let func =
            Function::new(statement.to_owned(), self.memory.clone());
        self.memory
            .define(&statement.name.lexeme, Value::Function(func))
    }

    fn is_truthy(&self, object: Value) -> bool {
        match object {
            Value::Nil => false,
            Value::Bool(value) => value,
            _ => true,
        }
    }

    fn execute_while(
        &mut self,
        statement: &WhileStmt,
    ) -> Result<(), LoxError> {
        let mut value = self.evaluate(&statement.condition)?;
        while self.is_truthy(value) {
            self.execute(&statement.body)?;
            value = self.evaluate(&statement.condition)?;
        }
        Ok(())
    }

    fn execute_if(&mut self, statement: &IfStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&statement.condition)?;
        if self.is_truthy(value) {
            self.execute(&statement.then_branch)
        } else if let Some(else_branch) = &statement.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn print(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expr)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn stringify(&self, result: Value) -> String {
        match result {
            Value::Number(num) => format!("{num}"),
            Value::Bool(tof) => format!("{tof}"),
            Value::String(value) => value,
            Value::Nil => "nil".to_string(),
            Value::Function(fun) => {
                format!("<fn {}>", fun.declaration.name.lexeme)
            }
        }
    }

    fn expression(
        &mut self,
        stmt: &ExpressionStmt,
    ) -> Result<(), LoxError> {
        let _ = self.evaluate(&stmt.expr)?;
        Ok(())
    }

    fn var(&mut self, decl: &VarStmt) -> Result<(), LoxError> {
        if let Some(init) = &decl.init {
            let value = self.evaluate(init)?;
            self.memory.define(&decl.name.lexeme, value)
        } else {
            self.memory.define(&decl.name.lexeme, Value::Nil)
        }
    }

    pub fn block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), LoxError> {
        let previous = env.frame_list.clone();
        self.memory.frame_list = env.frame_list;
        for stmt in statements {
            self.execute(stmt).map_err(|err| {
                self.memory.frame_list = previous.clone();
                err
            })?;
        }
        self.memory.frame_list = previous;
        Ok(())
    }

    fn variable(
        &self,
        expression: &VariableExpr,
    ) -> Result<Value, LoxError> {
        self.look_up(expression.name.clone())
    }

    fn look_up(&self, name: Token) -> Result<Value, LoxError> {
        match self.memory.fetch(&name.lexeme) {
            None => {
                let msg = format!("Undeclared variable '{}'", name.lexeme);
                error!(msg.as_str())
            }
            Some(value) => match value {
                Value::Nil => error!("Uninitialized variable."),
                _ => Ok(value.clone()),
            },
        }
    }

    fn unary(
        &mut self,
        expression: &UnaryExpr,
    ) -> Result<Value, LoxError> {
        let right = self.evaluate(&expression.right)?;
        match expression.oper.token_type {
            TokenType::Minus => match right {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => error!("Operand not a number."),
            },
            TokenType::Bang => match right {
                Value::Bool(value) => Ok(Value::Bool(!value)),
                _ => Ok(Value::Bool(false)),
            },
            _ => error!("Unknown unary operation."),
        }
    }

    fn division(&self, numer: f64, denom: f64) -> Result<Value, LoxError> {
        if denom == 0.0 {
            error!("Division by zero not allowed.")
        } else {
            Ok(Value::Number(numer / denom))
        }
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Value, LoxError> {
        match expression {
            Expr::Nil => Ok(Value::Nil),
            Expr::Number(expr) => Ok(Value::Number(expr.value)),
            Expr::String(expr) => Ok(Value::String(expr.to_string())),
            Expr::Boolean(expr) => Ok(Value::Bool(*expr)),
            Expr::Unary(expr) => self.unary(expr),
            Expr::Binary(expr) => self.binary(expr),
            Expr::Variable(expr) => self.variable(expr),
            Expr::Group(expr) => self.group(expr),
            Expr::Assign(expr) => self.assignment(expr),
            Expr::Call(expr) => self.call(expr),
        }
    }

    fn call(&mut self, expr: &CallExpr) -> Result<Value, LoxError> {
        let fun_name = self.evaluate(&expr.callee)?;
        let args = expr
            .args
            .iter() // iterate over the values by reference
            .map(|arg| self.evaluate(arg))
            .collect::<Result<Vec<_>, _>>()?;
        if let Value::Function(func) = fun_name {
            func.call(self, args)
        } else {
            error!("Can only call functions and classes.")
        }
    }

    fn assignment(
        &mut self,
        expression: &AssignExpr,
    ) -> Result<Value, LoxError> {
        let value = self.evaluate(&expression.value)?;
        self.memory.assign(&expression.name.lexeme, value)
    }

    fn group(
        &mut self,
        expression: &GroupExpr,
    ) -> Result<Value, LoxError> {
        self.evaluate(&expression.expr)
    }

    fn binary(
        &mut self,
        expression: &BinaryExpr,
    ) -> Result<Value, LoxError> {
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => {
                        Ok(Value::Bool(float_eq!(l, r, ulps <= 10)))
                    }
                    TokenType::BangEqual => {
                        Ok(Value::Bool(float_ne!(l, r, ulps <= 10)))
                    }
                    TokenType::Plus => Ok(Value::Number(l + r)),
                    TokenType::Minus => Ok(Value::Number(l - r)),
                    TokenType::Slash => self.division(l, r),
                    TokenType::Star => Ok(Value::Number(l * r)),
                    TokenType::Greater => Ok(Value::Bool(l > r)),
                    TokenType::GreaterEqual => Ok(Value::Bool(l >= r)),
                    TokenType::Less => Ok(Value::Bool(l < r)),
                    TokenType::LessEqual => Ok(Value::Bool(l <= r)),
                    _ => error!("Unknown operation for numbers."),
                }
            }
            (Value::Bool(l), Value::Bool(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                    TokenType::BangEqual => Ok(Value::Bool(l != r)),
                    _ => error!("Unknown operation for bools."),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match expression.oper.token_type {
                    TokenType::EqualEqual => Ok(Value::Bool(l.eq(&r))),
                    TokenType::BangEqual => Ok(Value::Bool(l.ne(&r))),
                    TokenType::Plus => {
                        Ok(Value::String(format!("{}{}", l, r)))
                    }
                    _ => error!("Unknown operation for strings."),
                }
            }
            _ => match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(false)),
                TokenType::BangEqual => Ok(Value::Bool(true)),
                _ => error!("Operands should be of same type."),
            },
        }
    }
}
