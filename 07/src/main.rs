mod error;
mod inode;
mod stat;

use crate::error::ParseError;
use crate::inode::Inode;
use crate::stat::Stat;

fn main() -> Result<(), ParseError> {
    let stats = Stat::parse_session()?;

    stats
        .iter()
        .filter(|stat| stat.1 != Inode::Directory)
        .for_each(|stat| println!("{:?}", stat));

    Ok(())
}
