use std::fs::File;
use std::io;
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Line> {
    let regex = Regex::new(r"^([0-9]+),([0-9]+)\s->\s([0-9]+),([0-9]+)$").unwrap();
    helpers::parse_file_to_list(input_file, |line| { 
        let capture = regex.captures(line).unwrap();
        Line {
            p1: helpers::Position {
                x: capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            },
            p2: helpers::Position {
                x: capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                y: capture.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            },
        }
    })
}

pub fn part1(input: &Vec<Line>) -> i32 {
    let mut width = 0;
    let mut height = 0;
    input.iter().for_each(|line| {
        if line.p1.x > width { width = line.p1.x } else if line.p2.x > width { width = line.p2.x };
        if line.p1.y > height { height = line.p1.y } else if line.p2.y > height { height = line.p2.y };
    });
    
    let mut map: helpers::Map<i16> = helpers::Map::new((width + 1) as usize, (height + 1) as usize);
    for &line in input {
        if line.strait() {
            for p in line {
                *map.get_mut(p).unwrap() += 1;
            }
        }
    }
    
    map.iter().fold(0, |count, &cell| if cell >= 2 {count + 1} else { count })
}

pub fn part2(input: &Vec<Line>) -> i32 {
    let mut width = 0;
    let mut height = 0;
    input.iter().for_each(|line| {
        if line.p1.x > width { width = line.p1.x } else if line.p2.x > width { width = line.p2.x };
        if line.p1.y > height { height = line.p1.y } else if line.p2.y > height { height = line.p2.y };
    });
    
    let mut map: helpers::Map<i16> = helpers::Map::new((width + 1) as usize, (height + 1) as usize);
    for &line in input {
        for p in line {
            *map.get_mut(p).unwrap() += 1;
        }
    }
    
    map.iter().fold(0, |count, &cell| if cell >= 2 {count + 1} else { count })
}

#[derive(Copy, Clone)]
pub struct Line {
    p1: helpers::Position,
    p2: helpers::Position,
}

impl Line {
    fn strait(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}

impl PartialEq<Line> for Line {
    fn eq(&self, other: &Line) -> bool {
        return self.p1 == other.p1 && self.p2 == other.p2;
    }
}

pub struct LineIntoIterator {
    line: Line,
    index: helpers::Position,
}

impl IntoIterator for Line {
    type Item = helpers::Position;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            line: self,
            index: helpers::Position { x: -1, y: -1 },
        }
    }
}

impl Iterator for LineIntoIterator {
    type Item = helpers::Position;
    fn next(&mut self) -> Option<helpers::Position> {
        if self.index == self.line.p2 {
            return None;
        }

        if self.index.x == -1 && self.index.y == -1 {
            self.index = self.line.p1;
        } else {
            let inc = helpers::Position {
                x: (self.line.p2.x - self.index.x).signum(),
                y: (self.line.p2.y - self.index.y).signum(),
            };
            self.index = self.index + inc;
        }
        
        Some(self.index)
    }
}
