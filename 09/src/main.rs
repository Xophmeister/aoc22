mod error;
mod moves;
mod rope;

use std::collections::HashSet;

use crate::error::Error;
use crate::moves::Move;
use crate::rope::RopeEnd;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let moves = Move::parse()?;

    let mut head = RopeEnd::new();
    let mut tail = RopeEnd::new();

    let mut visited: HashSet<RopeEnd> = HashSet::new();

    for m in moves {
        head.movement(m);
        for location in tail.follow(&head) {
            visited.insert(location);
        }
    }

    println!("Part 1: {}", visited.len());

    Ok(())
}
