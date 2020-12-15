mod parse;

use crate::Challenge;

pub struct Day08 {
    instructions: Vec<Instruction>,
}

impl Challenge for Day08 {
    fn name() -> &'static str {
        "day08"
    }
    fn new(input: String) -> Self {
        Day08 {
            instructions: parse::program(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        let mut vm = VM::new(self.instructions.clone());
        match vm.run() {
            Ok(acc) => panic!(format!("program terminated! acc: {}", acc)),
            Err(acc) => acc as usize,
        }
    }
    fn part_two(&self) -> usize {
        fix_program(self.instructions.clone()) as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug, Default, Copy, Clone)]
struct State {
    acc: isize,
    ip: isize,
}

impl Instruction {
    fn apply(&self, state: &mut State) {
        use Instruction::*;

        match self {
            Nop(_) => state.ip += 1,
            Jmp(x) => state.ip += x,
            Acc(x) => {
                state.acc += x;
                state.ip += 1;
            }
        }
    }
}

use std::collections::HashMap;
#[derive(Default)]
struct VM {
    state: State,
    instructions: Vec<Instruction>,

    // for this challenge
    step: usize,
    history: HashMap<isize, usize>, // ip -> step
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            instructions,
            ..Default::default()
        }
    }

    fn step(&mut self) {
        self.instructions[self.state.ip as usize].apply(&mut self.state);
    }

    fn run(&mut self) -> Result<isize, isize> {
        loop {
            if self.history.insert(self.state.ip, self.step).is_some() {
                return Err(self.state.acc);
            }

            if self.state.ip as usize >= self.instructions.len() {
                return Ok(self.state.acc);
            }

            self.step();
            self.step += 1;
        }
    }
}

struct InstructionIter {
    instructions: Vec<Instruction>,
    ip: usize,
}

impl Iterator for InstructionIter {
    type Item = Vec<Instruction>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ip >= self.instructions.len() {
                return None;
            }
            match self.instructions[self.ip] {
                Instruction::Jmp(x) => {
                    let mut copy = self.instructions.clone();
                    copy[self.ip] = Instruction::Nop(x);
                    self.ip += 1;
                    return Some(copy);
                }
                Instruction::Nop(x) => {
                    let mut copy = self.instructions.clone();
                    copy[self.ip] = Instruction::Jmp(x);
                    self.ip += 1;
                    return Some(copy);
                }
                _ => self.ip += 1,
            }
        }
    }
}

fn fix_program(instructions: Vec<Instruction>) -> isize {
    let iter = InstructionIter {
        instructions,
        ip: 0,
    };

    for instructions in iter {
        let mut vm = VM::new(instructions);

        if let Ok(acc) = vm.run() {
            return acc;
        }
    }
    panic!("could not fix program")
}

#[test]
fn test_program_step() {
    use Instruction::*;
    let instructions = vec![
        Nop(0),
        Acc(1),
        Jmp(4),
        Acc(3),
        Jmp(-3),
        Acc(-99),
        Acc(1),
        Jmp(-4),
        Acc(6),
    ];
    let mut vm = VM::new(instructions);

    let res = vm.run();
    assert_eq!(res, Err(5));
}

#[test]
fn test_program_step_fixed() {
    use Instruction::*;
    let instructions = vec![
        Nop(0),
        Acc(1),
        Jmp(4),
        Acc(3),
        Jmp(-3),
        Acc(-99),
        Acc(1),
        Nop(-4),
        Acc(6),
    ];
    let mut vm = VM::new(instructions);

    let res = vm.run();
    assert_eq!(res, Ok(8));
}

#[test]
fn test_program_step_fix() {
    use Instruction::*;
    let instructions = vec![
        Nop(0),
        Acc(1),
        Jmp(4),
        Acc(3),
        Jmp(-3),
        Acc(-99),
        Acc(1),
        Jmp(-4),
        Acc(6),
    ];
    let acc = fix_program(instructions);
    assert_eq!(acc, 8)
}
