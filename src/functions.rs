use crate::environment::Environment;
use crate::err::LoxError;
use crate::interpreter::{Interpreter, Value};
use crate::stmt::FunStmt;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Callable {
    fn arity(&self) -> usize; // maximum number of arguments is 255
    fn call(
        &self,
        intp: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, LoxError>;
}

#[derive(Clone, Debug)]
pub struct Function {
    pub declaration: FunStmt,
    closure: Rc<RefCell<Environment>>, // surrounding environment
}

impl Function {
    pub fn new(
        declaration: FunStmt,
        closure: Rc<RefCell<Environment>>,
    ) -> Function {
        Function {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        intp: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, LoxError> {
        if args.len() != self.arity() {
            error!(format!(
                "Expected {} arguments but got {}.",
                self.arity(),
                args.len()
            ))
        } else {
            let mut env =
                Environment::with_enclosing(self.closure.clone());
            self.declaration
                .params
                .iter()
                .zip(args.iter()) // combines two iters into one tuple
                .for_each(|(name, value)| {
                    env.define(&name.lexeme, value.clone())
                });

            let result = intp
                .block(&self.declaration.body, Rc::new(RefCell::new(env)));
            match result {
                Err(LoxError::Return(value)) => Ok(value),
                Err(LoxError::Error(msg)) => error!(msg),
                Ok(()) => Ok(Value::Nil),
            }
        }
    }
}
