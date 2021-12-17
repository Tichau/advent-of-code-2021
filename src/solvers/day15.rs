use std::fs::File;
use std::io;
use std::collections::{BinaryHeap};
use std::cmp::Ordering;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> helpers::Map<Pos> {
    let inputs: Vec<Vec<char>> = helpers::parse_file_to_list(input_file, |line| {
        line.chars().collect()
    });

    let mut map: helpers::Map<Pos> = helpers::Map::new(inputs[0].len(), inputs.len());
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            let cell = map.get_mut(helpers::Position::new(x as i32, y as i32)).unwrap(); 
            *cell = Pos::new(value.to_digit(10).unwrap() as i32);
        });
    });

    map
}

pub fn part1(input: &helpers::Map<Pos>) -> i32 {
    let mut map = input.clone();
    astar(&mut map)
}

pub fn part2(input: &helpers::Map<Pos>) -> i32 {
    let mut map: helpers::Map<Pos> = helpers::Map::new(input.width * 5, input.height * 5);

    for x in 0..input.width {
        for y in 0..input.height {
            let value = input.get(helpers::Position::new(x as i32, y as i32)).unwrap();

            for x_offset in 0..5usize {
                for y_offset in 0..5usize {
                    let offset_value = (value.weight - 1 + x_offset as i32 + y_offset as i32) % 9 + 1;
                    let position = helpers::Position::new((x + x_offset * input.width) as i32, (y + y_offset * input.height) as i32);
                    map.set(position, Pos::new(offset_value));
                }
            }
        }
    }

    astar(&mut map)
}

fn heuristic(start: &helpers::Position, destination: &helpers::Position) -> i32 {
    (destination.x - start.x).abs() + (destination.y - start.y).abs()
}

pub fn astar(input: &mut helpers::Map<Pos>) -> i32 {
    let start_pos = helpers::Position::new(0,0);
    let destination_pos = helpers::Position::new((input.width - 1) as i32, (input.height - 1) as i32);
    input.get_mut(start_pos).unwrap().cost = 0;
    let mut open_set: BinaryHeap<OpenPos> = BinaryHeap::new();
    open_set.push(OpenPos::new(start_pos, heuristic(&start_pos, &destination_pos)));

    while let Some(node) = open_set.pop() {
        let node_cost = input.get(node.position).unwrap().cost;
        if node.position == destination_pos {
            return node_cost;
        }

        for n in node.position.neighbours(false) {
            if let Some(neighbour) = input.get_mut(n) {
                if neighbour.closed {
                    continue;
                }
                
                let cost = node_cost + neighbour.weight;
                if cost < neighbour.cost {
                    neighbour.cost = cost;
                    open_set.push(OpenPos::new(n, cost + heuristic(&n, &destination_pos)));
                }
            }
        }

        input.get_mut(node.position).unwrap().closed = true;
    }

    i32::MAX
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct Pos {
    weight: i32,
    cost: i32,
    closed: bool,
}

impl Pos {
    fn new(weight: i32) -> Pos {
        Pos {
            weight: weight,
            cost: i32::MAX,
            closed: false,
        }
    }
}

#[derive(Eq)]
struct OpenPos {
    position: helpers::Position,
    heuristic: i32,
}

impl OpenPos {
    fn new(pos: helpers::Position, heuristic: i32) -> Self {
        OpenPos {
            position: pos,
            heuristic: heuristic,
        }
    }
}

impl PartialEq for OpenPos {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Ord for OpenPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl PartialOrd for OpenPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
