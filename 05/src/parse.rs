use std::io;

use crate::crane::Crane;
use crate::error::ParseError;

type Stack = Vec<char>;
type Dock = Vec<Stack>;
type Instructions = Vec<Crane>;

fn update_dock(dock: &mut Dock, input: &str) -> Result<(), ParseError> {
    // Mercifully, AoC pad the dock diagram lines to a set width
    // Width = (Stacks * 4) - 1
    let stacks = (input.len() + 1) / 4;

    // Initialise the dock
    if dock.is_empty() {
        for _ in 0..stacks {
            dock.push(Vec::new());
        }
    }

    for stack in 0..stacks {
        // Index = (Stack * 4) + 1
        let idx: usize = (stack * 4) + 1;
        let item = input.as_bytes()[idx] as char;

        match item {
            'A'..='Z' => dock[stack].push(item),
            _ => continue,
        }
    }

    Ok(())
}

pub fn parse() -> Result<(Dock, Instructions), ParseError> {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut dock: Dock = Vec::new();
    let mut instructions: Instructions = Vec::new();

    loop {
        match stdin.read_line(&mut line) {
            Err(_) => return Err(ParseError),
            Ok(0) => break,

            Ok(_) => match line.trim_end_matches('\n') {
                // Ignore the blank line separator
                "" => (),

                // Otherwise, try to parse the line as a crane instruction.
                // If that fails, then the only other option is part of the dock diagram.
                record => {
                    if let Ok(crane) = record.parse::<Crane>() {
                        instructions.push(crane);
                    } else {
                        update_dock(&mut dock, record)?;
                    }
                }
            },
        }

        line.clear();
    }

    // Reverse each stack in the dock, so they can be used like actual stacks
    dock.iter_mut().for_each(|stack| stack.reverse());

    Ok((dock, instructions))
}
