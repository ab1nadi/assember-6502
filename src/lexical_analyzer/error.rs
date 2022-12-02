use std::error::Error;
use std::fmt;

#[macro_export]
macro_rules! unwrap_result_or_return_err {
    ( $e:expr, $s:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) =>  return Err(LexicalError::new($s)),
        }
    }
}

// LexicalError
// the error that gets
// returned when something goes
// wrong
#[derive(Debug)]
#[derive(Clone)]
pub struct LexicalError
{
    pub details: String
}

impl LexicalError {
    pub fn new(msg: &str) -> LexicalError {
        LexicalError{details: msg.to_string()}
    }
}

impl Error for LexicalError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


