use std::fs::File;
use std::io;
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Command> {
    let regex = Regex::new(r"^(forward|down|up)\s([0-9]+)$").unwrap();
    helpers::parse_file_to_list(input_file, |line| {
        let capture = regex.captures(line).unwrap();
        let instruction = match capture.get(1).unwrap().as_str() {
            "forward" => Instruction::Forward,
            "down" => Instruction::Down,
            "up" => Instruction::Up,
            _ => panic!("unknown instruction"),
        };

        let distance = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        Command {
            instruction,
            distance,
        }
    })
}

pub fn part1(input: &Vec<Command>) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    for command in input {
        match command.instruction {
            Instruction::Forward => position += command.distance,
            Instruction::Down => depth += command.distance,
            Instruction::Up => depth -= command.distance,
        }
    };
    
    println!("Final position: {} depth: {}", position, depth);

    position * depth
}

pub fn part2(input: &Vec<Command>) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for command in input {
        match command.instruction {
            Instruction::Forward => { 
                position += command.distance;
                depth += command.distance * aim;
            },
            Instruction::Down => aim += command.distance,
            Instruction::Up => aim -= command.distance,
        }
    };
    
    println!("Final position: {} depth: {} aim: {}", position, depth, aim);

    position * depth
}

enum Instruction {
    Down,
    Up,
    Forward,
}

pub struct Command {
    instruction: Instruction,
    distance: i32,
}
