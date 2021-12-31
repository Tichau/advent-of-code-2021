use std::fs::File;
use std::io;
use std::collections::HashSet;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> helpers::Map<char> {
    let inputs: Vec<Vec<char>> = helpers::parse_file_to_list(input_file, |line| {
        line.chars().collect()
    });

    let mut map: helpers::Map<char> = helpers::Map::new(inputs[0].len(), inputs.len());
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            let cell = map.get_mut(helpers::Position::new(x as i32, y as i32)).unwrap(); 
            *cell = *value;
        });
    });

    map
}

pub fn part1(input: &helpers::Map<char>) -> u32 {
    let mut map = input.clone();
    let mut cache: HashSet<helpers::Position> = HashSet::new();

    let mut step = 0;
    while do_step(&mut map, &mut cache) {
        step += 1;
    }
    
    step + 1
}

pub fn part2(_: &helpers::Map<char>) -> u32 {
    0
}

fn do_step(map: &mut helpers::Map<char>, cache: &mut HashSet<helpers::Position>) -> bool {
    let mut at_least_one_move = false;
    for i in 0..2 {
        let cucumber = if i == 0 { '>' } else { 'v' };

        cache.clear();
        for x in 0..map.width {
            for y in 0..map.height {
                let position = helpers::Position::new(x as i32, y as i32);
                let cell = map.get_mut(position).unwrap();
                if *cell == cucumber {
                    if *map.get(position.next(map, cucumber)).unwrap() == '.' {
                        cache.insert(position);
                    }
                }
            }
        }
    
        for position in cache.iter() {
            *map.get_mut(*position).unwrap() = '.';
            *map.get_mut(position.next(map, cucumber)).unwrap() = cucumber;
            at_least_one_move = true;
        }
    }

    at_least_one_move
}

impl helpers::Position {

    fn next(&self, map: &helpers::Map<char>, cucumber: char) -> helpers::Position {
        match cucumber {
            '>' => helpers::Position::new(((self.x as usize + 1) % map.width) as i32, self.y),
            'v' => helpers::Position::new(self.x, ((self.y as usize + 1) % map.height) as i32),
            _ => panic!("Unknown cucumber"),
        }
    }
}
