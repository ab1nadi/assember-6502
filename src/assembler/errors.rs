use std::error::Error;
use std::fmt;


// LexicalError
// the error that gets
// returned when something goes
// wrong
#[derive(Debug)]
pub struct AssemblerError
{
    pub details: String
}

impl AssemblerError {
    pub fn new(msg: &str) -> AssemblerError {
        AssemblerError{details: msg.to_string()}
    }
}

impl Error for AssemblerError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}