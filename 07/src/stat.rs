use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::error::ParseError;
use crate::inode::Inode;

type Directory = PathBuf;

#[derive(Debug)]
pub struct Stat(pub Directory, pub Inode);

fn parse_cmd(input: &str, cwd: &mut Directory) -> Result<(), ParseError> {
    let argv: Vec<&str> = input.trim().split_whitespace().collect();

    if argv.is_empty() {
        return Err(ParseError);
    }

    match argv[0] {
        // Change the current working directory
        "cd" => {
            match argv[1] {
                // cwd.push("..") doesn't normalise the path
                ".." => {
                    cwd.pop();
                }

                _ => {
                    let cd = Directory::from(argv[1]);
                    cwd.push(&cd);
                }
            }
        }

        // We don't care about ls
        "ls" => (),

        // Unknown command
        _ => return Err(ParseError),
    }

    Ok(())
}

impl Stat {
    pub fn parse_session() -> Result<Vec<Stat>, ParseError> {
        let stdin = io::stdin();
        let mut line = String::new();

        let mut cwd = PathBuf::from("/");
        let mut stats: Vec<Stat> = Vec::new();

        while stdin.read_line(&mut line)? != 0 {
            if let Some(cmd) = line.strip_prefix("$ ") {
                // Parse command
                parse_cmd(cmd, &mut cwd)?;
            } else {
                // Parse directory listing
                let inode: Inode = line.parse()?;
                stats.push(Stat(cwd.clone(), inode));
            }

            line.clear()
        }

        Ok(stats)
    }

    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn is_file(&self) -> bool {
        self.1 != Inode::Directory
    }

    // Well this is gross :/
    pub fn is_under(&self, parent: &Path) -> bool {
        let dir: Vec<_> = self.0.components().collect();
        let parent: Vec<_> = parent.components().collect();

        (parent.len() <= dir.len()) && parent.iter().zip(dir.iter()).all(|(a, b)| a == b)
    }

    pub fn get_size(&self) -> u32 {
        match self.1 {
            Inode::File(_, size) => size,
            _ => 0,
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir: &str = self.0.to_str().unwrap_or("???");

        match &self.1 {
            Inode::Directory => write!(f, "{}", dir),
            Inode::File(filename, size) => write!(f, "{}\t{}\t{}", dir, filename, size),
        }
    }
}
