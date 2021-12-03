use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    day3s2();
    day3s1();
    return;
    day2s2();
    day2s1();
    day1s2();
    day1s1();
}

// DAY 3

fn day3s1() {
    
    println!("Day3.1: answer: {}", "-");
}

fn day3s2() {

    println!("Day3.2: answer: {}", "-");
}

// DAY 2

enum Instruction {
    Down,
    Up,
    Forward,
}

struct Command {
    instruction: Instruction,
    distance: i32,
}

fn day2s1() {
    let regex = Regex::new(r"^(forward|down|up)\s([0-9]+)$").unwrap();
    let commands: Vec<Command> = parse_file_to_list("day2.txt", |line| {
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
    });

    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    for command in commands {
        match command.instruction {
            Instruction::Forward => position += command.distance,
            Instruction::Down => depth += command.distance,
            Instruction::Up => depth -= command.distance,
        }
    };
    
    println!("Day2.1: Final position: {} depth: {} answer: {}", position, depth, position * depth);
}

fn day2s2() {
    let regex = Regex::new(r"^(forward|down|up)\s([0-9]+)$").unwrap();
    let commands: Vec<Command> = parse_file_to_list("day2.txt", |line| {
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
    });

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
    
    println!("Day2.2: Final position: {} depth: {} aim: {} answer: {}", position, depth, aim, position * depth);
}

// DAY 1

fn day1s1() {
    let inputs: Vec<i32> = parse_file_to_list("day1.txt", |line| { line.parse().unwrap() });

    let mut increased: i32 = 0;
    inputs.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && *x > inputs[i-1] {
            increased += 1;
        }
    });

    println!("Day1.1: Increased {} times", increased);
}

fn day1s2() {
    let inputs: Vec<i32> = parse_file_to_list("day1.txt", |line| { line.parse().unwrap() });

    let mut windows: [(i32, i32); 3] = [(0, 0), (0, 0), (0, 0)];
    let mut windows_count: usize = 0;
    let mut current_window: usize = 0;

    let mut increased: i32 = 0;
    let mut last: i32 = 0;
    inputs.iter().enumerate().for_each(|(i, x)| {
        if windows_count < 3 {
            windows_count += 1;
        }

        for i in 0..windows_count {
            windows[i].0 += 1;
            windows[i].1 += x;
        }

        if windows[current_window].0 == 3 {
            if i > 3 && windows[current_window].1 > last {
                increased += 1;
            }

            last = windows[current_window].1;
            windows[current_window] = (0, 0);
            current_window = (current_window + 1) % 3;
        }
    });

    println!("Day1.2: Increased {} times", increased);
}

// TOOLS

fn parse_file_to_list<P, T>(filename: P, parse_func: impl Fn(&str) -> T) -> Vec<T>
where P: AsRef<Path>, {
    let mut inputs: Vec<T> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                inputs.push(parse_func(&ip));
            }
        }
    }

    inputs
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let path = Path::new("/mnt/c/Users/aallard/Documents/AdventOfCode/aoc2021/data/").join(filename);
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
