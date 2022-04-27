use crate::interpreter::{Interpreter, Value};
use crate::stmt::FunStmt;
use crate::err::LoxError;

pub trait Callable {
    fn arity(&self) -> usize; // maximum number of arguments is 255
    fn call(&self,
            intp: &mut Interpreter,
            args: Vec<Value>
    ) -> Result<Value, LoxError>;
}

#[derive(Clone, Debug)]
pub struct Function {
    pub declaration: FunStmt
}

impl Function {
    pub fn new(declaration: FunStmt) -> Function {
        Function { declaration }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(&self,
            intp: &mut Interpreter,
            args: Vec<Value>
    ) -> Result<Value, LoxError> {
        if args.len() != self.arity() {
            error!(format!("Expected {} arguments but got {}.",
                           self.arity(), args.len()))
        } else {
            // create a new frame
            intp.memory.new_block();

            // bind all argument values to function parameters in the new frame
            self.declaration.params 
                .iter()
                .zip(args.iter()) // combines two iters into one tuple
                .try_for_each(|(name, value)| {
                    intp.memory.define(&name.lexeme, value.clone())
                })?;

            // interpret function statements in the context of newly created frame
            let result = intp.interpret(&self.declaration.body);
            // remove new frame after interpreting body of function 
            intp.memory.exit_block();
            // result could be a return value or an error 
            match result {
                Err(LoxError::Return(value)) => Ok(value),
                Err(LoxError::Error(msg)) => error!(msg), 
                Ok(()) => Ok(Value::Nil),
            }
        }
    }
}
            

    
        
