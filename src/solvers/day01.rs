use std::fs::File;
use std::io;
use crate::helpers;

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let inputs: Vec<i32> = helpers::parse_file_to_list(input_file, |line| { line.parse().unwrap() });

    let mut increased: i64 = 0;
    inputs.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && *x > inputs[i-1] {
            increased += 1;
        }
    });

    increased
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let inputs: Vec<i32> = helpers::parse_file_to_list(input_file, |line| { line.parse().unwrap() });

    let mut windows: [(i32, i32); 3] = [(0, 0), (0, 0), (0, 0)];
    let mut windows_count: usize = 0;
    let mut current_window: usize = 0;

    let mut increased: i64 = 0;
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

    increased
}
