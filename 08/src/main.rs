mod forest;

use std::io;

use crate::forest::Forest;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    let trees = Forest::parse()?;

    println!("Part 1: {}", trees.visible());
    println!("Part 2: {}", trees.scenic_score());

    Ok(())
}
