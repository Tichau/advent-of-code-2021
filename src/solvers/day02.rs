use std::fs::File;
use std::io;
use regex::Regex;
use crate::helpers;

fn parse(input_file: io::BufReader<File>) -> Vec<Command> {
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

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let commands: Vec<Command> = parse(input_file);

    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    for command in commands {
        match command.instruction {
            Instruction::Forward => position += command.distance,
            Instruction::Down => depth += command.distance,
            Instruction::Up => depth -= command.distance,
        }
    };
    
    println!("Final position: {} depth: {}", position, depth);

    (position * depth) as i64
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let commands: Vec<Command> = parse(input_file);

    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    let mut aim: i32 = 0;
    for command in commands {
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

    (position * depth) as i64
}

enum Instruction {
    Down,
    Up,
    Forward,
}

struct Command {
    instruction: Instruction,
    distance: i32,
}
