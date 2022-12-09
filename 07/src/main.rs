mod error;
mod inode;
mod stat;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::error::ParseError;
use crate::stat::Stat;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), ParseError> {
    let stats = Stat::parse_session()?;

    // Unique directories attested in session
    let directories: HashSet<&PathBuf> = HashSet::from_iter(stats.iter().map(|stat| stat.path()));

    let part_a: u32 = directories
        .iter()
        .map(|directory| {
            stats
                .iter()
                .filter(|stat| stat.is_file() && stat.is_under(directory))
                .map(|stat| stat.get_size())
                .sum()
        })
        .filter(|size: &u32| *size <= 100000)
        .sum();

    println!("Part 1: {part_a}");

    Ok(())
}
