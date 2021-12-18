use std::fs::File;
use std::ops::{Add, AddAssign};
use std::fmt::{Display, Formatter, Result};
use std::io;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Number> {
    let inputs: Vec<Number> = helpers::parse_file_to_list(input_file, |line| {
        let mut pairs: Vec<Pair> = Vec::new();
        let mut open_elements: Vec<usize> = Vec::new();
        let mut root_index: usize = usize::MAX;
        for c in line.chars() {
            match c {
                '[' => {
                    pairs.push(Pair::default());
                    open_elements.push(pairs.len() - 1);
                },
                '0'..='9' => {
                    let &pair_index = open_elements.last().unwrap();
                    let mut pair = &mut pairs[pair_index];
                    if pair.left == Element::None {
                        pair.left = Element::Value(u16::from_str_radix(&c.to_string(), 10).unwrap());
                    } else {
                        pair.right = Element::Value(u16::from_str_radix(&c.to_string(), 10).unwrap());
                    }
                }
                ',' => (),
                ']' => {
                    let finished_pair_index = open_elements.pop().unwrap();
                    if open_elements.len() == 0 {
                        root_index = finished_pair_index;
                    } else {
                        let &pair_index = open_elements.last().unwrap();
                        let mut pair = &mut pairs[pair_index];
                        if pair.left == Element::None {
                            pair.left = Element::PairRef(finished_pair_index);
                        } else {
                            pair.right = Element::PairRef(finished_pair_index);
                        }
                    }
                },
                _ => panic!("Unknown character"),
            }
        }

        assert!(open_elements.len() == 0);
        assert!(root_index != usize::MAX);
        Number::new(pairs, root_index)
    });

    inputs
}

pub fn part1(input: &Vec<Number>) -> i32 {
    let mut number = input[0].clone();

    for i in 1..input.len() {
        number += &input[i];
        number.reduce(false);
    }
    
    println!("{}", number);
    number.magnitude()
}

pub fn part2(input: &Vec<Number>) -> i32 {
    let mut max_magnitude = 0;
    for i1 in 0..input.len() {
        for i2 in 0..input.len() {
            let mut n1 = input[i1].clone();
            n1 += &input[i2];
            n1.reduce(false);
            if n1.magnitude() > max_magnitude {
                max_magnitude = n1.magnitude();
            }
        }
    }

    max_magnitude
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Element {
    None,
    Value(u16),
    PairRef(usize),
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Side {
    None,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Pair {
    left: Element,
    right: Element,
    parent: (usize, Side),
}

impl Pair {
    fn new(left: Element, right: Element) -> Self {
        Self {
            left,
            right,
            parent: (usize::MAX, Side::None),
        }
    }
}

impl Default for Pair {
    fn default() -> Self {
        Self {
            left: Element::None,
            right: Element::None,
            parent: (usize::MAX, Side::None),
        }
    }
}

#[derive(Clone)]
pub struct Number {
    pairs: Vec<Pair>,
    root: usize,
}

impl Number {
    fn new(pairs: Vec<Pair>, root: usize) -> Self {
        Self {
            pairs,
            root,
        }
    }

    fn refresh_parents(&mut self) {
        for pair_index in 0..self.pairs.len() {
            self.pairs[pair_index].parent = (usize::MAX, Side::None);
        }

        for pair_index in 0..self.pairs.len() {
            if let Element::PairRef(pair_ref) = self.pairs[pair_index].left {
                assert!(self.pairs[pair_ref].parent.0 == usize::MAX);
                self.pairs[pair_ref].parent = (pair_index, Side::Left);
            }
            if let Element::PairRef(pair_ref) = self.pairs[pair_index].right {
                assert!(self.pairs[pair_ref].parent.0 == usize::MAX);
                self.pairs[pair_ref].parent = (pair_index, Side::Right);
            }
        }
    }

    pub fn reduce(&mut self, verbose: bool) {
        self.refresh_parents();
        loop {
            if verbose {
                println!("  {}", self);
            }

            if self.explode_pair(self.root, 0) {
                // self.refresh_parents();
                continue;
            }

            if self.split_pair(self.root) {
                self.refresh_parents();
                continue;
            }

            break;
        }
    }

    fn explode_pair(&mut self, pair_index: usize, depth: u8) -> bool {
        if depth == 4 {
            // Explode
            let pair = self.pairs[pair_index];

            if let Element::Value(value) = pair.left {
                self.add_to_previous_value(pair_index, value);
            } else {
                panic!("Should be a value");
            }

            if let Element::Value(value) = pair.right {
                self.add_to_next_value(pair_index, value);
            } else {
                panic!("Should be a value");
            }
            
            match pair.parent.1 {
                Side::Left => self.pairs[pair.parent.0].left = Element::Value(0),
                Side::Right => self.pairs[pair.parent.0].right = Element::Value(0),
                _ => panic!("Invalid parent"),
            }

            self.pairs[pair_index] = Pair::default();
            return true;
        }

        if let Element::PairRef(child_index) = self.pairs[pair_index].left {
            if self.explode_pair(child_index, depth + 1) { return true; }
        } 
       
        if let Element::PairRef(child_index) = self.pairs[pair_index].right {
            if self.explode_pair(child_index, depth + 1) { return true; }
        } 

        false
    }

    fn split_pair(&mut self, pair_index: usize) -> bool {
        match self.pairs[pair_index].left {
            Element::Value(value) => {
                if value >= 10 {
                    self.pairs.push(Pair::new(Element::Value(value/2), Element::Value(value/2 + value%2)));
                    self.pairs[pair_index].left = Element::PairRef(self.pairs.len() - 1);
                    return true;
                }
            },
            Element::PairRef(pair_index) => {
                if self.split_pair(pair_index) { return true; }
            },
            _ => panic!()
        }

        match self.pairs[pair_index].right {
            Element::Value(value) => {
                if value >= 10 {
                    self.pairs.push(Pair::new(Element::Value(value/2), Element::Value(value/2 + value%2)));
                    self.pairs[pair_index].right = Element::PairRef(self.pairs.len() - 1);
                    return true;
                }
            },
            Element::PairRef(pair_index) => {
                if self.split_pair(pair_index) { return true; }
            },
            _ => panic!()
        }

        false
    }

    fn add_to_next_value(&mut self, pair_index: usize, value: u16) {
        let pair = self.pairs[pair_index];
        if pair.parent.0 == usize::MAX {
            return;
        }

        match pair.parent.1 {
            Side::Left => self.add_to_value((pair.parent.0, Side::Right), value, Side::Left),
            Side::Right => self.add_to_next_value(pair.parent.0, value),
            _ => panic!("Invalid side"),
        }
    }

    fn add_to_previous_value(&mut self, pair_index: usize, value: u16) {
        let pair = self.pairs[pair_index];
        if pair.parent.0 == usize::MAX {
            return;
        }

        match pair.parent.1 {
            Side::Left => self.add_to_previous_value(pair.parent.0, value),
            Side::Right => self.add_to_value((pair.parent.0, Side::Left), value, Side::Right),
            _ => panic!("Invalid side"),
        }
    }

    fn add_to_value(&mut self, element_ref: (usize, Side), value: u16, direction: Side) {
        match element_ref.1 {
            Side::Left => {
                match self.pairs[element_ref.0].left {
                    Element::Value(current) => self.pairs[element_ref.0].left = Element::Value(current + value),
                    Element::PairRef(pair_ref) => self.add_to_value((pair_ref, direction), value, direction),
                    _ => panic!("Invalid element"),
                }
            },
            Side::Right => {
                match self.pairs[element_ref.0].right {
                    Element::Value(current) => self.pairs[element_ref.0].right = Element::Value(current + value),
                    Element::PairRef(pair_ref) => self.add_to_value((pair_ref, direction), value, direction),
                    _ => panic!("Invalid element"),
                }
            },
            _ => panic!("Invalid side"),
        }
    }

    pub fn magnitude(&self) -> i32 {
        self.pair_magnitude(&self.pairs[self.root])
    }

    fn pair_magnitude(&self, pair: &Pair) -> i32 {
        let left = match pair.left {
            Element::None => panic!("Invalid element: None"),
            Element::Value(value) => value as i32,
            Element::PairRef(pair_index) => self.pair_magnitude(&self.pairs[pair_index]),
        };

        let right = match pair.right {
            Element::None => panic!("Invalid element: None"),
            Element::Value(value) => value as i32,
            Element::PairRef(pair_index) => self.pair_magnitude(&self.pairs[pair_index]),
        };

        3 * left + 2 * right
    }
    
    fn fmt_pair(&self, pair: &Pair, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        self.fmt_element(pair.left, f)?;
        write!(f, ",")?;
        self.fmt_element(pair.right, f)?;
        write!(f, "]")?;
        Ok(())
    }

    fn fmt_element(&self, element: Element, f: &mut Formatter<'_>) -> Result {
        match element {
            Element::None => write!(f, "None"),
            Element::Value(value) => write!(f, "{}", value),
            Element::PairRef(pair_index) => self.fmt_pair(&self.pairs[pair_index], f),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut pairs = self.pairs.clone();
        let right_offset = pairs.len();
        for mut pair in rhs.pairs {
            if let Element::PairRef(pair_id) = pair.left {
                pair.left = Element::PairRef(pair_id + right_offset);
            }
            if let Element::PairRef(pair_id) = pair.right {
                pair.right = Element::PairRef(pair_id + right_offset);
            }

            pairs.push(pair);
        }

        pairs.push(Pair::new(Element::PairRef(self.root), Element::PairRef(rhs.root + right_offset)));
        let root = pairs.len() - 1;

        Self {
            pairs,
            root,
        }
    }
}

impl AddAssign<&Self> for Number {
    fn add_assign(&mut self, rhs: &Self) {
        let right_offset = self.pairs.len();
        for pair in &rhs.pairs {
            let mut new_pair = pair.clone();
            if let Element::PairRef(pair_id) = pair.left {
                new_pair.left = Element::PairRef(pair_id + right_offset);
            }
            if let Element::PairRef(pair_id) = pair.right {
                new_pair.right = Element::PairRef(pair_id + right_offset);
            }

            self.pairs.push(new_pair);
        }

        self.pairs.push(Pair::new(Element::PairRef(self.root), Element::PairRef(rhs.root + right_offset)));
        self.root = self.pairs.len() - 1;
    }
}

impl Display for Number
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt_pair(&self.pairs[self.root], f)
    }
}
