use std::fs::File;
use std::io;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> helpers::Map<i32> {
    let inputs: Vec<Vec<char>> = helpers::parse_file_to_list(input_file, |line| {
        line.chars().collect()
    });

    let mut map: helpers::Map<i32> = helpers::Map::new(inputs[0].len(), inputs.len());
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            let cell = map.get_mut(helpers::Position::new(x, y)).unwrap(); 
            *cell = value.to_digit(10).unwrap() as i32;
        });
    });

    map
}

pub fn part1(input: &helpers::Map<i32>) -> i32 {
    astar(input)
}

pub fn part2(input: &helpers::Map<i32>) -> i32 {
    let mut map: helpers::Map<i32> = helpers::Map::new(input.width * 5, input.height * 5);

    for x in 0..input.width {
        for y in 0..input.height {
            let &value = input.get(helpers::Position::new(x, y)).unwrap();

            for x_offset in 0..5usize {
                for y_offset in 0..5usize {
                    let offset_value = (value - 1 + x_offset as i32 + y_offset as i32) % 9 + 1;
                    
                    map.set(helpers::Position::new(x + x_offset * input.width, y + y_offset * input.height), offset_value);
                }
            }
        }
    }

    astar(&map)
}

fn heuristic(start: helpers::Position, destination: helpers::Position) -> i32 {
    (destination.x - start.x).abs() + (destination.y - start.y).abs()
}

pub fn astar(input: &helpers::Map<i32>) -> i32 {
    let start_pos = helpers::Position::new(0,0);
    let destination_pos = helpers::Position::new(input.width - 1, input.height - 1);
    let start = Pos::new(start_pos, 0, heuristic(start_pos, destination_pos));
    let mut close_set: Vec<Pos> = Vec::new();
    let mut open_set: BinaryHeap<Pos> = BinaryHeap::new();
    open_set.push(start);

    while let Some(node) = open_set.pop() {
        if node.position == destination_pos {
            return node.cost;
        }

        for n in node.position.neighbours(false) {
            if let Some(&transition_cost) = input.get(n) {
                let cost = node.cost + transition_cost;
                let neighbour = Pos::new(n, cost, cost + heuristic(n, destination_pos));
                if close_set.contains(&neighbour) {
                    continue;
                }

                let mut neighbour_cost = i32::MAX;
                if let Some(neighbour_pos) = open_set.iter().find(|&p| p.eq(&neighbour)) {
                    neighbour_cost = neighbour_pos.cost;
                }

                if neighbour.cost < neighbour_cost {
                    open_set.push(neighbour);
                }
            }
        }

        close_set.push(node);
    }

    0
}

#[derive(Eq)]
struct Pos {
    position: helpers::Position,
    cost: i32,
    heuristic: i32,
}

impl Pos {
    fn new(pos: helpers::Position, cost: i32, heuristic: i32) -> Pos {
        Pos {
            position: pos,
            cost: cost,
            heuristic: heuristic,
        }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
