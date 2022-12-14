use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    IOError(String),
    ParseError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(reason) => write!(f, "IO error: {reason}"),
            Self::ParseError => write!(f, "Parse error: Couldn't parse input"),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseError
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}
