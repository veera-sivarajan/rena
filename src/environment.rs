use crate::err::LoxError;
use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    frame_list: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            frame_list: vec![HashMap::new()],
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Environment {
        let mut new_env = enclosing.frame_list.clone();
        new_env.push(HashMap::new());
        Environment {
            frame_list: new_env, 
        }
    }

    pub fn new_block(&mut self) {
        self.frame_list.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.frame_list.pop();
    }

    pub fn define(&mut self, name: String, value: Value) -> Result<(), LoxError> {
        if let Some(frame) = self.frame_list.last_mut() {
            frame.insert(name, value);
            Ok(())
        } else {
            error!("Frame not available.")
        }
    }

    pub fn fetch(&self, name: String) -> Option<&Value> {
        for frame in self.frame_list.iter().rev() {
            if frame.contains_key(&name) {
                return frame.get(&name);
            }
        }
        None
    }

    pub fn assign(&mut self,
                  name: String,
                  value: Value
    ) -> Result<Value, LoxError> {
        // start searching from the innermost scope
        for frame in self.frame_list.iter_mut().rev() {
            if frame.contains_key(&name) {
                frame.insert(name, value.clone());
                return Ok(value);
            }
        }
        error!("Undefined variable.")
    }
}
