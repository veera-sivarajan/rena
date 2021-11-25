use crate::err::LoxError;
use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    frame_list: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut frames = Vec::new();
        frames.push(HashMap::new());
        
        Environment {
            frame_list: frames,
        }
    }

    pub fn new_block(&mut self) {
        self.frame_list.insert(0, HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.frame_list.remove(0);
    }

    pub fn define(&mut self,
                  name: String, value: Value) -> Result<(), LoxError> {
        if let Some(frame) = self.frame_list.get_mut(0) {
            frame.insert(name, value);
            Ok(())
        } else {
            error!("Frame not available.")
        }
    }

    pub fn fetch(&self, name: String) -> Option<&Value> {
        for frame in &self.frame_list {
            if frame.contains_key(&name) {
                return frame.get(&name);
            }
        }
        None
    }

    pub fn assign(&mut self, name: String,
                  value: Value) -> Result<Value, LoxError> {
        for frame in &mut self.frame_list {
            if frame.contains_key(&name) {
                frame.insert(name, value.clone());
                return Ok(value);
            }
        }
        error!("Undefined variable.")
    }
        
}
