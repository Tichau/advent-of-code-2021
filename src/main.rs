use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1s1();
    day1s2();
}

fn day1s1() {
    let inputs: Vec<i32> = parse_file_to_list_of_int("day1.txt");

    let mut increased: i32 = 0;
    inputs.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && *x > inputs[i-1] {
            increased += 1;
        }
    });

    println!("Increased {} times", increased);
}

fn day1s2() {
    let inputs: Vec<i32> = parse_file_to_list_of_int("day1.txt");

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

    println!("Increased {} times", increased);
}

fn parse_file_to_list_of_int<P>(filename: P) -> Vec<i32>
where P: AsRef<Path>, {
    let mut inputs: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                inputs.push(ip.parse().unwrap());
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
