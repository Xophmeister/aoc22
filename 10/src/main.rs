use aoc22_10::{Error, Program, Vm};

const CRT_WIDTH: usize = 40;

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

    let part_b: String = {
        let vm = Vm::default();

        vm.exec(&program)
            .fold(String::new(), |display, (cycle, x)| {
                let crt = (cycle - 1) % CRT_WIDTH;
                let eol = if crt == CRT_WIDTH - 1 { "\n" } else { "" };

                if (crt as i32 - x).abs() <= 1 {
                    format!("{display}##{eol}")
                } else {
                    format!("{display}  {eol}")
                }
            })
    };

    print!("Part 2:\n{part_b}");

    Ok(())
}
