use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LoxError {
    err: String,
}

impl Error for LoxError {
    fn description(&self) -> &str {
        &self.err
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.err)
    }
}

impl LoxError {
    pub fn new(err: String) -> LoxError {
        LoxError{ err }
    }
}

#[derive(Debug)]
pub struct RError {
    err: String,
}

impl RError {
    pub fn new(err: String) -> RError {
        RError{ err }
    }
}

impl Error for RError {
    fn description(&self) -> &str {
        &self.err
    }
}
    
impl fmt::Display for RError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", &self.err)
    }
}
