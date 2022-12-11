mod error;
mod moves;
mod rope;

use std::collections::{HashMap, HashSet};

use crate::error::Error;
use crate::moves::Move;
use crate::rope::Knot;

const KNOTS: usize = 10;
const FIRST: usize = 1;
const LAST: usize = KNOTS - 1;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let moves = Move::parse()?;
    let mut rope = vec![Knot::new(); KNOTS];

    let mut visited: HashMap<usize, HashSet<Knot>> =
        HashMap::from([(FIRST, HashSet::new()), (LAST, HashSet::new())]);

    for m in moves {
        rope[0].travel(m);

        for i in 1..KNOTS {
            // We have to use .split_at_mut to appease the borrow checker
            if let ([.., prior], [this, ..]) = rope.split_at_mut(i) {
                let path = this.follow(prior);

                // Record unique locations visited by the first (after head) and last knots
                if i == FIRST || i == LAST {
                    for location in path {
                        visited.get_mut(&i).unwrap().insert(location);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", visited[&FIRST].len());
    println!("Part 2: {}", visited[&LAST].len());

    Ok(())
}
