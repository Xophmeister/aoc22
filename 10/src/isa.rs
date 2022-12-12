use std::io::stdin;
use std::str::FromStr;

use crate::error::Error;

#[derive(Copy, Clone)]
pub enum Isa {
    NoOp,
    AddX(i32),
}

impl FromStr for Isa {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = input.trim().split(' ').collect();

        match words[..] {
            ["noop"] => Ok(Self::NoOp),
            ["addx", value] => Ok(Self::AddX(value.parse()?)),

            _ => Err(Error::ParseError),
        }
    }
}

pub struct Program(Vec<Isa>);

impl Program {
    pub fn parse() -> Result<Self, Error> {
        let stdin = stdin();
        let mut line = String::new();

        let mut program: Vec<Isa> = Vec::new();

        while stdin.read_line(&mut line)? != 0 {
            program.push(line.parse()?);
            line.clear();
        }

        Ok(Program(program))
    }

    pub fn instructions(&self) -> &Vec<Isa> {
        &self.0
    }
}
