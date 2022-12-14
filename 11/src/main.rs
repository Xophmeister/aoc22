use std::io;

use aoc22_11::Troop;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    let mut troop = Troop::parse(io::stdin())?;

    for _ in 0..20 {
        troop.round();
    }

    let part_a: u32 = {
        let mut inspections = troop.inspections();
        inspections.sort_by(|a, b| b.partial_cmp(a).unwrap());

        inspections
            .iter()
            .take(2)
            .fold(1, |acc, &count| acc * count as u32)
    };
    println!("Part 1: {part_a}");

    Ok(())
}
