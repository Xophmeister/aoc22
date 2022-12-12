use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
struct Error;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't parse input")
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Self
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

enum Isa {
    NoOp,
    AddX(i32),
}

impl Isa {
    fn parse(input: &str) -> Result<Self, Error> {
        let words: Vec<&str> = input.split(' ').collect();

        match words[..] {
            ["noop"] => Ok(Self::NoOp),
            ["addx", value] => Ok(Self::AddX(value.parse()?)),

            _ => Err(Error),
        }
    }
}

#[derive(Copy, Clone)]
struct Cpu(usize, i32);

impl Default for Cpu {
    fn default() -> Self {
        Self(1, 1)
    }
}

impl Cpu {
    fn exec(&mut self, instruction: Isa) -> Vec<Cpu> {
        let mut state = Vec::new();

        match instruction {
            Isa::NoOp => {
                self.0 += 1;
                state.push(*self);
            }

            Isa::AddX(value) => {
                self.0 += 1;
                state.push(*self);

                self.0 += 1;
                self.1 += value;
                state.push(*self);
            }
        }

        state
    }

    fn pointer(self) -> usize {
        self.0
    }

    fn signal(self) -> i32 {
        self.0 as i32 * self.1
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut state = Cpu::default();

    let cycles = [20, 60, 100, 140, 180, 220];
    let mut signal: Vec<Cpu> = Vec::new();

    while stdin.read_line(&mut line)? != 0 {
        let instruction = Isa::parse(line.trim())?;
        let history = state.exec(instruction);

        for cycle in cycles {
            for cpu in &history {
                if cpu.pointer() == cycle {
                    signal.push(*cpu);
                }
            }
        }

        line.clear();
    }

    let strength: i32 = signal.iter().map(|cpu| cpu.signal()).sum();
    println!("{strength}");

    Ok(())
}
