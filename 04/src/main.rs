mod assignment;
mod record;

use crate::record::Record;
use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut count_a = 0; // Count for part 1
    let mut count_b = 0; // Count for part 2

    loop {
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Err(e) => panic!("{e}"),

            Ok(_) => {
                let record = Record::from_str(line.trim()).unwrap();

                if record.redundant() {
                    count_a += 1;
                }

                if record.overlaps() {
                    count_b += 1;
                }
            }
        };

        line.clear()
    }

    println!("Part 1: {count_a}");
    println!("Part 2: {count_b}");
}
