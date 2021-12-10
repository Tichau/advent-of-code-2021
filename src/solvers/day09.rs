use std::fs::File;
use std::io;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Vec<Cell>> {
    helpers::parse_file_to_list(input_file, |line| {
        line.chars().map(|c| { 
            Cell { 
                elevation: c.to_digit(10).unwrap(), 
                bassin: -1 
            }
        }).collect()
    })
}

pub fn part1(input: &Vec<Vec<Cell>>) -> u32 {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let elevation = input[y][x].elevation;
            if  (x == 0 || elevation < input[y][x-1].elevation) && (x + 1 >= input[y].len() || elevation < input[y][x+1].elevation) && 
                (y == 0 || elevation < input[y-1][x].elevation) && (y + 1 >= input.len() || elevation < input[y+1][x].elevation) {
                count += elevation + 1;
            }
        }
    }

    count
}

pub fn part2(input: &Vec<Vec<Cell>>) -> i32 {
    let mut map = input.clone();

    let mut bassin_index = -1;
    let mut bassin_sizes = [0i32; 3];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].elevation < 9 && map[y][x].bassin < 0 {
                bassin_index += 1;
                let mut bassin_size = 0;
                grow(&mut map, x, y, bassin_index, &mut bassin_size);
                let smallest_index = if bassin_sizes[0] < bassin_sizes[1] && bassin_sizes[0] < bassin_sizes[2] { 0 } else if bassin_sizes[1] < bassin_sizes[2] { 1 } else { 2 };
                if bassin_size > bassin_sizes[smallest_index] {
                    bassin_sizes[smallest_index] = bassin_size
                }
            }
        }
    }

    bassin_sizes.iter().fold(1, |a, &b| a * b)
}

fn grow(map: &mut Vec<Vec<Cell>>, x: usize, y: usize, bassin: i32, bassin_size: &mut i32) {
    if map[y][x].bassin >= 0 || map[y][x].elevation == 9 {
        return;
    }

    map[y][x].bassin = bassin;
    *bassin_size += 1;

    if x > 0 { 
        grow(map, x-1, y, bassin, bassin_size); 
    } 
    if x < map[y].len() - 1 { 
        grow(map, x+1, y, bassin, bassin_size); 
    }
    if y > 0 { 
        grow(map, x, y-1, bassin, bassin_size); 
    }
    if y < map.len() - 1 { 
        grow(map, x, y+1, bassin, bassin_size); 
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    elevation: u32,
    bassin: i32,
}
