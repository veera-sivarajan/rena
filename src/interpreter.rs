use crate::environment::Environment;
use crate::err::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, Expr, GroupExpr, UnaryExpr, VariableExpr
};
use crate::stmt::{
    BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt
};
use crate::token::{Token, TokenType};

use float_eq::{float_eq, float_ne};

#[derive(Clone)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    String(String),
}

pub struct Interpreter {
    memory: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Print(stmt) => self.print(stmt),
            Stmt::Expression(stmt) => self.expression(stmt),
            Stmt::Var(stmt) => self.var(stmt),
            Stmt::Block(stmt) => self.block(stmt),
            Stmt::If(stmt) => self.execute_if(stmt),
            Stmt::While(stmt) => self.execute_while(stmt),
            Stmt::Function(stmt) => error!("Fun declaration not implemented."),
        }
    }

    fn is_truthy(&self, object: Value) -> bool {
        match object {
            Value::Nil => false,
            Value::Bool(value) => value,
            _ => true,
        }
    }

    fn execute_while(&mut self, statement: &WhileStmt) -> Result<(), LoxError> {
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
            self.execute(&statement.then_branch)?
        } else if let Some(else_branch) = &statement.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn print(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expr)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn stringify(&self, result: Value) -> String {
        match result {
            Value::Number(num) => format!("{}", num),
            Value::Bool(tof) => format!("{}", tof),
            Value::String(value) => value,
            Value::Nil => "nil".to_string(),
        }
    }

    fn expression(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        let _value = self.evaluate(&stmt.expr)?;
        Ok(())
    }

    fn var(&mut self, decl: &VarStmt) -> Result<(), LoxError> {
        if let Some(init) = &decl.init {
            let value = self.evaluate(init)?;
            self.memory.define(decl.name.lexeme.clone(), value)
        } else {
            self.memory.define(decl.name.lexeme.clone(), Value::Nil)
        }
    }

    fn block(&mut self, block: &BlockStmt) -> Result<(), LoxError> {
        self.memory.new_block();

        for stmt in &block.statements {
            match self.execute(stmt) {
                Ok(()) => continue,
                Err(error) => {
                    self.memory.exit_block();
                    return Err(error);
                }
            }
        }
        self.memory.exit_block();
        Ok(())
    }

    fn variable(&self, expression: &VariableExpr) -> Result<Value, LoxError> {
        self.look_up(expression.name.clone())
    }

    fn look_up(&self, name: Token) -> Result<Value, LoxError> {
        match self.memory.fetch(name.lexeme) {
            None => error!("Undeclared variable."),
            Some(value) => match value {
                Value::Nil => error!("Uninitialized variable."),
                _ => Ok(value.clone()),
            },
        }
    }

    fn unary(&mut self, expression: &UnaryExpr) -> Result<Value, LoxError> {
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
            Expr::Call(_expr) => error!("Call expression not implemented!"),
        }
    }

    // fn call(&mut self, expr: &CallExpr) -> Result<Value, LoxError> {
    //     let fun_name = self.evaluate(&expr.callee)?;
    //     let args: Vec<Value> = vec![];
    //     for arg in &expr.args {
    //         args.push(self.evaluate(arg)?);
    //     }
    //     // TODO
    //     // if let fun_name != Value::Function {
    //     //     error!("Can only call functions and classes.");
    //     // } else {
    // }

    fn assignment(&mut self,
                  expression: &AssignExpr
    ) -> Result<Value, LoxError> {
        let value = self.evaluate(&expression.value)?;
        self.memory.assign(expression.name.lexeme.clone(), value)
    }

    fn group(&mut self, expression: &GroupExpr) -> Result<Value, LoxError> {
        self.evaluate(&expression.expr)
    }

    fn binary(&mut self, expression: &BinaryExpr) -> Result<Value, LoxError> {
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(float_eq!(l, r, ulps <= 10))),
                TokenType::BangEqual => Ok(Value::Bool(float_ne!(l, r, ulps <= 10))),
                TokenType::Plus => Ok(Value::Number(l + r)),
                TokenType::Minus => Ok(Value::Number(l - r)),
                TokenType::Slash => self.division(l, r),
                TokenType::Star => Ok(Value::Number(l * r)),
                TokenType::Greater => Ok(Value::Bool(l > r)),
                TokenType::GreaterEqual => Ok(Value::Bool(l >= r)),
                TokenType::Less => Ok(Value::Bool(l < r)),
                TokenType::LessEqual => Ok(Value::Bool(l <= r)),
                _ => error!("Unknown operation for numbers."),
            },
            (Value::Bool(l), Value::Bool(r)) => match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => error!("Unknown operation for bools."),
            },
            (Value::String(l), Value::String(r)) => match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l.eq(&r))),
                TokenType::BangEqual => Ok(Value::Bool(l.ne(&r))),
                _ => error!("Unknown operation for strings."),
            },
            _ => match expression.oper.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(false)),
                TokenType::BangEqual => Ok(Value::Bool(true)),
                _ => error!("Operands should be of same type."),
            },
        }
    }
}
