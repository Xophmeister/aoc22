mod assignment;
mod record;

use crate::record::Record;
use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut count = 0;

    loop {
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Err(e) => panic!("{e}"),

            Ok(_) => {
                if Record::from_str(line.trim()).map_or(false, |v| v.redundant()) {
                    count += 1;
                }
            }
        };

        line.clear()
    }

    println!("{count}");
}
