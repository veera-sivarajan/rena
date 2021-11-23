use crate::interpreter::Value;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub fn fetch(&self, name: String) -> Option<&Option<Value>> {
        self.values.get(&name)
    }
<<<<<<< HEAD
=======

    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn assign(&mut self, name: String, value: Value) {
        self.values.insert(name, Some(value));
    }
            
        
        
>>>>>>> stream
}
