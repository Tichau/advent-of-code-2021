use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Deref, DerefMut};
use std::fmt::{Display, Formatter, Result};

pub fn parse_file_to_list<T>(file: io::BufReader<File>, parse_func: impl Fn(&str) -> T) -> Vec<T> {
    let mut inputs: Vec<T> = Vec::new();
    for line in file.lines() {
        if let Ok(ip) = line {
            inputs.push(parse_func(&ip));
        }
    }

    inputs
}

#[derive(Copy, Clone)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn neighbours(&self, diags: bool) -> IntoNeighbourIterator {
        if diags {
            IntoNeighbourIterator { pos: *self, index: 0 }
        } else {
            IntoNeighbourIterator { pos: *self, index: 4 }
        }
    }
}

impl PartialEq<Position> for Position {
    fn eq(&self, other: &Position) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct IntoNeighbourIterator {
    pos: Position,
    index: u8,
}

impl Iterator for IntoNeighbourIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        let neighbour = match self.index {
            0 => self.pos + Position { x: 1, y: -1 },
            1 => self.pos + Position { x: -1, y: -1 },
            2 => self.pos + Position { x: -1, y: 1 },
            3 => self.pos + Position { x: 1, y: 1 },
            4 => self.pos + Position { x: 1, y: 0 },
            5 => self.pos + Position { x: 0, y: -1 },
            6 => self.pos + Position { x: -1, y: 0 },
            7 => self.pos + Position { x: 0, y: 1 },
            _ => return None,
        };
        
        self.index += 1;
        Some(neighbour)
    }
}

/// Map

#[derive(Default)]
#[derive(Clone)]
pub struct Map<T> 
    where T: Clone
{
    pub width: usize,
    pub height: usize,
    map: Box<[T]>,
}

impl<T> Map<T>
    where T: Clone
{
    pub fn get(&self, pos: Position) -> Option<&T> {
        if pos.x < 0 || pos.x as usize >= self.width || pos.y < 0 || pos.y as usize >= self.height {
            return None
        } 

        Some(&self.map[pos.y as usize * self.width + pos.x as usize])
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        if pos.x < 0 || pos.x as usize >= self.width || pos.y < 0 || pos.y as usize >= self.height {
            return None
        } 

        Some(&mut self.map[pos.y as usize * self.width + pos.x as usize])
    }

    pub fn set(&mut self, pos: Position, value: T) {
        if pos.x < 0 || pos.x as usize >= self.width || pos.y < 0 || pos.y as usize >= self.height {
            return;
        } 

        self.map[pos.y as usize * self.width + pos.x as usize] = value;
    }
}

impl<T> Map<T>
    where T: Default + Clone
{
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            map: vec![Default::default(); width * height].into_boxed_slice(),
        }
    }

    pub fn new_init(width: usize, height: usize, default: T) -> Self {
        Map {
            width,
            height,
            map: vec![default; width * height].into_boxed_slice(),
        }
    }
}

impl<T> Display for Map<T>
    where T: Clone + Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Err(error) = write!(f, "{}", self.get(Position::new(x, y)).unwrap()) {
                    return Err(error);
                }
            }

            if let Err(error) = writeln!(f, "") {
                return Err(error);
            }
        }

        Ok(())
    }
}

impl<T> Deref for Map<T>
    where T: Clone
{
    type Target = [T];
    fn deref(&self) -> &Self::Target { 
        &self.map 
    }
}

impl<T> DerefMut for Map<T>
    where T: Clone
{
    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.map 
    }
}
