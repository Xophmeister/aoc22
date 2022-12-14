use std::collections::VecDeque;
use std::io;

use crate::parse;

pub type Value = u64;
type MonkeyId = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Multiply(Value),
    Sum(Value),
    Square,
}

#[derive(Clone, Debug, PartialEq)]
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
        match self.operation {
            Operation::Multiply(value) => item * value,
            Operation::Sum(value) => item + value,
            Operation::Square => item * item,
        }
    }

    fn inspect(&mut self, item: Value) -> bool {
        self.inspections += 1;
        item % self.divisor == 0
    }
}

#[derive(Clone)]
pub struct Troop(
    Vec<Monkey>,
    Value, // Modulo, for Chinese Remainder Theorem
);

impl Troop {
    // .split_at_mut is a PITA, so let's throw caution to the wind...
    fn trio(&mut self, thrower: usize) -> (&mut Monkey, &mut Monkey, &mut Monkey) {
        let ptr = self.0.as_mut_ptr();

        unsafe {
            let see = &mut *ptr.add(thrower); // Thrower Monkey
            let hear = &mut *ptr.add(see.if_true); // True Catcher Monkey
            let say = &mut *ptr.add(see.if_false); // False Catcher Monkey

            (see, hear, say)
        }
    }

    /// Parse from file
    pub fn parse(mut file: impl io::Read) -> Result<Self, io::Error> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        if let Ok((_, monkeys)) = parse::troop(buffer.as_str()) {
            let modulo = monkeys.iter().map(|monkey| monkey.divisor).product();
            Ok(Troop(monkeys, modulo))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not parse input",
            ))
        }
    }

    /// Perform a single round of Keep Away at the given worry level
    pub fn play(&mut self, worry: Value) {
        let monkeys = self.0.len();
        let modulo = self.1;

        for idx in 0..monkeys {
            let (see, hear, say) = self.trio(idx);

            while let Some(item) = see.items.pop_front() {
                let item = (see.assess(item) / worry) % (modulo * worry);

                if see.inspect(item) {
                    hear.items.push_back(item);
                } else {
                    say.items.push_back(item);
                }
            }
        }
    }

    /// Calculate the Monkey Business score for the troop
    pub fn monkey_business(&self) -> Value {
        let mut inspections: Vec<Value> = self
            .0
            .iter()
            .map(|monkey| monkey.inspections as Value)
            .collect();

        inspections.sort_by(|a, b| b.partial_cmp(a).unwrap());
        inspections.iter().take(2).product()
    }
}
