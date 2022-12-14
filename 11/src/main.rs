use std::io;

use aoc22_11::{Troop, Value};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    // NOTE Default worry level is 1
    let mut troop = Troop::parse(io::stdin())?;

    let part_a: Value = {
        let mut troop = troop.clone();
        *troop.worry_level() = 3;

        for _round in 0..20 {
            troop.play();
        }

        troop.monkey_business()
    };

    println!("Part 1: {part_a}");

    let part_b: Value = {
        for _round in 0..10000 {
            troop.play();
        }

        troop.monkey_business()
    };

    println!("Part 2: {part_b}");

    Ok(())
}
