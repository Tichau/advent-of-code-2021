use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_file_to_list<T>(file: io::BufReader<File>, parse_func: impl Fn(&str) -> T) -> Vec<T> {
    let mut inputs: Vec<T> = Vec::new();
    for line in file.lines() {
        if let Ok(ip) = line {
            inputs.push(parse_func(&ip));
        }
    }

    inputs
}
