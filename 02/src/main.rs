mod hand;
mod round;
mod strategy;

use crate::round::Round;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut score = 0;

    loop {
        score += match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => Round::new(&line).score(),

            Err(e) => panic!("{e}"),
        };

        line.clear();
    }

    println!("{score}");
}
