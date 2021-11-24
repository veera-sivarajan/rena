use crate::interpreter::Value;
use std::collections::HashMap;
use crate::err::LoxError;

pub struct Environment {
    values: HashMap<String, Option<Value>>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Environment {
        Environment { values: HashMap::new(),
                      enclosing: enclosing.map(Box::new), 
        }
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub fn fetch(&self, name: String) -> &Option<Value> {
        if let Some(value) = self.values.get(&name) {
            value
        } else if self.enclosing.is_some() {
            self.enclosing
                .as_ref()
                .unwrap()
                .fetch(name)
        } else {
            &None
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn assign(&mut self, name: String,
                   value: Value) -> Result<Value, LoxError> {
        if self.contains(&name) {
            self.values.insert(name, Some(value.clone()));
            Ok(value)
        } else if self.enclosing.is_some() {
            self.enclosing
                .as_mut()
                .unwrap()
                .assign(name, value)
        } else {
            Err(LoxError::new(String::from("Undefined variable.")))
        }
    }

            
}
