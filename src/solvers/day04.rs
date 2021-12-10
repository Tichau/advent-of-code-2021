use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub fn parser(input_file: io::BufReader<File>) -> (Vec<u8>, Vec<Grid>) {
    let regex = Regex::new(r"^\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*$").unwrap();
    let mut numbers: Vec<u8> = Vec::new();
    let mut grids: Vec<Grid> = Vec::new();
    
    {
        let mut grid: Grid = Grid::new();
        let mut row = 0;
        for line in input_file.lines() {
            if let Ok(ip) = line {
                if numbers.len() == 0 {
                    numbers = ip.split(',').map(|str| str.parse::<u8>().unwrap()).collect();  
                } else {
                    if let Option::Some(capture) = regex.captures(&ip) {
                        let mut col = 0;
                        for i in 1..capture.len() {
                            grid.grid[row][col].number = capture.get(i).unwrap().as_str().parse::<u8>().unwrap();
                            col += 1;
                        }

                        row += 1;
                        if row == 5 {
                            // New grid
                            grids.push(grid);
                            grid = Grid::new();
                            row = 0;
                        }
                    }
                }
            }
        }
    }

    (numbers, grids)
}

pub fn part1(input: &(Vec<u8>, Vec<Grid>)) -> i32 {
    let numbers = &input.0;
    let mut grids = input.1.clone();

    for &n in numbers {
        for i in (0..grids.len()).rev() {
            let grid = &mut grids[i];
            grid.mark(n);
            let (victory, unmarked_sum) = grid.check_victory();
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

pub fn part2(input: &(Vec<u8>, Vec<Grid>)) -> i32 {
    let numbers = &input.0;
    let mut grids = input.1.clone();

    let mut last_to_win = -1;
    for &n in numbers {
        for i in (0..grids.len()).rev() {
            let grid = &mut grids[i];
            grid.mark(n);
            let (victory, unmarked_sum) = grid.check_victory();
            if victory {
                last_to_win = n as i32 * unmarked_sum;
                grids.remove(i);
            }
        }
    }

    println!("Last grid to win: {}", last_to_win);
    last_to_win
}

#[derive(Copy, Clone)]
pub struct Grid {
    grid: [[ Cell; 5]; 5],
}

impl Grid {
    fn new() -> Self {
        Self {
          grid: [[Cell {number: 0, marked: false}; 5]; 5]
        }
    }

    fn mark(&mut self, n: u8) {
        self.grid.iter_mut().for_each(|line| line.iter_mut().for_each(|cell| if cell.number == n { cell.marked = true }));
    }

    fn check_victory(&self) -> (bool, i32) {
        let mut unmarked_sum = 0i32;
        let mut col_count = [0i32; 5];
        let mut win = false;
        for (_, line) in self.grid.iter().enumerate() {
            let mut row_count = 0;
            for (col, cell) in line.iter().enumerate() {
                if cell.marked { row_count += 1; col_count[col] += 1 } else { unmarked_sum += cell.number as i32 };
            }

            if row_count == 5 { win = true };
        }

        col_count.iter().for_each(|c| if *c == 5 { win = true });

        (win, unmarked_sum)
    }
}

#[derive(Copy, Clone)]
struct Cell {
    number: u8,
    marked: bool,
}
