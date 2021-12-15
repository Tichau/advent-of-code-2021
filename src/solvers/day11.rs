use std::fs::File;
use std::io;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> helpers::Map<EnergyLevel> {
    let inputs: Vec<Vec<char>> = helpers::parse_file_to_list(input_file, |line| {
        line.chars().collect()
    });

    let mut map: helpers::Map<EnergyLevel> = helpers::Map::new(inputs[0].len(), inputs.len());
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            let cell = map.get_mut(helpers::Position::new(x, y)).unwrap(); 
            *cell = EnergyLevel::Charge(value.to_digit(10).unwrap() as i32);
        });
    });

    map
}

pub fn part1(input: &helpers::Map<EnergyLevel>) -> i32 {
    let mut map = input.clone();
    let mut score = 0;

    for _ in 0..100 {
        for x in 0..map.width {
            for y in 0..map.height {
                let pos = helpers::Position::new(x, y);
                score += map.increase_energy_level(pos);
            }
        }
    
        map.reset_energy_levels();
    }

    score
}

pub fn part2(input: &helpers::Map<EnergyLevel>) -> i32 {
    let mut map = input.clone();
    let mut step = 1;
    loop {
        let mut flashes = 0;
        for x in 0..map.width {
            for y in 0..map.height {
                let pos = helpers::Position::new(x, y);
                flashes += map.increase_energy_level(pos);
            }
        }

        if flashes == (map.width * map.height) as i32 {
            break;
        }
    
        map.reset_energy_levels();
        step += 1;
    }

    step
}

impl helpers::Map<EnergyLevel> {
    fn increase_energy_level(&mut self, pos: helpers::Position) -> i32 {
        if let Some(cell) = self.get_mut(pos) {
            match *cell {
                EnergyLevel::Flash => (),
                EnergyLevel::Charge(energy) => {
                    if energy >= 9 {
                        *cell = EnergyLevel::Flash;
                        let mut flashes = 1;
                        for npos in pos.neighbours(true) {
                            flashes += self.increase_energy_level(npos);
                        }

                        return flashes;
                    } else {
                        *cell = EnergyLevel::Charge(energy + 1);
                    }
                }
            }
        }

        0
    }

    fn reset_energy_levels(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_mut(helpers::Position::new(x, y)).unwrap(); 
                if let EnergyLevel::Flash = cell { *cell = EnergyLevel::Charge(0) }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum EnergyLevel {
    Charge(i32),
    Flash,
}

impl Default for EnergyLevel {
    fn default() -> EnergyLevel {
        EnergyLevel::Charge(0)
    }
}
