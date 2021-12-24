use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};
use chrono::Datelike;

mod helpers;
mod solvers;
mod tests;

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

    let path = Path::new("data").join(if input_path.is_empty() { format!("day{:02}.txt", day) } else { input_path });

    println!("### Day {} ###", day);
    
    if let Some(mut solver) = get_solvers(day) {
        {
            println!("### Parsing input ###");
            println!("Reading: {}", path.display());
            let start = Instant::now();
            let file = io::BufReader::new(File::open(&path).expect("Failed to read file"));
            solver.parse(file);
            let duration = start.elapsed();
            println!("Took {}", fmt_dur(duration));
        }
        
        println!("");

        {
            println!("### Running Part 1 ###");
            let start = Instant::now();
            solver.part1();
            let duration = start.elapsed();
            println!("Took {}", fmt_dur(duration));
        }

        println!("");

        {
            println!("### Running Part 2 ###");
            let start = Instant::now();
            solver.part2();
            let duration = start.elapsed();
            println!("Took {}", fmt_dur(duration));
        }
    } else {
        println!("No solver found for day {}.", day);
    }
}

fn get_solvers(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(DaySolver::from(solvers::day01::parser, solvers::day01::part1, solvers::day01::part2))),
        2 => Some(Box::new(DaySolver::from(solvers::day02::parser, solvers::day02::part1, solvers::day02::part2))),
        3 => Some(Box::new(DaySolver::from(solvers::day03::parser, solvers::day03::part1, solvers::day03::part2))),
        4 => Some(Box::new(DaySolver::from(solvers::day04::parser, solvers::day04::part1, solvers::day04::part2))),
        5 => Some(Box::new(DaySolver::from(solvers::day05::parser, solvers::day05::part1, solvers::day05::part2))),
        6 => Some(Box::new(DaySolver::from(solvers::day06::parser, solvers::day06::part1, solvers::day06::part2))),
        7 => Some(Box::new(DaySolver::from(solvers::day07::parser, solvers::day07::part1, solvers::day07::part2))),
        8 => Some(Box::new(DaySolver::from(solvers::day08::parser, solvers::day08::part1, solvers::day08::part2))),
        9 => Some(Box::new(DaySolver::from(solvers::day09::parser, solvers::day09::part1, solvers::day09::part2))),
        10 => Some(Box::new(DaySolver::from(solvers::day10::parser, solvers::day10::part1, solvers::day10::part2))),
        11 => Some(Box::new(DaySolver::from(solvers::day11::parser, solvers::day11::part1, solvers::day11::part2))),
        12 => Some(Box::new(DaySolver::from(solvers::day12::parser, solvers::day12::part1, solvers::day12::part2))),
        13 => Some(Box::new(DaySolver::from(solvers::day13::parser, solvers::day13::part1, solvers::day13::part2))),
        14 => Some(Box::new(DaySolver::from(solvers::day14::parser, solvers::day14::part1, solvers::day14::part2))),
        15 => Some(Box::new(DaySolver::from(solvers::day15::parser, solvers::day15::part1, solvers::day15::part2))),
        16 => Some(Box::new(DaySolver::from(solvers::day16::parser, solvers::day16::part1, solvers::day16::part2))),
        17 => Some(Box::new(DaySolver::from(solvers::day17::parser, solvers::day17::part1, solvers::day17::part2))),
        18 => Some(Box::new(DaySolver::from(solvers::day18::parser, solvers::day18::part1, solvers::day18::part2))),
        19 => Some(Box::new(DaySolver::from(solvers::day19::parser, solvers::day19::part1, solvers::day19::part2))),
        20 => Some(Box::new(DaySolver::from(solvers::day20::parser, solvers::day20::part1, solvers::day20::part2))),
        21 => Some(Box::new(DaySolver::from(solvers::day21::parser, solvers::day21::part1, solvers::day21::part2))),
        _ => None,
    }
}

trait Solver {
    fn parse(&mut self, input_file: io::BufReader<File>);

    fn part1(&self);
    
    fn part2(&self);
}

pub struct DaySolver<T, R1, R2> {
    parser: fn(io::BufReader<File>) -> T,
    solver1: fn(&T) -> R1,
    solver2: fn(&T) -> R2,
    data: T,
}

impl<T, R1, R2> DaySolver<T, R1, R2> 
    where T: Default,
          R1: fmt::Display,
          R2: fmt::Display,
{
    fn from(parser: fn(io::BufReader<File>) -> T, solver1: fn(&T) -> R1, solver2: fn(&T) -> R2) -> DaySolver<T, R1, R2> {
        DaySolver {
            parser,
            solver1,
            solver2,
            data: Default::default(),
        }
    }

    pub fn solve_part1(&self) -> R1 {
        (self.solver1)(&self.data)
    }

    pub fn solve_part2(&self) -> R2 {
        (self.solver2)(&self.data)
    }
}

impl<T, R1, R2> Solver for DaySolver<T, R1, R2> 
    where R1: fmt::Display,
          R2: fmt::Display
{
    fn parse(&mut self, input_file: io::BufReader<File>) {
        self.data = (self.parser)(input_file);
    }

    fn part1(&self) {
        let result = (self.solver1)(&self.data);
        println!("Result: {}", result);
    }
    
    fn part2(&self) {
        let result = (self.solver2)(&self.data);
        println!("Result: {}", result);
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
