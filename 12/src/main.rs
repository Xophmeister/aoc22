use aoc22_12::{Error, Map};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let map = Map::parse()?;

    //println!("{} x {}", map.size().x(), map.size().y());

    Ok(())
}
