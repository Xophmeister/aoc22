use std::fmt;
use std::io::stdin;
use std::ops::Sub;

use crate::error::Error;

#[derive(Clone, Copy)]
pub struct Elevation(u8);

impl From<char> for Elevation {
    fn from(input: char) -> Self {
        Self(input as u8 - b'a')
    }
}

pub struct Delta(i8);

impl Delta {
    pub fn assailable(&self) -> bool {
        self.0.abs() <= 1
    }
}

impl Sub for Elevation {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta(self.0 as i8 - rhs.0 as i8)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Map {
    field: (Vec<Elevation>, usize, usize),
    pub start: Coord,
    pub finish: Coord,
}

impl Map {
    pub fn parse() -> Result<Self, Error> {
        let stdin = stdin();
        let mut line = String::new();

        let mut field: Vec<Elevation> = Vec::new();
        let mut width: Option<usize> = None;
        let mut start: Option<Coord> = None;
        let mut end: Option<Coord> = None;

        let mut y = 0;

        while stdin.read_line(&mut line)? != 0 {
            line = line.trim().into();

            if width.is_none() {
                width = Some(line.len());
            }

            for (x, height) in line.chars().enumerate() {
                match height {
                    'a'..='z' => {
                        field.push(height.into());
                    }

                    'S' => {
                        field.push('a'.into());
                        start = Some(Coord { x, y });
                    }

                    'E' => {
                        field.push('z'.into());
                        end = Some(Coord { x, y });
                    }

                    _ => return Err(Error::ParseError),
                }
            }

            y += 1;
            line.clear();
        }

        Ok(Self {
            field: (field, width.ok_or(Error::ParseError)?, y),
            start: start.ok_or(Error::ParseError)?,
            finish: end.ok_or(Error::ParseError)?,
        })
    }

    pub fn size(&self) -> Coord {
        let (_, x, y) = self.field;
        Coord { x, y }
    }

    pub fn elevation(&self, at: Coord) -> Elevation {
        let (field, width, height) = &self.field;
        assert!(at.x < *width && at.y < *height);

        field[at.x + (at.y * width)]
    }
}
