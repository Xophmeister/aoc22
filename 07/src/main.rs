mod error;
mod inode;
mod stat;

use crate::inode::Inode;
use crate::stat::Stat;

fn main() {
    Stat::parse_session()
        .unwrap()
        .iter()
        .filter(|stat| stat.1 != Inode::Directory)
        .for_each(|stat| println!("{:?}", stat));
}
