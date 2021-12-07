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
    let positions = parse(input_file);

    let mut min = 99999;
    let mut max = -99999;
    positions.iter().for_each(|p| {
        if *p < min { min = *p }
        if *p > max { max = *p }
    });

    let mut min_conso = 9999999;
    let mut min_position = 0;
    for position in min..max {
        let mut conso = 0;
        positions.iter().for_each(|p| conso += (position - *p).abs());
        if conso < min_conso {
            min_conso = conso;
            min_position = position;
        }
    }

    println!("Position {} Consommation {}", min_position, min_conso);

    min_conso as i64
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let positions = parse(input_file);

    let mut min = 99999;
    let mut max = -99999;
    positions.iter().for_each(|p| {
        if *p < min { min = *p }
        if *p > max { max = *p }
    });

    let mut min_conso = 999999999;
    let mut min_position = -1;
    for position in min..max {
        let mut conso = 0i32;
        positions.iter().for_each(|p| {
            let d = (position - *p).abs();
            conso += d*(d+1) / 2; // arithmetic sum
        });
        if conso < min_conso {
            min_conso = conso;
            min_position = position;
        }
    }

    println!("Position {} Consommation {}", min_position, min_conso);

    min_conso as i64
}
