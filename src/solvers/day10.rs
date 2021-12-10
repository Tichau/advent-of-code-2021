use std::fs::File;
use std::io;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Vec<char>> {
    helpers::parse_file_to_list(input_file, |line| {
        line.chars().collect()
    })
}

pub fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut score = 0;
    let mut chunks: Vec<char> = Vec::new();
    for line in input {
        chunks.clear();
        let mut corrupted_char: char = ' ';
        for c in line {
            match c {
                '('|'['|'{'|'<' => chunks.push(c.clone()),
                ')' => if chunks.pop() != Some('(') { corrupted_char = c.clone(); break;},
                ']' => if chunks.pop() != Some('[') { corrupted_char = c.clone(); break;},
                '}' => if chunks.pop() != Some('{') { corrupted_char = c.clone(); break;},
                '>' => if chunks.pop() != Some('<') { corrupted_char = c.clone(); break;},
                _ => println!("Unknown character '{}'", c),
            }
        }

        match corrupted_char {
            ')' => score += 3,
            ']' => score += 57,
            '}' => score += 1197,
            '>' => score += 25137,
            _ => (),
        }
    }

    score
}

pub fn part2(input: &Vec<Vec<char>>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    let mut chunks: Vec<char> = Vec::new();
    for line in input {
        chunks.clear();
        let mut corrupted: bool = false;
        for c in line {
            match c {
                '('|'['|'{'|'<' => chunks.push(c.clone()),
                ')' => if chunks.pop() != Some('(') { corrupted = true; break; },
                ']' => if chunks.pop() != Some('[') { corrupted = true; break; },
                '}' => if chunks.pop() != Some('{') { corrupted = true; break; },
                '>' => if chunks.pop() != Some('<') { corrupted = true; break; },
                _ => println!("Unknown character '{}'", c),
            }
        }

        if corrupted {
            continue;
        }

        let mut score: i64 = 0;
        for i in (0..chunks.len()).rev() {
            score *= 5;
            match chunks[i] {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => (),
            }
        }
        
        scores.push(score);
    }

    scores.sort();

    scores[scores.len() / 2]
}
