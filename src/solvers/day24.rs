use std::fs::File;
use std::io;
use std::fmt::{Display, Formatter, Result};
use std::ops::{AddAssign, MulAssign, DivAssign, RemAssign};
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Instruction> {
    let regex = Regex::new(r"^([a-z]+) (w|x|y|z|-?[0-9]+)( (w|x|y|z|-?[0-9]+))?$").unwrap();
    helpers::parse_file_to_list(input_file, |line| {
        if let Option::Some(capture) = regex.captures(&line) {
            let instruction = capture.get(1).unwrap().as_str();
            return match instruction {
                "inp" => Instruction::Inp(parse_operand(capture.get(2).unwrap().as_str())),
                "add" => Instruction::Add(parse_operand(capture.get(2).unwrap().as_str()), parse_operand(capture.get(4).unwrap().as_str())),
                "mul" => Instruction::Mul(parse_operand(capture.get(2).unwrap().as_str()), parse_operand(capture.get(4).unwrap().as_str())),
                "div" => Instruction::Div(parse_operand(capture.get(2).unwrap().as_str()), parse_operand(capture.get(4).unwrap().as_str())),
                "mod" => Instruction::Mod(parse_operand(capture.get(2).unwrap().as_str()), parse_operand(capture.get(4).unwrap().as_str())),
                "eql" => Instruction::Eql(parse_operand(capture.get(2).unwrap().as_str()), parse_operand(capture.get(4).unwrap().as_str())),
                _ => panic!("Unknown instruction {}", instruction),
            }
        } else {
            panic!("Unknown instruction {}", line);
        }
    })
}

fn parse_operand(input: &str) -> Operand {
    match input {
        "w" => Operand::Variable(0),
        "x" => Operand::Variable(1),
        "y" => Operand::Variable(2),
        "z" => Operand::Variable(3),
        _ => Operand::Number(input.parse().unwrap()),
    }
}

pub fn part1(input: &Vec<Instruction>) -> i64 {
    let mut context = Context::new([9i32; 14], 1);

    loop {
        context.stdinptr = 0;
        context.reset_memory();
        for instruction in input {
            instruction.execute(&mut context);
        }

        if let Memory::Value(value) = context.memory[3] {
            if value == 0 {
                // The MONAD is valid.
                break;
            } else {
                // No solutions
                context.stdin[context.stdinsize - 1] -= 1;
                assert!(context.stdin[context.stdinsize - 1] < 10);
            }
        } else if let Memory::Range(start, _) = context.memory[3] {
            if start == 0 {
                // Potential solutions
                context.stdinsize += 1;
            } else {
                // No solutions
                context.stdin[context.stdinsize - 1] -= 1;
                assert!(context.stdin[context.stdinsize - 1] < 10);
            }
        }
    }

    context.stdin_as_number()
}

pub fn part2(input: &Vec<Instruction>) -> i64 {
    let mut context = Context::new([1i32; 14], 1);

    loop {
        context.stdinptr = 0;
        context.reset_memory();
        for instruction in input {
            instruction.execute(&mut context);
        }
        
        if let Memory::Value(value) = context.memory[3] {
            if value == 0 {
                // The MONAD is valid.
                break;
            } else {
                // No solutions
                context.stdin[context.stdinsize - 1] += 1;
                while context.stdin[context.stdinsize - 1] == 10 {
                    context.stdin[context.stdinsize - 1] = 1;
                    context.stdinsize -= 1;
                    context.stdin[context.stdinsize - 1] += 1;
                }
            }
        } else if let Memory::Range(start, _) = context.memory[3] {
            if start == 0 {
                // Potential solutions
                context.stdinsize += 1;
            } else {
                // No solutions
                context.stdin[context.stdinsize - 1] += 1;
                while context.stdin[context.stdinsize - 1] == 10 {
                    context.stdin[context.stdinsize - 1] = 1;
                    context.stdinsize -= 1;
                    context.stdin[context.stdinsize - 1] += 1;
                }
            }
        }
    }

    context.stdin_as_number()
}

#[derive(Copy, Clone)]
pub enum Operand {
    Number(i32),
    Variable(usize),
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl Instruction {
    fn execute(&self, context: &mut Context) {
        match self {
            Self::Inp(Operand::Variable(ptr)) => context.memory[*ptr] = context.next_input(),
            Self::Add(Operand::Variable(ptr), value) => context.memory[*ptr] += context.evaluate(value),
            Self::Mul(Operand::Variable(ptr), value) => context.memory[*ptr] *= context.evaluate(value),
            Self::Div(Operand::Variable(ptr), value) => context.memory[*ptr] /= context.evaluate(value),
            Self::Mod(Operand::Variable(ptr), value) => context.memory[*ptr] %= context.evaluate(value),
            Self::Eql(Operand::Variable(ptr), value) => context.memory[*ptr] = context.memory[*ptr].equals(&context.evaluate(value)),
            _ => panic!("Invalid instruction"),
        }
    }
}

pub struct Context {
    stdin: [i32; 14],
    stdinptr: usize,
    stdinsize: usize,
    memory: [Memory; 4],
}

impl Context {
    fn new(stdin: [i32; 14], stdinsize: usize) -> Self {
        Context {
            stdin,
            stdinptr: 0,
            stdinsize,
            memory: [Memory::Value(0); 4],
        }
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "stdin: {}", self.stdin_as_number())?;
        writeln!(f, "w: {}", self.memory[0].to_string())?;
        writeln!(f, "x: {}", self.memory[1].to_string())?;
        writeln!(f, "y: {}", self.memory[2].to_string())?;
        writeln!(f, "z: {}", self.memory[3].to_string())?;
        
        Ok(())
    }
}

impl Context {
    fn next_input(&mut self) -> Memory {
        if self.stdinptr >= self.stdinsize {
            Memory::Range(1, 9)
        } else {
            self.stdinptr += 1;
            Memory::Value(self.stdin[self.stdinptr - 1] as i64)
        }
    }

    fn evaluate(&self, operand: &Operand) -> Memory {
        match operand {
            Operand::Number(number) => Memory::Value(*number as i64),
            Operand::Variable(ptr) => self.memory[*ptr],
        }
    }

    fn reset_memory(&mut self) {
        self.memory[0] = Memory::Value(0);
        self.memory[1] = Memory::Value(0);
        self.memory[2] = Memory::Value(0);
        self.memory[3] = Memory::Value(0);
    }

    fn stdin_as_number(&self) -> i64 {
        let mut result : i64 = 0;
        for i in 0..self.stdinsize {
            assert!(self.stdin[i] >= 1 && self.stdin[i] <= 9);
            result += self.stdin[i] as i64 * 10i64.pow(13-i as u32);
        }

        result
    }
}

#[derive(Copy, Clone)]
pub enum Memory {
    Value(i64),
    Range(i64,i64),
}

impl Memory {
    fn equals(&self, other: &Self) -> Self {
        match self {
            Memory::Value(value) => {
                match other {
                    Memory::Value(other_value) => Memory::Value(if *value == *other_value { 1 } else { 0 }),
                    Memory::Range(other_start, other_end) => if value >= other_start && value <= other_end { Memory::Range(0, 1) } else { Memory::Value(0) },
                }
            },
            Memory::Range(start, end) => {
                match other {
                    Memory::Value(other_value) => if other_value >= start && other_value <= end { Memory::Range(0, 1) } else { Memory::Value(0) },
                    Memory::Range(other_start, other_end) => if start > other_end || end < other_start { Memory::Value(0) } else { Memory::Range(0, 1) },
                }
            },
        }
    }
}

impl AddAssign for Memory {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Memory::Value(value) => {
                match rhs {
                    Memory::Value(rhs_value) => *value += rhs_value,
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*value + rhs_start, *value + rhs_end),
                }
            },
            Memory::Range(start, end) => {
                match rhs {
                    Memory::Value(rhs_value) => *self = Memory::Range(*start + rhs_value, *end + rhs_value),
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*start + rhs_start, *end + rhs_end),
                }
            },
        }
    }
}

impl MulAssign for Memory {
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Memory::Value(value) => {
                match rhs {
                    Memory::Value(rhs_value) => *value *= rhs_value,
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*value * rhs_start, *value * rhs_end),
                }
            },
            Memory::Range(start, end) => {
                match rhs {
                    Memory::Value(rhs_value) => *self = Memory::Range(*start * rhs_value, *end * rhs_value),
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*start * rhs_start, *end * rhs_end),
                }
            },
        }
    }
}

impl DivAssign for Memory {
    fn div_assign(&mut self, rhs: Self) {
        match self {
            Memory::Value(value) => {
                match rhs {
                    Memory::Value(rhs_value) => *value /= rhs_value,
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*value / rhs_start, *value / rhs_end),
                }
            },
            Memory::Range(start, end) => {
                match rhs {
                    Memory::Value(rhs_value) => *self = Memory::Range(*start / rhs_value, *end / rhs_value),
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*start / rhs_start, *end / rhs_end),
                }
            },
        }
    }
}

impl RemAssign for Memory {
    fn rem_assign(&mut self, rhs: Self) {
        match self {
            Memory::Value(value) => {
                match rhs {
                    Memory::Value(rhs_value) => *value %= rhs_value,
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*value % rhs_start, *value % rhs_end),
                }
            },
            Memory::Range(start, end) => {
                match rhs {
                    Memory::Value(rhs_value) => *self = Memory::Range(*start % rhs_value, *end % rhs_value),
                    Memory::Range(rhs_start, rhs_end) => *self = Memory::Range(*start % rhs_start, *end % rhs_end),
                }
            },
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Memory::Value(value) => write!(f, "{}", value.to_string())?,
            Memory::Range(start, end) => write!(f, "{}..{}", start.to_string(), end.to_string())?,
        }
        
        Ok(())
    }
}
