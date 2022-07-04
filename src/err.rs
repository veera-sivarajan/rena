use crate::interpreter::Value;
use std::error::Error;
use std::fmt;

macro_rules! error {
    ( $message:expr) => {
        Err(LoxError::Error(String::from($message)))
    };
}

#[derive(Debug)]
pub enum LoxError {
    Error(String),
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Error(msg) => write!(f, "Error: {}", msg),
            LoxError::Return(_) => write!(f, "Unreachable code!"),
        }
    }
}
