mod crane;
mod error;
mod parse;

use crate::crane::Model;
use crate::parse as input;

fn read_stacks(dock: &Vec<Vec<char>>) -> String {
    dock.iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect::<String>()
}

fn main() {
    let (mut dock_a, instructions) = input::parse().unwrap();
    let mut dock_b = dock_a.clone();

    for instruction in instructions {
        instruction.apply(&mut dock_a, Model::CrateMover9000);
        instruction.apply(&mut dock_b, Model::CrateMover9001);
    }

    println!("Part 1: {}", read_stacks(&dock_a));
    println!("Part 2: {}", read_stacks(&dock_b));
}
