use crate::hand::Hand;
use crate::strategy::Strategy;

pub struct Round {
    opponent: Hand,
    strategy: Strategy,
}

impl Round {
    pub fn new(encoded: &str) -> Self {
        if let Some((opponent, strategy)) = encoded.trim().split_once(' ') {
            Round {
                opponent: Hand::new(opponent),
                strategy: Strategy::new(strategy),
            }
        } else {
            panic!("Cannot decode round: {encoded:?}")
        }
    }

    pub fn score(self) -> u32 {
        let strategy = self.strategy;
        let hand = strategy.play(self.opponent);

        hand.score() + strategy.score()
    }
}
