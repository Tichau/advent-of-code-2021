use std::fs::File;
use std::io;
use crate::helpers;

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let inputs: Vec<String> = helpers::parse_file_to_list(input_file, |line| { String::from(line) });

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

    println!("epsilon: {}, gamma: {}", epsilon, gamma);

    (gamma * epsilon) as i64
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let inputs: Vec<String> = helpers::parse_file_to_list(input_file, |line| { String::from(line) });

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

    println!("oxygen: {}, co2: {}", oxygen, co2);

    (oxygen * co2) as i64
}
