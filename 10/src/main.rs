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

    // FIXME There's an off-by-one error somewhere,
    // but the output is legible enough to read :P
    {
        let vm = Vm::default();

        println!("Part 2:");
        for (cycle, x) in vm.exec(&program) {
            let crt = (cycle - 1) % CRT_WIDTH;

            if crt == 0 {
                println!();
            }

            if (crt as i32 - x).abs() <= 1 {
                print!("##");
            } else {
                print!("  ");
            }
        }

        println!();
    }

    Ok(())
}
