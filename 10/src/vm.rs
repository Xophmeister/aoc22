use crate::isa::{Isa, Program};

type Cycle = usize;
type Register = i32;
type Stack = Vec<Isa>;

pub struct Vm(Cycle, Register, Stack);

impl Default for Vm {
    fn default() -> Self {
        Vm(0, 1, Vec::new())
    }
}

impl Vm {
    pub fn exec(mut self, program: &Program) -> Execution {
        // Load program into stack...
        let stack = &mut self.2;
        for instruction in program.instructions().iter().rev() {
            stack.push(*instruction);
        }

        // ...and execute
        Execution::exec(self)
    }

    fn pop(&mut self) -> Option<Isa> {
        self.2.pop()
    }
}

pub struct Execution {
    vm: Vm,
    todo: Option<Isa>,
}

impl Execution {
    fn exec(vm: Vm) -> Self {
        Execution { vm, todo: None }
    }
}

impl Iterator for Execution {
    type Item = (Cycle, Register);

    fn next(&mut self) -> Option<Self::Item> {
        // Increase the cycle count
        self.vm.0 += 1;
        let original_x = self.vm.1;

        // Mutate register
        if let Some(Isa::AddX(value)) = self.todo {
            self.vm.1 += value;
            self.todo = None;
        } else if let Some(instruction) = self.vm.pop() {
            self.todo = Some(instruction);
        } else {
            // End of program
            return None;
        }

        // We return the register value "during" the cycle, not after
        Some((self.vm.0, original_x))
    }
}
