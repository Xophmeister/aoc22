use aoc22_10::{Error, Program, Vm};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let program = Program::parse()?;

    let part_a: i32 = {
        let vm = Vm::default();
        let interesting = vec![20, 60, 100, 140, 180, 220];

        vm.exec(&program)
            .filter(|(cycle, _)| interesting.contains(cycle))
            .map(|(cycle, x)| cycle as i32 * x)
            .sum()
    };

    println!("Part 1: {part_a}");

    Ok(())
}
