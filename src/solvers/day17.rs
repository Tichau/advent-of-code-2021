use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> TargetZone {
    let regex = Regex::new(r"^target\sarea: x=(-?[0-9]+)\.\.(-?[0-9]+),\sy=(-?[0-9]+)\.\.(-?[0-9]+)$").unwrap();
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if let Option::Some(capture) = regex.captures(&ip) {
                let x_min = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let x_max = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let y_min = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let y_max = capture.get(4).unwrap().as_str().parse::<i32>().unwrap();
                return TargetZone::new(x_min, x_max, y_min, y_max)
            }
        }
    }

    panic!("No input found")
}

pub fn part1(input: &TargetZone) -> i32 {
    let mut y_max = 0;
    let mut v_max = helpers::Position::default();
    for vx in 1..input.x_max+1 {
        for vy in input.y_min..100 {
            let start_velocity = helpers::Position::new(vx, vy);
            let (hit_target, max_y) = evoluate(input, &start_velocity);
            if hit_target && max_y > y_max {
                y_max = max_y;
                v_max = start_velocity;
            }
        }
    }

    println!("velocity: {}", v_max);
    y_max
}

pub fn part2(input: &TargetZone) -> i32 {
    let mut count = 0;
    for vx in 1..input.x_max+1 {
        for vy in input.y_min..100 {
            let start_velocity = helpers::Position::new(vx, vy);
            let (hit_target, _) = evoluate(input, &start_velocity);
            if hit_target {
                count += 1;
            }
        }
    }

    count
}

fn evoluate(target: &TargetZone, start_velocity: &helpers::Position) -> (bool, i32) {
    let mut probe = helpers::Position::new(0, 0);
    let mut max_y = 0;
    let mut hit_target = false;
    let mut velocity = start_velocity.clone();
    while probe.x <= target.x_max && probe.y >= target.y_min {
        probe = probe + velocity;
        if velocity.x > 0 { velocity.x -= 1 } else if velocity.x < 0 { velocity.x += 1 } 
        velocity.y -= 1;

        if probe.y > max_y { max_y = probe.y }

        if target.contains(&probe) {
            hit_target = true;
            break;
        }
    }
    
    (hit_target, max_y)
}

#[derive(Default)]
pub struct TargetZone {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetZone {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn contains(&self, position: &helpers::Position) -> bool {
        position.x >= self.x_min && position.x <= self.x_max && position.y >= self.y_min && position.y <= self.y_max
    }
}
