mod error;
mod inode;
mod stat;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::error::ParseError;
use crate::stat::Stat;

const DISK_SPACE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;

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

    let dir_sizes: Vec<u32> = directories
        .iter()
        .map(|directory| {
            stats
                .iter()
                .filter(|stat| stat.is_file() && stat.is_under(directory))
                .map(|stat| stat.get_size())
                .sum()
        })
        .collect();

    let used_space: u32 = stats
        .iter()
        .filter(|stat| stat.is_file() && stat.is_under(&PathBuf::new()))
        .map(|stat| stat.get_size())
        .sum();

    let need_to_delete: u32 = NEEDED_SPACE - (DISK_SPACE - used_space);

    let part_a: u32 = dir_sizes.iter().filter(|size| **size <= 100000).sum();

    let part_b: u32 = *dir_sizes
        .iter()
        .filter(|size| **size >= need_to_delete)
        .min()
        .unwrap();

    println!("Part 1: {part_a}");
    println!("Part 2: {part_b}");

    Ok(())
}
