use std::collections::{HashMap, HashSet};

use aoc22_09::{Error, Knot, Move, Rope};

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
    let mut rope = Rope::new(KNOTS);

    let mut visited: HashMap<usize, HashSet<Knot>> =
        HashMap::from([(FIRST, HashSet::new()), (LAST, HashSet::new())]);

    for m in moves {
        rope.simulate(m);

        for (&knot, visited) in visited.iter_mut() {
            visited.insert(rope[knot]);
        }
    }

    println!("Part 1: {}", visited[&FIRST].len());
    println!("Part 2: {}", visited[&LAST].len());

    Ok(())
}
