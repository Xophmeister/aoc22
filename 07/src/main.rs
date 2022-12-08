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

    let files = stats.iter().filter(|stat| stat.is_file());
    let directories: HashSet<&PathBuf> = HashSet::from_iter(files.map(|stat| stat.path()));

    for dir in directories {
        let dir_size: u32 = stats
            .iter()
            .filter(|stat| stat.is_file() && stat.is_under(dir))
            .map(|stat| stat.get_size())
            .sum();

        println!("{}\t{}", dir.to_str().unwrap(), dir_size);
    }

    Ok(())
}
