use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use regex::Regex;
use std::fmt::{Display, Formatter, Result};

pub fn parser(input_file: io::BufReader<File>) -> Level {
    let regex = Regex::new(r"#([A-D])#([A-D])#([A-D])#([A-D])#").unwrap();
    let mut level = Level {
        hallway: ['.'; 11],
        rooms: [['.'; 4]; 4],
        room_size: 2,
    };
    
    let mut room_index = 0;
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if let Option::Some(capture) = regex.captures(&ip) {
                level.rooms[0][room_index] = capture.get(1).unwrap().as_str().chars().nth(0).unwrap();
                level.rooms[1][room_index] = capture.get(2).unwrap().as_str().chars().nth(0).unwrap();
                level.rooms[2][room_index] = capture.get(3).unwrap().as_str().chars().nth(0).unwrap();
                level.rooms[3][room_index] = capture.get(4).unwrap().as_str().chars().nth(0).unwrap();
                room_index += 1;
            }
        }
    }

    level
}

pub fn part1(input: &Level) -> u32 {
    search(input)
}

pub fn part2(input: &Level) -> u32 {
    let mut real_input = input.clone();
    real_input.room_size = 4;

    for room_index in 0..4 {
        real_input.rooms[room_index][3] = real_input.rooms[room_index][1];
    }

    real_input.rooms[0][1] = 'D';
    real_input.rooms[1][1] = 'C';
    real_input.rooms[2][1] = 'B';
    real_input.rooms[3][1] = 'A';

    real_input.rooms[0][2] = 'D';
    real_input.rooms[1][2] = 'B';
    real_input.rooms[2][2] = 'A';
    real_input.rooms[3][2] = 'C';

    search(&real_input)
}

fn search(level: &Level) -> u32 {
    let start_pos = Pos::new(level.clone(), 0);
    
    let mut close_set: HashSet<Pos> = HashSet::new();
    let mut open_set: BinaryHeap<Pos> = BinaryHeap::new();
    open_set.push(start_pos);

    while let Some(node) = open_set.pop() {
        if node.level.victory() {
            return node.cost;
        }

        for m in node.available_moves() {
            if close_set.contains(&m) {
                continue;
            }

            open_set.push(m);
        }

        close_set.insert(node);
    }

    u32::MAX
}

#[derive(Eq)]
#[derive(Copy, Clone)]
#[derive(Hash)]
struct Pos {
    level: Level,
    cost: u32,
    heuristic: u32,
}

impl Pos {
    fn new(level: Level, cost: u32) -> Self {
        Pos{ 
            level, 
            cost, 
            heuristic: level.heuristic_to_victory(),
        }
    }

    pub fn available_moves(&self) -> IntoMoveIterator {
        IntoMoveIterator { starting_pos: *self, room_index: 0, hallway_index: 0 }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost + self.heuristic).cmp(&(other.cost + other.heuristic)).reverse()
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct IntoMoveIterator {
    starting_pos: Pos,
    room_index: usize,
    hallway_index: usize,
}

impl Iterator for IntoMoveIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Pos> {
        let hallway_indexes = [0, 1, 3, 5, 7, 9, 10];
        let mut next = None;

        // From rooms to hallway
        while next == None {
            if self.room_index < 4 {
                let mut subroom_index = usize::MAX;
                let mut any_stranger = false;
                for i in 0..self.starting_pos.level.room_size {
                    if subroom_index == usize::MAX && self.starting_pos.level.rooms[self.room_index][i] != '.' {
                        subroom_index = i;
                    }

                    if self.starting_pos.level.rooms[self.room_index][i] != '.' && self.starting_pos.level.rooms[self.room_index][i] != amphipod(self.room_index) {
                        any_stranger = true;
                        break;
                    }
                }

                if any_stranger && subroom_index < self.starting_pos.level.room_size {
                    let pod = self.starting_pos.level.rooms[self.room_index][subroom_index];
                    assert!(self.hallway_index < hallway_indexes.len());
                    let path_length = self.starting_pos.level.path_length(self.room_index, subroom_index, hallway_indexes[self.hallway_index], true, false);
                    if path_length >= 0 {
                        let cost = (path_length * cost(pod)) as u32;
                        let mut new_level = self.starting_pos.level.clone();
                        new_level.hallway[hallway_indexes[self.hallway_index]] = pod;
                        new_level.rooms[self.room_index][subroom_index] = '.';
                        next = Some(Pos::new(new_level, self.starting_pos.cost + cost));
                    }
                }
            } else {
                break;
            }

            self.hallway_index += 1;
            if self.hallway_index >= hallway_indexes.len() {
                self.hallway_index = 0;
                self.room_index += 1;
            }
        }

        if next != None {
            return next;
        }

        // From hallway to rooms
        while next == None && self.hallway_index < hallway_indexes.len() {
            let pod = self.starting_pos.level.hallway[hallway_indexes[self.hallway_index]];
            if pod != '.' {
                let room_index = room(pod);

                let mut subroom_index = usize::MAX;
                for i in 0..self.starting_pos.level.room_size {
                    if self.starting_pos.level.rooms[room_index][i] == '.' {
                        subroom_index = i;
                    } else if self.starting_pos.level.rooms[room_index][i] != amphipod(room_index) {
                        subroom_index = usize::MAX;
                        break;
                    }
                }

                if subroom_index < self.starting_pos.level.room_size {
                    assert!(self.starting_pos.level.rooms[room_index][subroom_index] == '.');
                    let path_length = self.starting_pos.level.path_length(room_index, subroom_index, hallway_indexes[self.hallway_index], true, true);
                    if path_length >= 0 {
                        let cost = (path_length * cost(pod)) as u32;
                        let mut new_level = self.starting_pos.level.clone();
                        new_level.rooms[room_index][subroom_index] = pod;
                        new_level.hallway[hallway_indexes[self.hallway_index]] = '.';
                        next = Some(Pos::new(new_level, self.starting_pos.cost + cost));
                    }
                }
            }

            self.hallway_index += 1;
        }

        return next;
    }
}

fn cost(amphipod: char) -> i32 {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Unknown amphipod"),
    }
}

fn room(amphipod: char) -> usize {
    match amphipod {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Unknown amphipod"),
    }
}

fn amphipod(room: usize) -> char {
    match room {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!("Unknown room"),
    }
}

#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Level {
    hallway: [char; 11],
    rooms: [[char; 4]; 4],
    room_size: usize,
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}

impl Level {
    fn victory(&self) -> bool {
        for room_index in 0..4 {
            for subroom_index in 0..self.room_size {
                if self.rooms[room_index][subroom_index] != amphipod(room_index) {
                    return false;
                }
            }
        }
        
        true
    }

    fn heuristic_to_victory(&self) -> u32 {
        let mut heuristic = 0;
        for hallway_index in 0..self.hallway.len() {
            let pod = self.hallway[hallway_index];
            if pod != '.' {
                let move_cost = self.path_length(room(pod), 0, hallway_index, false, false);
                heuristic += move_cost * cost(pod);
            }
        }

        for room_index in 0..4 {
            for subroom_index in 0..self.room_size {
                let pod = self.rooms[room_index][subroom_index];
                if pod != '.' && pod != amphipod(room_index) {
                    let move_cost = self.path_length(room_index, subroom_index, room(pod) * 2 + 2, false, false) + 1;
                    heuristic += move_cost * cost(pod);
                }
            }
        }

        heuristic as u32
    }

    fn path_length(&self, room_index: usize, subroom_index: usize, hallway_index: usize, check_collision: bool, ignore_collision_for_hallway_index: bool) -> i32 {
        let room_hallway_index = (room_index * 2) + 2;
        if check_collision {
            for i in (0..subroom_index).rev() {
                if self.rooms[room_index][i] != '.' {
                    return -1;
                }
            }

            if room_hallway_index <= hallway_index {
                for index in room_hallway_index..hallway_index+1 {
                    if ignore_collision_for_hallway_index && index == hallway_index {
                        continue;
                    }

                    if self.hallway[index] != '.' {
                        return -1;
                    }
                }
            } else {
                for index in hallway_index..room_hallway_index+1 {
                    if ignore_collision_for_hallway_index && index == hallway_index {
                        continue;
                    }

                    if self.hallway[index] != '.' {
                        return -1;
                    }
                }
            }
        }

        let length: i32 = hallway_index as i32 - room_hallway_index as i32;
        length.abs() + (subroom_index + 1) as i32
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for c in self.hallway {
            write!(f, "{}", c)?;
        }
        writeln!(f, "#")?;

        writeln!(f, "###{}#{}#{}#{}###", self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0])?;
        for subroom_index in 1..self.room_size {
            writeln!(f, "  #{}#{}#{}#{}#  ", self.rooms[0][subroom_index], self.rooms[1][subroom_index], self.rooms[2][subroom_index], self.rooms[3][subroom_index])?;
        }
        writeln!(f, "  #########  ")?;

        Ok(())
    }
}
