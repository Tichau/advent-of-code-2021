use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> (helpers::Map<char>, Vec<FoldInstruction>) {
    let regex = Regex::new(r"^fold\salong\s([xy])=([0-9]+)$").unwrap();
    let mut coordinates: Vec<helpers::Position> = Vec::new();
    let mut instructions: Vec<FoldInstruction> = Vec::new();
    let mut read_inst = false;
    let mut width = 0usize;
    let mut height = 0usize;
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if ip == "" {
                read_inst = true;
                continue;
            }

            if read_inst {
                if let Option::Some(capture) = regex.captures(&ip) {
                    let orientation = capture.get(1).unwrap().as_str();
                    let line: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    instructions.push(FoldInstruction {
                        x: if orientation == "x" { line } else { 0 }, 
                        y: if orientation == "y" { line } else { 0 }, 
                    });
                }
            } else {
                let mut parts = ip.split(',');
                let position = helpers::Position::new(parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap());
                if position.x as usize >= width { width = position.x as usize + 1 }
                if position.y as usize >= height { height = position.y as usize + 1 }
                coordinates.push(position);
            }
        }
    }

    let mut map: helpers::Map<char> = helpers::Map::new_init(width, height, '.');
    for c in coordinates {
        map.set(c, '#');
    }

    (map, instructions)
}

pub fn part1(input: &(helpers::Map<char>, Vec<FoldInstruction>)) -> usize {
    let map: &helpers::Map<char> = &input.0;
    let instructions = &input.1;

    let folded_map = process_instruction(map, &instructions[0]);
    folded_map.iter().fold(0, |count, c| if c == &'#' { count + 1 } else { count })
}

pub fn part2(input: &(helpers::Map<char>, Vec<FoldInstruction>)) -> String {
    let map: &helpers::Map<char> = &input.0;
    let instructions = &input.1;

    let mut current_map = map.clone();
    for instruction in instructions {
        current_map = process_instruction(&current_map, instruction);
    }
    
    format!("\n{}", current_map)
}

fn process_instruction(map: &helpers::Map<char>, instruction: &FoldInstruction) -> helpers::Map<char> {
    let mut fold_map: helpers::Map<char>;
    if instruction.x > 0 {
        let fold = instruction.x as usize;
        fold_map = helpers::Map::new_init(map.width - fold, map.height, '.');
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = helpers::Position::new(x as i32, y as i32);
                if x < fold {
                    if map.get(pos).unwrap() == &'#' {
                        fold_map.set(pos, '#')
                    }
                } else if x > fold {
                    if map.get(pos).unwrap() == &'#' {
                        fold_map.set(helpers::Position::new((2 * fold - x) as i32, y as i32), '#')
                    }
                }
                
            }
        }
    } else if instruction.y > 0 {
        let fold = instruction.y as usize;
        fold_map = helpers::Map::new_init(map.width, map.height - fold, '.');
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = helpers::Position::new(x as i32, y as i32);
                if y < fold {
                    if map.get(pos).unwrap() == &'#' {
                        fold_map.set(pos, '#')
                    }
                } else if y > fold {
                    if map.get(pos).unwrap() == &'#' {
                        fold_map.set(helpers::Position::new(x as i32, (2 * fold - y) as i32), '#')
                    }
                }
                
            }
        }

    } else {
        panic!("Invalid instruction");
    }

    fold_map
}

pub struct FoldInstruction {
    x: i32,
    y: i32,
}
