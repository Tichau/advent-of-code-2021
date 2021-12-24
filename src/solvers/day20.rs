use std::fs::File;
use std::io::{self, BufRead};
use crate::helpers::{Map, Position};

pub fn parser(input_file: io::BufReader<File>) -> (String, Map<char>) {
    let mut enhancement_read = false;
    let mut enhancement = String::default();
    let mut image_data: Vec<String> = Vec::new();
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if !enhancement_read {
                enhancement = ip;
                enhancement_read = true;
            } else {
                if ip == "" {
                    continue;
                }

                image_data.push(ip);
            }
        }
    }

    let mut image: Map<char> = Map::new(image_data.last().unwrap().len(), image_data.len());
    for y in 0..image_data.len() {
        let line = &image_data[y];
        for x in 0..line.len() {
            let cell = image.get_mut(Position::new(x as i32, y as i32)).unwrap();
            *cell = line.chars().nth(x).unwrap();
        }
    }
    
    (enhancement, image)
}

pub fn part1(input: &(String, Map<char>)) -> i32 {
    let enhancement = &input.0;
    let mut input = InfiniteImage {
        image: input.1.clone(),
        infinite_char: '.',
    };

    for _ in 0..2 {
        input = enhance(&input, enhancement);
    }

    input.image.iter().fold(0, |count, &cell| count + if cell == '#' { 1 } else { 0 })
}

pub fn part2(input: &(String, Map<char>)) -> i32 {
    let enhancement = &input.0;
    let mut input = InfiniteImage {
        image: input.1.clone(),
        infinite_char: '.',
    };

    for _ in 0..50 {
        input = enhance(&input, enhancement);
    }

    input.image.iter().fold(0, |count, &cell| count + if cell == '#' { 1 } else { 0 })
}

fn enhance(input: &InfiniteImage, enhancement: &String) -> InfiniteImage {
    let offset = 2i32;
    let mut output: Map<char> = Map::new(input.image.width + offset as usize, input.image.height + offset as usize);

    for x in 0..output.width {
        for y in 0..output.height {
            let mut value = 0;
            for i in 0..9 {
                let ix = x as i32 - offset/2 + i % 3 - 1;
                let iy = y as i32 - offset/2 + i / 3 - 1;
                if let Some(&icell) = input.image.get(Position::new(ix, iy)) {
                    if icell == '#' {
                        value += 1 << (8 - i);
                    }
                } else {
                    if input.infinite_char == '#' {
                        value += 1 << (8 - i);
                    }
                }
            }
            
            let cell = output.get_mut(Position::new(x as i32, y as i32)).unwrap();
            *cell = enhancement.chars().nth(value).unwrap();
        }
    }
    
    InfiniteImage {
        image: output,
        infinite_char: if input.infinite_char == '.' { enhancement.chars().nth(0).unwrap() } else { enhancement.chars().nth(255).unwrap() },
    }
}

struct InfiniteImage {
    image: Map<char>,
    infinite_char: char, 
}
