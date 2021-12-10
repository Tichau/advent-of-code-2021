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
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;
    input.iter().for_each(|&p| {
        if p < min { min = p }
        if p > max { max = p }
    });

    let mut min_conso = std::i32::MAX;
    for position in min..max {
        let conso = input.iter().fold(0, |conso, &p| conso + (position - p).abs());
        if conso < min_conso {
            min_conso = conso;
        }
    }

    min_conso
}

pub fn part2(input: &Vec<i32>) -> i64 {
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;
    input.iter().for_each(|&p| {
        if p < min { min = p }
        if p > max { max = p }
    });

    let mut min_conso = std::i64::MAX;
    for position in min..max {
        let conso: i64 = input.iter().fold(0, |conso, &p| {
            let d = (position - p).abs() as i64;
            conso + d*(d+1) / 2 // arithmetic sum
        });
        if conso < min_conso {
            min_conso = conso;
        }
    }

    min_conso
}
