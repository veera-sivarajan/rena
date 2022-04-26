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

#[derive(Clone)]
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
            error!("Number of arguments does not match number of parameters.")
        } else {
            // create a new frame
            intp.memory.new_block();

            // bind all argument values to function parameters in the new frame
            self.declaration.params 
                .iter()
                .zip(args.iter())
                .try_for_each(|(name, value)| {
                    intp.memory.define(name.lexeme.clone(), value.clone())
                })?;

            // interpret function statements in the context of newly created frame
            intp.interpret(&self.declaration.body)?; 

            // after executing all function statements, remove the new frame
            intp.memory.exit_block();
            Ok(Value::Nil)
        }
    }
}
            

    
        

