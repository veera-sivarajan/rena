use crate::err::LoxError;
use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Environment {
    pub frame_list: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            frame_list: vec![HashMap::new()],
        }
    }

    pub fn new_block(&mut self) {
        self.frame_list.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.frame_list.pop();
    }

    pub fn define(&mut self, name: &str, value: Value) -> Result<(), LoxError> {
        if let Some(frame) = self.frame_list.last_mut() {
            frame.insert(name.to_owned(), value);
            Ok(())
        } else {
            error!("Frame not available.")
        }
    }

    pub fn fetch(&self, name: &str) -> Option<&Value> {
        let frame = self.frame_list 
            .iter()
            .rev() // traverse from inner scope
            .find(|f| f.contains_key(name));
        
        if let Some(f) = frame {
            f.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self,
                  name: &str,
                  value: Value
    ) -> Result<Value, LoxError> {
        let frame = self.frame_list
            .iter_mut()
            .rev()
            .find(|f| f.contains_key(name));

        if let Some(f) = frame {
            f.insert(name.to_owned(), value.clone());
            Ok(value)
        } else {
            error!("Undefined variable.")
        }
    }
}
