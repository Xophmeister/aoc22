mod error;
mod parse;

use crate::error::ParseError;
use crate::parse::Grid;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), ParseError> {
    let trees = Grid::parse()?;

    println!("Part 1: {}", trees.visible());
    println!("Part 2: {}", trees.scenic_score());

    Ok(())
}
