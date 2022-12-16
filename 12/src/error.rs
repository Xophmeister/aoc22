use std::fmt;
use std::io;

use crate::map::Coord;

#[derive(Debug)]
pub enum Error {
    IOError(String),
    ParseError,
    NoRoute(Coord, Coord),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(reason) => write!(f, "IO error: {reason}"),
            Self::ParseError => write!(f, "Parse error: Couldn't parse input"),
            Self::NoRoute(start, finish) => write!(f, "No route found from {start} to {finish}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}
