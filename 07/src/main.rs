mod error;
mod inode;
mod stat;

use crate::error::ParseError;
use crate::stat::Stat;

fn main() -> Result<(), ParseError> {
    let stats = Stat::parse_session()?;

    stats
        .iter()
        .filter(|stat| stat.is_file())
        .for_each(|stat| println!("{stat}"));

    Ok(())
}
