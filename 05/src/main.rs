mod crane;
mod error;
mod parse;

use crate::parse as input;

fn main() {
    let (mut dock, instructions) = input::parse().unwrap();

    for instruction in instructions {
        instruction.apply(&mut dock);
    }

    println!(
        "{}",
        dock.iter()
            .map(|stack| stack.iter().last().unwrap())
            .collect::<String>()
    );
}
