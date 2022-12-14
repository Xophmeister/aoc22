use std::collections::VecDeque;
use std::io;

use crate::parse;

pub type Value = u64;
type MonkeyId = usize;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Multiply(Value),
    Sum(Value),
    Square,
}

#[derive(Debug, PartialEq)]
pub struct Monkey {
    items: VecDeque<Value>,
    operation: Operation,
    divisor: Value,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspections: usize,
}

impl Monkey {
    pub fn new(
        items: Vec<Value>,
        operation: Operation,
        divisor: Value,
        if_true: MonkeyId,
        if_false: MonkeyId,
    ) -> Monkey {
        Monkey {
            items: VecDeque::from(items),
            operation,
            divisor,
            if_true,
            if_false,
            inspections: 0,
        }
    }

    fn assess(&self, item: Value) -> Value {
        let worry = match self.operation {
            Operation::Multiply(value) => item * value,
            Operation::Sum(value) => item + value,
            Operation::Square => item * item,
        };

        worry / 3
    }

    fn inspect(&mut self, item: Value) -> bool {
        self.inspections += 1;
        item % self.divisor == 0
    }
}

pub struct Troop(Vec<Monkey>);

impl Troop {
    pub fn parse(mut file: impl io::Read) -> Result<Self, io::Error> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        if let Ok((_, monkeys)) = parse::troop(buffer.as_str()) {
            Ok(Troop(monkeys))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not parse input",
            ))
        }
    }

    // .split_at_mut is a PITA, so I role my own
    fn trio(&mut self, thrower: usize) -> (&mut Monkey, &mut Monkey, &mut Monkey) {
        let ptr = self.0.as_mut_ptr();

        unsafe {
            let see = &mut *ptr.add(thrower);
            let hear = &mut *ptr.add(see.if_true);
            let say = &mut *ptr.add(see.if_false);

            // (Thrower, True Catcher, False Catcher)
            (see, hear, say)
        }
    }

    pub fn round(&mut self) {
        let monkeys = self.0.len();

        for idx in 0..monkeys {
            let (see, hear, say) = self.trio(idx);

            while let Some(item) = see.items.pop_front() {
                let item = see.assess(item);

                if see.inspect(item) {
                    hear.items.push_back(item);
                } else {
                    say.items.push_back(item);
                }
            }
        }
    }

    pub fn inspections(&self) -> Vec<usize> {
        self.0.iter().map(|monkey| monkey.inspections).collect()
    }
}
