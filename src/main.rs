use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    day4();
    return;
    day3s2();
    day3s1();
    day2s2();
    day2s1();
    day1s2();
    day1s1();
}

// DAY 4

fn day4() {
    let (numbers, mut grids) = {
        let regex = Regex::new(r"^\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*$").unwrap();
        let mut numbers: Vec<u8> = Vec::new();
        let mut grids: Vec<Grid> = Vec::new();
        
        if let Ok(lines) = read_lines("day4.txt") {
            let mut grid: Grid = Grid::new();
            let mut row = 0;
            for line in lines {
                if let Ok(ip) = line {
                    if numbers.len() == 0 {
                        numbers = ip.split(',').map(|str| str.parse::<u8>().unwrap()).collect();  
                    } else {
                        if let Option::Some(capture) = regex.captures(&ip) {
                            let mut col = 0;
                            for i in 1..capture.len() {
                                grid.grid[row][col].number = capture.get(i).unwrap().as_str().parse::<u8>().unwrap();
                                col += 1;
                            }

                            row += 1;
                            if row == 5 {
                                // New grid
                                grids.push(grid);
                                grid = Grid::new();
                                row = 0;
                            }
                        }
                    }
                }
            }
        }

        (numbers, grids)
    };

    let mut first_to_win = -1;
    let mut last_to_win = -1;
    for n in numbers {
        for i in (0..grids.len()).rev() {
            let grid = &mut grids[i];
            grid.mark(n);
            let (victory, unmarked_sum) = grid.check_victory();
            if victory {
                if first_to_win < 0 { first_to_win = n as i32 * unmarked_sum };
                last_to_win = n as i32 * unmarked_sum;
                grids.remove(i);
            }
        }
    }

    println!("Day4.1: First grid to win: {}", first_to_win);
    println!("Day4.2: Last grid to win: {}", last_to_win);
}

struct Grid {
    grid: [[ Cell; 5]; 5],
}

impl Grid {
    fn new() -> Self {
        Self {
          grid: [[Cell {number: 0, marked: false}; 5]; 5]
        }
    }

    fn mark(&mut self, n: u8) {
        self.grid.iter_mut().for_each(|line| line.iter_mut().for_each(|cell| if cell.number == n { cell.marked = true }));
    }

    fn check_victory(&self) -> (bool, i32) {
        let mut unmarked_sum = 0i32;
        let mut col_count = [0i32; 5];
        let mut win = false;
        for (_, line) in self.grid.iter().enumerate() {
            let mut row_count = 0;
            for (col, cell) in line.iter().enumerate() {
                if cell.marked { row_count += 1; col_count[col] += 1 } else { unmarked_sum += cell.number as i32 };
            }

            if row_count == 5 { win = true };
        }

        col_count.iter().for_each(|c| if *c == 5 { win = true });

        (win, unmarked_sum)
    }
}

#[derive(Copy, Clone)]
struct Cell {
    number: u8,
    marked: bool,
}

// DAY 3

fn day3s1() {
    let inputs: Vec<String> = parse_file_to_list("day3.txt", |line| { String::from(line) });

    let half = inputs.len() as i32 / 2;

    let mut count: Vec<i32> = vec![0; inputs[0].len()];
    for input in inputs {
        input.chars().enumerate().for_each(|(i, c)| {
            if c == '0' {
                count[i] += 1;
            }
        });
    }

    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();
    for c in count {
        if c > half {
            gamma_string.push('0');
            epsilon_string.push('1');
        } else {
            gamma_string.push('1');
            epsilon_string.push('0');
        }
    }

    let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();

    println!("Day3.1: epsilon: {}, gamma: {}, answer: {}", epsilon, gamma, gamma * epsilon);
}

fn day3s2() {
    let inputs: Vec<String> = parse_file_to_list("day3.txt", |line| { String::from(line) });

    // Oxygen: bit with most common value (1 if equal)
    let mut values = inputs.clone();
    let mut index = 0;
    loop {
        let half = values.len() as i32 / 2;
        let mut count: i32 = 0;
        for value in &values {
            if value.chars().nth(index).unwrap() == '0' {
                count += 1;
            }
        }

        let wanted = if count > half { '0' } else { '1' };
        for i in (0..values.len()).rev() {
            if values[i].chars().nth(index).unwrap() != wanted {
                values.remove(i);
            }
        }

        index += 1;
        if values.len() <= 1 {
            break;
        }
    }

    let oxygen_string = values.last().unwrap();
    
    // CO2: bit with least common value (0 if equal)
    let mut values = inputs.clone();
    let mut index = 0;
    loop {
        let half = values.len() as i32 / 2;
        let mut count: i32 = 0;
        for value in &values {
            if value.chars().nth(index).unwrap() == '0' {
                count += 1;
            }
        }

        let wanted = if count <= half { '0' } else { '1' };
        for i in (0..values.len()).rev() {
            if values[i].chars().nth(index).unwrap() != wanted {
                values.remove(i);
            }
        }

        index += 1;
        if values.len() <= 1 {
            break;
        }
    }

    let co2_string = values.last().unwrap();

    let oxygen = isize::from_str_radix(oxygen_string, 2).unwrap();
    let co2 = isize::from_str_radix(co2_string, 2).unwrap();

    println!("Day3.2: oxygen: {}, co2: {}, answer: {}", oxygen, co2, oxygen * co2);
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
