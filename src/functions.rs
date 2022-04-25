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
        intp.memory.new_block();
        let mut args_iter = args.iter();
        for param in &self.declaration.params {
            intp.memory.define(param.lexeme.clone(),
                               args_iter.next().unwrap().clone()).unwrap();
        }
        let _result = intp.execute_stmts(&self.declaration.body);
        intp.memory.exit_block();
        Ok(Value::Nil)
    }
}
            

    
        

