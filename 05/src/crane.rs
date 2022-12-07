use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::ParseError;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?x)
        ^                # Anchor to start of string
        move \s          # 'move '
        (?P<crates>\d+)  # crates: At least one digit
        \s from \s       # ' from '
        (?P<from>\d+)    # from: At least one digit
        \s to \s         # ' to '
        (?P<to>\d+)      # to: At least one digit
        $                # Anchor to end of string
        "
    )
    .unwrap();
}

pub enum Model {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug)]
pub struct Crane {
    pub crates: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Crane {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parsed = RE.captures(input).ok_or(ParseError)?;
        Ok(Crane {
            crates: parsed["crates"].parse().unwrap(),
            from: parsed["from"].parse().unwrap(),
            to: parsed["to"].parse().unwrap(),
        })
    }
}

impl Crane {
    pub fn apply<T>(&self, dock: &mut [Vec<T>], model: Model) {
        match model {
            Model::CrateMover9000 => {
                for _ in 0..self.crates {
                    let item = dock[self.from - 1].pop().unwrap();
                    dock[self.to - 1].push(item);
                }
            }

            Model::CrateMover9001 => {
                let at = dock[self.from - 1].len() - self.crates;
                let mut items = dock[self.from - 1].split_off(at);
                dock[self.to - 1].append(&mut items);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let crates = 1;
        let from = 2;
        let to = 3;

        let input = format!("move {crates} from {from} to {to}");
        let created = Crane::from_str(&input).unwrap();

        let expected = Crane { crates, from, to };

        assert_eq!(created.crates, expected.crates);
        assert_eq!(created.from, expected.from);
        assert_eq!(created.to, expected.to);
    }

    #[test]
    fn crane9000() {
        let mut dock: Vec<Vec<char>> = vec![vec!['A', 'B', 'C'], vec![]];

        Crane {
            crates: 2,
            from: 1,
            to: 2,
        }
        .apply(&mut dock, Model::CrateMover9000);

        assert_eq!(dock[0], vec!['A']);
        assert_eq!(dock[1], vec!['C', 'B']);

        Crane {
            crates: 1,
            from: 2,
            to: 1,
        }
        .apply(&mut dock, Model::CrateMover9000);

        assert_eq!(dock[0], vec!['A', 'B']);
        assert_eq!(dock[1], vec!['C']);
    }

    #[test]
    fn crane9001() {
        let mut dock: Vec<Vec<char>> = vec![vec!['A', 'B', 'C'], vec![]];

        Crane {
            crates: 2,
            from: 1,
            to: 2,
        }
        .apply(&mut dock, Model::CrateMover9001);

        assert_eq!(dock[0], vec!['A']);
        assert_eq!(dock[1], vec!['B', 'C']);

        Crane {
            crates: 1,
            from: 2,
            to: 1,
        }
        .apply(&mut dock, Model::CrateMover9001);

        assert_eq!(dock[0], vec!['A', 'C']);
        assert_eq!(dock[1], vec!['B']);
    }
}
