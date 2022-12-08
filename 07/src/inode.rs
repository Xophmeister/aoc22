use std::str::FromStr;

use crate::error::ParseError;

type Filename = String;
type Size = u32;

#[derive(Debug, PartialEq)]
pub enum Inode {
    Directory,
    File(Filename, Size),
}

impl FromStr for Inode {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (inode, file) = input.trim().split_once(' ').ok_or(ParseError)?;

        match inode {
            // We don't really care about directories
            "dir" => Ok(Inode::Directory),

            // We only care about files (i.e., leaves)
            _ => {
                let filename: String = file.into();
                let size: u32 = inode.parse()?;

                Ok(Inode::File(filename, size))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inode() -> Result<(), ParseError> {
        assert_eq!("dir abc".parse::<Inode>()?, Inode::Directory);

        assert_eq!(
            "123 xyz".parse::<Inode>()?,
            Inode::File(String::from("xyz"), 123)
        );

        Ok(())
    }
}
