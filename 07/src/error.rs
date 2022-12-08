use std::fmt;
use std::io;
use std::num::ParseIntError;

pub struct ParseError;

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't parse input")
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't parse input")
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_error: ParseIntError) -> Self {
        ParseError
    }
}

impl From<io::Error> for ParseError {
    fn from(_error: io::Error) -> Self {
        ParseError
    }
}
