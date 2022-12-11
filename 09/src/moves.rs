use std::io;

use crate::error::Error;

pub enum Move {
    Right,
    Up,
    Left,
    Down,
}

impl Move {
    pub fn parse() -> Result<Vec<Self>, Error> {
        let stdin = io::stdin();
        let mut line = String::new();

        let mut moves: Vec<Move> = Vec::new();

        while stdin.read_line(&mut line)? != 0 {
            let (direction, distance) = line.trim().split_once(' ').ok_or(Error::ParseError)?;
            let distance: usize = distance.parse()?;

            for _ in 0..distance {
                moves.push(match direction {
                    "R" => Self::Right,
                    "U" => Self::Up,
                    "L" => Self::Left,
                    "D" => Self::Down,

                    _ => return Err(Error::ParseError),
                });
            }

            line.clear();
        }

        Ok(moves)
    }
}
