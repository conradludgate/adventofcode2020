use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space1},
    combinator::map_res,
    combinator::value,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    let (input, sign) = alt((value(-1, char('-')), value(1, char('+'))))(input)?;
    let (input, abs) = map_res(digit1, |s: &str| s.parse::<isize>())(input)?;
    Ok((input, sign * abs))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (instruction, value)) = separated_pair(
        alt((tag("nop"), tag("acc"), tag("jmp"))),
        space1,
        parse_number,
    )(input)?;

    let instruction = match instruction {
        "nop" => Instruction::Nop(value),
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        x => panic!(format!("invalid instruction recieved: {}", x)),
    };

    Ok((input, instruction))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn read_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

use std::collections::HashMap;
struct VM {
    acc: isize,
    ip: isize,
    instructions: Vec<Instruction>,

    // for this challenge
    step: usize,
    history: HashMap<isize, usize>, // ip -> step
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            acc: 0,
            ip: 0,
            instructions,

            step: 0,
            history: HashMap::new(),
        }
    }

    fn step(&mut self){
        match self.instructions[self.ip as usize] {
            Instruction::Nop(_) => self.ip += 1,
            Instruction::Jmp(x) => self.ip += x,
            Instruction::Acc(x) => {
                self.acc += x;
                self.ip += 1;
            }
        }

        self.step += 1;
    }

    fn run(&mut self) -> Result<isize, isize> {
        loop {
            if self.history.insert(self.ip, self.step).is_some() {
                return Err(self.acc);
            }
    
            if self.ip as usize >= self.instructions.len() {
                return Ok(self.acc);
            }

            self.step();
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
    let iter = InstructionIter{
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

fn main() {
    let input = read_file();
    let (_, instructions) = parse_program(&input).unwrap();
    let mut vm = VM::new(instructions.clone());
    match vm.run() {
        Ok(acc) => println!("program terminated! acc: {}", acc),
        Err(acc) => println!("loop encountered! acc: {}", acc),
    }

    let acc = fix_program(instructions);
    println!("Program fixed! acc: {}", acc);
}

#[test]
fn test_parse_program() {
    use Instruction::*;

    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let (input, instructions) = parse_program(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(instructions, vec![
        Nop(0),
        Acc(1),
        Jmp(4),
        Acc(3),
        Jmp(-3),
        Acc(-99),
        Acc(1),
        Jmp(-4),
        Acc(6),
    ])
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
