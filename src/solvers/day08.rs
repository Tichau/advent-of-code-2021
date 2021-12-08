use std::fs::File;
use std::collections::HashMap;
use std::io;
use std::panic;
use crate::helpers;

fn parse(input_file: io::BufReader<File>) -> Vec<Entry> {
    let inputs = helpers::parse_file_to_list(input_file, |line| {
        let mut parts = line.split(" | ");
        let signals = parts.next().unwrap().split(" ").map(|signal| Digit::from(signal)).collect();
        let outputs = parts.next().unwrap().split(" ").map(|output| Digit::from(output)).collect();
        Entry {
            signals,
            outputs,
        }

    });

    inputs
}

pub fn part1(input_file: io::BufReader<File>) -> i64 {
    let inputs = parse(input_file);

    let mut count = 0i64;
    for entry in inputs {
        for output in entry.outputs {
            match output.string.len() {
                2 | 4 | 3 | 7 => count += 1, // 1, 4, 7, 8
                _ => (),
            }
        }
    }

    count
}

pub fn part2(input_file: io::BufReader<File>) -> i64 {
    let inputs = parse(input_file);
    
    let mut count = 0i64;
    for mut entry in inputs {
        let mut digits: [Digit; 10] = Default::default();
        for i in (0..entry.signals.len()).rev() {
            match &entry.signals[i].string.len() {
                2 => digits[1] = entry.signals.remove(i),
                3 => digits[7] = entry.signals.remove(i),
                4 => digits[4] = entry.signals.remove(i),
                7 => digits[8] = entry.signals.remove(i),
                _ => (),
            }
        }

        for i in (0..entry.signals.len()).rev() {
            match &entry.signals[i].string.len() {
                5 => {
                    if entry.signals[i].contains(digits[1].hash) {
                        digits[3] = entry.signals.remove(i);
                    } else if entry.signals[i].contains(digits[4].hash - digits[1].hash) {
                        digits[5] = entry.signals.remove(i);
                    } else {
                        digits[2] = entry.signals.remove(i);
                    }
                }
                6 => {
                    if entry.signals[i].contains(digits[4].hash) {
                        digits[9] = entry.signals.remove(i);
                    } else if entry.signals[i].contains(digits[1].hash) {
                        digits[0] = entry.signals.remove(i);
                    } else {
                        digits[6] = entry.signals.remove(i);
                    }
                }
                _ => (),
            }
        }

        let mut digits_hash: HashMap<u32,u32> = HashMap::new();
        for n in 0..digits.len() {
            if digits[n].string.len() == 0 {
                panic!("Digit {} not found", n);
            }

            digits_hash.insert(digits[n].hash, n as u32);
        }

        let mut number = 0;
        for i in 0..entry.outputs.len() {
            number += digits_hash[&entry.outputs[i].hash] * u32::pow(10, (entry.outputs.len() - i - 1) as u32);
        }

        count += number as i64;
    }

    count
}

struct Entry {
    signals: Vec<Digit>,
    outputs: Vec<Digit>,
}

struct Digit {
    string: String,
    hash: u32,
}

impl Default for Digit {
    fn default() -> Self {
        Self { string: Default::default(), hash: Default::default() }
    }
}

impl Digit {
    pub fn from(s: &str) -> Self {
        Digit {
            string: String::from(s),
            hash: Digit::to_flag(s),
        }
    }

    pub fn contains(&self, digit_hash: u32) -> bool {
        (self.hash & digit_hash) == digit_hash
    }

    fn to_flag(digit: &str) -> u32 {
        let mut result: u32 = 0;
        for c in digit.chars() {
            match c {
                'a' => result = result | (1u32 << 0),
                'b' => result = result | (1u32 << 1),
                'c' => result = result | (1u32 << 2),
                'd' => result = result | (1u32 << 3),
                'e' => result = result | (1u32 << 4),
                'f' => result = result | (1u32 << 5),
                'g' => result = result | (1u32 << 6),
                _ => (),
            }
        }
    
        result
    }
}
