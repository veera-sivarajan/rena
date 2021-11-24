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

    pub fn define(&mut self, name: String,
                  value: Value) -> Result<(), LoxError> {
        if let Some(frame) = self.frame_list.get_mut(0) {
            frame.insert(name, value);
            Ok(())
        } else {
            Err(LoxError::new(String::from("Frame not available.")))
        }
    }

    fn fetch_helper(&self, name: String,
                    frame_count: usize) -> Option<&Value> {
        if let Some(frame) = self.frame_list.get(frame_count) {
            if frame.contains_key(&name) {
                frame.get(&name)
            } else {
                println!("Element not found in frame.");
                self.fetch_helper(name, frame_count + 1)
            }
        } else {
            None
        }
    }

    pub fn fetch(&self, name: String) -> Option<&Value> {
        self.fetch_helper(name, 0)
    }

    fn assign_helper(&mut self, name: String,
                     value: Value,
                     frame_count: usize) -> Result<Value, LoxError> {
        if let Some(frame) = self.frame_list.get_mut(frame_count) {
            if frame.contains_key(&name) {
                frame.insert(name, value.clone()).unwrap();
                Ok(value)
            } else {
                self.assign_helper(name, value, frame_count + 1)
            }
        } else {
            Err(LoxError::new(String::from("Undefined variable.")))
        }
    }

    pub fn assign(&mut self, name: String,
                  value: Value) -> Result<Value, LoxError> {
        self.assign_helper(name, value, 0)
    }
}
