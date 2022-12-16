use aoc22_12::{Error, Map, Route};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let map = Map::parse()?;

    let route = Route::try_from(&map)?;
    println!("Part 1: {}", route.len());

    Ok(())
}
