use std::io::stdin;

use crate::error::Error;

#[derive(Clone, Copy)]
struct Elevation(u8);

impl From<char> for Elevation {
    fn from(input: char) -> Self {
        Elevation(input as u8 - b'a')
    }
}

#[derive(Clone, Copy)]
pub struct Coord(usize, usize);

impl Coord {
    pub fn x(self) -> usize {
        self.0
    }

    pub fn y(self) -> usize {
        self.1
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Map {
    field: (Vec<Elevation>, usize),
    start: Coord,
    end: Coord,
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
                        start = Some(Coord(x, y));
                    }

                    'E' => {
                        field.push('z'.into());
                        end = Some(Coord(x, y));
                    }

                    _ => return Err(Error::ParseError),
                }
            }

            y += 1;
            line.clear();
        }

        Ok(Map {
            field: (field, width.ok_or(Error::ParseError)?),
            start: start.ok_or(Error::ParseError)?,
            end: end.ok_or(Error::ParseError)?,
        })
    }

    fn size(&self) -> Coord {
        let (field, width) = &self.field;
        Coord(*width, field.len() / *width)
    }

    fn elevation(&self, at: Coord) -> Elevation {
        let (field, width) = &self.field;
        let idx = at.x() + (at.y() * width);
        field[idx]
    }

    pub fn routes(&self, from: Coord) -> Vec<Direction> {
        let elevation = self.elevation(from);

        todo!();
    }
}
