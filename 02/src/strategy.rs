use crate::hand::Hand;

#[derive(Copy, Clone)]
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

    pub fn score(self) -> u32 {
        match self {
            Strategy::Win => 6,
            Strategy::Draw => 3,
            Strategy::Lose => 0,
        }
    }

    pub fn play(self, opponent: Hand) -> Hand {
        match self {
            Strategy::Win => opponent.beaten_by(),
            Strategy::Draw => opponent,
            Strategy::Lose => opponent.beats_with(),
        }
    }
}
