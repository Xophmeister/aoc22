use crate::hand::Hand;

pub enum Strategy {
    Win,
    Lose,
    Draw,
}

impl Strategy {
    pub fn new(encoded: &str) -> Self {
        match encoded {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,

            unknown => panic!("Cannot decode strategy: {unknown:?}"),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Strategy::Win => 6,
            Strategy::Draw => 3,
            Strategy::Lose => 0,
        }
    }

    pub fn play(&self, opponent: &Hand) -> Hand {
        // TODO There's something quite unsatisfying about enumerating everything...
        match (self, opponent) {
            // Winning strategies
            (Strategy::Win, Hand::Rock) => Hand::Paper,
            (Strategy::Win, Hand::Paper) => Hand::Scissors,
            (Strategy::Win, Hand::Scissors) => Hand::Rock,

            // Drawing strategies
            (Strategy::Draw, Hand::Rock) => Hand::Rock,
            (Strategy::Draw, Hand::Paper) => Hand::Paper,
            (Strategy::Draw, Hand::Scissors) => Hand::Scissors,

            // Losing strategies
            (Strategy::Lose, Hand::Rock) => Hand::Scissors,
            (Strategy::Lose, Hand::Paper) => Hand::Rock,
            (Strategy::Lose, Hand::Scissors) => Hand::Paper,
        }
    }
}
