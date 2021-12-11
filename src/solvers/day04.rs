use std::fs::File;
use std::io::{self, BufRead};
use std::fmt::{Display, Formatter, Result};
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> (Vec<u8>, Vec<helpers::Map<Cell>>) {
    let regex = Regex::new(r"^\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*$").unwrap();
    let mut numbers: Vec<u8> = Vec::new();
    let mut grids: Vec<helpers::Map<Cell>> = Vec::new();
    
    {
        let mut grid: helpers::Map<Cell> = helpers::Map::new(5, 5);
        let mut y = 0;
        for line in input_file.lines() {
            if let Ok(ip) = line {
                if numbers.len() == 0 {
                    numbers = ip.split(',').map(|str| str.parse::<u8>().unwrap()).collect();  
                } else {
                    if let Option::Some(capture) = regex.captures(&ip) {
                        let mut x = 0;
                        for i in 1..capture.len() {
                            let cell = grid.get_mut(helpers::Position { x, y }).unwrap();
                            cell.number = capture.get(i).unwrap().as_str().parse::<u8>().unwrap();
                            x += 1;
                        }

                        y += 1;
                        if y == 5 {
                            // New grid
                            grids.push(grid);
                            grid = helpers::Map::new(5, 5);
                            y = 0;
                        }
                    }
                }
            }
        }
    }

    (numbers, grids)
}

pub fn part1(input: &(Vec<u8>, Vec<helpers::Map<Cell>>)) -> i32 {
    let numbers = &input.0;
    let mut maps = input.1.clone();

    for &n in numbers {
        for i in (0..maps.len()).rev() {
            let map = &mut maps[i];
            map.mark(n);
            let (victory, unmarked_sum) = map.check_victory();
            if victory {
                let first_to_win = n as i32 * unmarked_sum;
                println!("First grid to win: {}", first_to_win);
                return first_to_win;
            }
        }
    }

    println!("Can find any winning grid");
    -1
}

pub fn part2(input: &(Vec<u8>, Vec<helpers::Map<Cell>>)) -> i32 {
    let numbers = &input.0;
    let mut maps = input.1.clone();

    let mut last_to_win = -1;
    for &n in numbers {
        for i in (0..maps.len()).rev() {
            let map = &mut maps[i];
            map.mark(n);
            let (victory, unmarked_sum) = map.check_victory();
            if victory {
                last_to_win = n as i32 * unmarked_sum;
                maps.remove(i);
            }
        }
    }

    println!("Last grid to win: {}", last_to_win);
    last_to_win
}

impl helpers::Map<Cell> {
    fn mark(&mut self, n: u8) {
        self.iter_mut().for_each(|cell| if cell.number == n { cell.marked = true });
    }

    fn check_victory(&self) -> (bool, i32) {
        let mut unmarked_sum = 0i32;
        let mut col_count = [0i32; 5];
        let mut win = false;
        for x in 0usize..5 {
            let mut row_count = 0;
            for y in 0usize..5 {
                let cell = self.get(helpers::Position::new(x, y)).unwrap();
                if cell.marked { row_count += 1; col_count[y] += 1 } else { unmarked_sum += cell.number as i32 };
            }

            if row_count == 5 { win = true };
        }

        col_count.iter().for_each(|&c| if c == 5 { win = true });

        (win, unmarked_sum)
    }
}

#[derive(Copy, Clone)]
#[derive(Default)]
pub struct Cell {
    number: u8,
    marked: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}{}", self.number, if self.marked { "x" } else { "." })
    }
}
