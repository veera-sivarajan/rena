use crate::err::LoxError;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    closure: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            closure: None,
            values: HashMap::new()
        }
    }

    pub fn with_enclosing(closure: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            closure: Some(closure),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<Value, LoxError> {
        if self.values.contains_key(name) {
            Ok(self.values.insert(name.to_string(), value).unwrap())
        } else {
            match &self.closure {
                Some(env) => {
                    env.borrow_mut().assign(name, value)
                }
                None => {
                    error!(format!("Cannot assign to undeclared variable `{}`", name))
                }
            }
        }
    }

    pub fn fetch(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(val) => Some(val.clone()),
            None => match &self.closure {
                Some(env) => env.borrow().fetch(name),
                None => None,
            }
        }
    }
}

