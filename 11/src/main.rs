use std::io;

use aoc22_11::{Troop, Value};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    let mut troop = Troop::parse(io::stdin())?;

    let part_a: Value = {
        let mut troop = troop.clone();
        let worry_level = 3;

        for _round in 0..20 {
            troop.play(worry_level);
        }

        troop.monkey_business()
    };

    println!("Part 1: {part_a}");

    let part_b: Value = {
        let worry_level = 1;

        for _round in 0..10000 {
            troop.play(worry_level);
        }

        troop.monkey_business()
    };

    println!("Part 2: {part_b}");

    Ok(())
}
