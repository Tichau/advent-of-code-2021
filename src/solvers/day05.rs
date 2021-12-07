use std::fs::File;
use std::io;
use regex::Regex;
use crate::helpers;

fn parse(input_file: io::BufReader<File>) -> Vec<Line> {
    let regex = Regex::new(r"^([0-9]+),([0-9]+)\s->\s([0-9]+),([0-9]+)$").unwrap();
    helpers::parse_file_to_list(input_file, |line| { 
        let capture = regex.captures(line).unwrap();
        Line {
            p1: Point {
                x: capture.get(1).unwrap().as_str().parse::<i16>().unwrap(),
                y: capture.get(2).unwrap().as_str().parse::<i16>().unwrap(),
            },
            p2: Point {
                x: capture.get(3).unwrap().as_str().parse::<i16>().unwrap(),
                y: capture.get(4).unwrap().as_str().parse::<i16>().unwrap(),
            },
        }
    })
}

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let inputs = parse(input_file);

    let mut width = 0i16;
    let mut height = 0i16;
    inputs.iter().for_each(|line| {
        if line.p1.x > width { width = line.p1.x } else if line.p2.x > width { width = line.p2.x };
        if line.p1.y > height { height = line.p1.y } else if line.p2.y > height { height = line.p2.y };
    });
    
    let mut map = Map::new((width + 1) as usize, (height + 1) as usize);
    for line in inputs {
        if line.strait() {
            for p in line {
                map.increment(p);
            }
        }
    }
    
    let danger_position = map.danger() as i64;
    println!("{} dangerous positions", danger_position);

    danger_position
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let inputs = parse(input_file);

    let mut width = 0i16;
    let mut height = 0i16;
    inputs.iter().for_each(|line| {
        if line.p1.x > width { width = line.p1.x } else if line.p2.x > width { width = line.p2.x };
        if line.p1.y > height { height = line.p1.y } else if line.p2.y > height { height = line.p2.y };
    });
    
    let mut map = Map::new((width + 1) as usize, (height + 1) as usize);
    for line in inputs {
        for p in line {
            map.increment(p);
        }
    }
    
    let danger_position = map.danger() as i64;
    println!("{} dangerous positions", danger_position);

    danger_position
}

struct Map {
    width: usize,
    height: usize,
    map: Box<[i16]>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        println!("Instantiate map of size {}x{}", width, height);
        Map {
            height,
            width,
            map: vec![0i16; width * height].into_boxed_slice(),
        }
    }

    fn increment(&mut self, pos: Point) {
        self.map[pos.x as usize * self.width + pos.y as usize] += 1;
    }

    fn danger(&self) -> i32 {
        let mut count = 0;
        self.map.iter().for_each(|&cell| if cell >= 2i16 {count+=1});
        count
    }
}

struct Line {
    p1: Point,
    p2: Point,
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
    index: Point,
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            line: self,
            index: Point { x: -1, y: -1 },
        }
    }
}

impl Iterator for LineIntoIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.index == self.line.p2 {
            return None;
        }

        if self.index.x == -1 && self.index.y == -1 {
            self.index = self.line.p1;
        } else {
            let inc = Point {
                x: (self.line.p2.x - self.index.x).signum(),
                y: (self.line.p2.y - self.index.y).signum(),
            };
            self.index = self.index + inc;
        }
        
        Some(self.index)
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    x: i16,
    y: i16,
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
