pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn new(encoded: &str) -> Self {
        match encoded {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,

            unknown => panic!("Cannot decode hand: {unknown:?}"),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}
