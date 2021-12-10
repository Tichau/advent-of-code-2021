use std::fs::File;
use std::io;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<i32> {
    let mut inputs = helpers::parse_file_to_list(input_file, |line| { 
        let values: Vec<i32> = line.split(',').map(|str| str.parse::<i32>().unwrap()).collect();
        values
    });

    inputs.pop().unwrap()
}

pub fn part1(input: &Vec<i32>) -> i32 {
    let mut fishes = input.clone();

    for _day in 0..80 {
        for i in (0..fishes.len()).rev() {
            fishes[i] -= 1;
            if fishes[i] < 0 {
                fishes[i] = 6;
                fishes.push(8); // new fish
            }
        }
    }

    fishes.len() as i32
}

pub fn part2(input: &Vec<i32>) -> i64 {
    let mut fishes = [0i64; 9];
    for &fish in input {
        fishes[fish as usize] += 1;
    }

    for _day in 0..256 {
        let new_fishes = fishes[0];
        for i in 1..fishes.len() {
            fishes[i - 1] = fishes[i];
        }

        fishes[6] += new_fishes;
        fishes[8] = new_fishes;
    }

    fishes.iter().fold(0, |count, &total| total + count)
}
