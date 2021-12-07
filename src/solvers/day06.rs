use std::fs::File;
use std::io;
use crate::helpers;

fn parse(input_file: io::BufReader<File>) -> Vec<i32> {
    let mut inputs = helpers::parse_file_to_list(input_file, |line| { 
        let values: Vec<i32> = line.split(',').map(|str| str.parse::<i32>().unwrap()).collect();
        values
    });

    inputs.pop().unwrap()
}

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let mut fishes = parse(input_file);

    for _day in 0..80 {
        for i in (0..fishes.len()).rev() {
            fishes[i] -= 1;
            if fishes[i] < 0 {
                fishes[i] = 6;
                fishes.push(8); // new fish
            }
        }
    }

    println!("{} fishes", fishes.len());

    fishes.len() as i64
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let inputs = parse(input_file);

    let mut fishes = [0i64; 9];
    for fish in inputs {
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

    let mut total: i64 = 0;
    fishes.iter_mut().for_each(|count| total += *count);
    println!("{} fishes", total);

    total
}
