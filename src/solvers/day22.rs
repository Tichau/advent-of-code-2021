use std::fs::File;
use std::io;
use std::collections::HashMap;
use regex::Regex;
use crate::helpers::parse_file_to_list;

const SIZE: usize = 101;

pub fn parser(input_file: io::BufReader<File>) -> Vec<Instruction> {
    let regex = Regex::new(r"^(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)$").unwrap();
    let inputs: Vec<Instruction> = parse_file_to_list(input_file, |line| {
        if let Option::Some(capture) = regex.captures(line) {
            Instruction {
                zone: Zone {
                    min: [capture.get(2).unwrap().as_str().parse::<i32>().unwrap(), capture.get(4).unwrap().as_str().parse::<i32>().unwrap(), capture.get(6).unwrap().as_str().parse::<i32>().unwrap()],
                    max: [capture.get(3).unwrap().as_str().parse::<i32>().unwrap(), capture.get(5).unwrap().as_str().parse::<i32>().unwrap(), capture.get(7).unwrap().as_str().parse::<i32>().unwrap()],
                },
                state: capture.get(1).unwrap().as_str() == "on",
            }
        } else {
            panic!("");
        }
    });

    inputs
}

pub fn part1(input: &Vec<Instruction>) -> i32 {
    let mut reactor = vec![vec![vec![false; SIZE]; SIZE]; SIZE];
    
    for instruction in input {
        instruction.apply(&mut reactor);
    }
    
    reactor.iter().fold(0, |sum1, grid| sum1 + grid.iter().fold(0, |sum2, line| sum2 + line.iter().fold(0, |sum3, &cell| sum3 + if cell { 1 } else { 0 })))
}

pub fn part2(input: &Vec<Instruction>) -> i64 {
    let all = Zone {
        min: [i32::MIN, i32::MIN, i32::MIN],
        max: [i32::MAX, i32::MAX, i32::MAX],
    };

    let mut cache: HashMap<Id, i64> = HashMap::new();

    count_in_zone(&input.as_slice(), &all, &mut cache)
}

fn count_in_zone(instructions: &[Instruction], zone: &Zone, cache: &mut HashMap<Id, i64>) -> i64 {
    if instructions.len() == 0 {
        return 0;
    }
    
    let id = Id {
        index: instructions.len() - 1,
        zone: zone.clone(),
    };

    let mut count = 0;
    if instructions.len() > 1 {
        if let Option::Some(&value) = cache.get(&id) {
            count = value;
        } else {
            count = count_in_zone(&instructions[0..id.index], zone, cache);
            cache.insert(id, count);
        }
    }
    
    let instruction = &instructions[id.index];
    let intersection = instruction.zone.intersect(zone);
    if !intersection.is_empty() {
        let intersection_id = Id {
            index: id.index,
            zone: intersection.clone(),
        };

        let mut count_in_intersection = 0;
        if instructions.len() > 1 {
            if let Option::Some(&value) = cache.get(&intersection_id) {
                count_in_intersection = value;
            } else {
                count_in_intersection = count_in_zone(&instructions[0..id.index], &intersection, cache);
                cache.insert(intersection_id, count_in_intersection);
            }
        }
        
        if instruction.state {
            count += intersection.area_size() - count_in_intersection;
        } else {
            count -= count_in_intersection;
        }
    }

    count
}

#[derive(Copy, Clone)]
#[derive(Eq)]
#[derive(Hash)]
struct Id {
    index: usize,
    zone: Zone,
}

impl PartialEq<Self> for Id {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.zone == other.zone
    }
}

#[derive(Copy, Clone)]
#[derive(Eq)]
#[derive(Hash)]
struct Zone {
    min: [i32; 3],
    max: [i32; 3],
}

impl PartialEq<Self> for Zone {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if self.min[i] != other.min[i] || self.max[i] != other.max[i] {
                return false;
            }
        }

        true
    }
}

impl Zone {
    fn get_range(&self, axis: usize) -> std::ops::Range<usize> {
        let size = SIZE as i32;
        let mut min = self.min[axis] + size/2;
        if min < 0 { min = 0 }
        else if min >= size { return 0..0 }

        let mut max = self.max[axis] + size/2 + 1;
        if max < 0 { return 0..0 }
        else if max > size { max = size }

        min as usize..max as usize
    }

    fn area_size(&self) -> i64 {
        let sx = (self.max[0] - self.min[0] + 1) as i64;
        let sy = (self.max[1] - self.min[1] + 1) as i64;
        let sz = (self.max[2] - self.min[2] + 1) as i64;
        sx * sy * sz
    }

    fn intersect(&self, other: &Self) -> Self {
        Self {
            min: [
                self.min[0].max(other.min[0]),
                self.min[1].max(other.min[1]),
                self.min[2].max(other.min[2]),
            ],
            max: [
                self.max[0].min(other.max[0]),
                self.max[1].min(other.max[1]),
                self.max[2].min(other.max[2]),
            ],
        }
    }

    fn is_empty(&self) -> bool {
        self.max[0] < self.min[0] || self.max[1] < self.min[1] || self.max[2] < self.min[2]
    }
}

pub struct Instruction {
    zone: Zone,
    state: bool,
}

impl Instruction {
    fn apply(&self, reactor: &mut Vec<Vec<Vec<bool>>>) {
        for x in self.zone.get_range(0) {
            let grid = &mut reactor[x as usize];
            for y in self.zone.get_range(1) {
                let line = &mut grid[y as usize];
                for z in self.zone.get_range(2) {
                    line[z as usize] = self.state; 
                }
            }
        }
    }
}
