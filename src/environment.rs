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
            frame_list: Rc::new(RefCell::new(vec![HashMap::new()]))
        }
    }

    pub fn with_enclosing(frame_list: Rc<RefCell<Vec<HashMap<String, Value>>>>) -> Environment {
        // let mut frames = vec![];
        // frames.append(&mut environment.borrow_mut().frame_list);
        // frames.push(HashMap::new());
        frame_list.borrow_mut().push(HashMap::new());
        Environment {
            frame_list: frame_list.clone()
        }
    }

    // pub fn new_block(&mut self) {
    //     self.frame_list.push(HashMap::new());
    // }

    // pub fn exit_block(&mut self) {
    //     self.frame_list.pop();
    // }

    pub fn define(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), LoxError> {
        if let Some(frame) = self.frame_list.borrow_mut().last_mut() {
            frame.insert(name.to_owned(), value);
            Ok(())
        } else {
            let msg = format!("Frame not available to define '{name}'");
            error!(msg.as_str())
        }
    }

    pub fn fetch(&self, name: &str) -> &Option<&Value> {
            
        let frame = self
            .frame_list
            .borrow()
            .iter()
            .rev() // traverse from inner scope
            .find(|f| f.contains_key(name));
        
        if let Some(f) = frame {
            &f.get(name)
        } else {
            &None
        }
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<Value, LoxError> {
        let mut binding= self
            .frame_list
            .borrow_mut();
            
        let frame = binding
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
