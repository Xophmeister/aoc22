use std::io;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn decode(encrypted: &str) -> Self {
        match encrypted {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,

            unknown => panic!("Cannot decode hand: {unknown:?}"),
        }
    }

    fn score(hand: &Self) -> u32 {
        match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

fn score_round(hands: &(Hand, Hand)) -> u32 {
    let (opponent, you) = hands;
    let hand_score = Hand::score(you);
    let result = match (you, opponent) {
        // Winning conditions
        (Hand::Rock, Hand::Scissors)
        | (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper) => 6,

        // Draws
        (x, y) if x == y => 3,

        // Losses
        _ => 0,
    };

    hand_score + result
}

fn decode_round(round: &str) -> (Hand, Hand) {
    if let Some((opponent, you)) = round.trim().split_once(" ") {
        (Hand::decode(opponent), Hand::decode(you))
    } else {
        panic!("Cannot decode round: {round:?}")
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut score = 0;

    loop {
        score += match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => score_round(&decode_round(&line)),

            Err(e) => panic!("{e}"),
        };

        line.clear();
    }

    println!("{score}");
}
