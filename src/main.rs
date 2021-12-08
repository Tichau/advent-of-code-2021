use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};
use chrono::Datelike;

mod helpers;
mod solvers;

pub type SolverFn = fn(io::BufReader<File>) -> i64;

fn main() {
    let mut day: u8 = 0;
    let mut input_path: String = String::new();
    
    {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            1 => (),
            2 => {
                if let Ok(number) = args[1].parse::<u8>() {
                    day = number;
                } else {
                    input_path = args[1].clone();
                }
            },
            3 => {
                if let Ok(number) = args[1].parse::<u8>() {
                    day = number;
                } else {
                    println!("usage: aoc2021 [day] [input-path]");
                }
    
                input_path = args[2].clone();
            }
            _ => {
                println!("usage: aoc2021 [day] [input-path]");
                return;
            }
        }
    }

    if day == 0 {
        day = chrono::Local::today().day() as u8;
    }

    println!("### Day {} ###", day);

    let path = Path::new("data").join(if input_path.is_empty() { format!("day{:02}.txt", day) } else { input_path });
    println!("Reading: {}", path.display());
    
    let (solver_part1, solver_part2) = get_solvers(day);

    println!("");

    {
        println!("### Running Part 1 ###");
        let file = io::BufReader::new(File::open(&path).expect("Failed to read file"));
        let start = Instant::now();
        let result = solver_part1(file);
        let duration = start.elapsed();
        println!("Took {}", fmt_dur(duration));
        println!("Result: {}", result);
    }

    println!("");

    {
        println!("### Running Part 2 ###");
        let file = io::BufReader::new(File::open(&path).expect("Failed to read file"));
        let start = Instant::now();
        let result = solver_part2(file);
        let duration = start.elapsed();
        println!("Took {}", fmt_dur(duration));
        println!("Result: {}", result);
    }
}

fn get_solvers(day: u8) -> (SolverFn, SolverFn) {
    match day {
        1 => (solvers::day01::part1, solvers::day01::part2),
        2 => (solvers::day02::part1, solvers::day02::part2),
        3 => (solvers::day03::part1, solvers::day03::part2),
        4 => (solvers::day04::part1, solvers::day04::part2),
        5 => (solvers::day05::part1, solvers::day05::part2),
        6 => (solvers::day06::part1, solvers::day06::part2),
        7 => (solvers::day07::part1, solvers::day07::part2),
        8 => (solvers::day08::part1, solvers::day08::part2),
        _ => (|_| { println!("Part1: No solver found"); 0 }, |_| { println!("Part2: No solver found"); 0 }),
    }
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}
