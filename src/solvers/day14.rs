use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

pub fn parser(input_file: io::BufReader<File>) -> (String, Vec<InsertionRule>) {
    let regex = Regex::new(r"^([A-Z]+)\s->\s([A-Z])$").unwrap();
    let mut instructions: Vec<InsertionRule> = Vec::new();
    let mut template: String = String::new();
    let mut template_read = false;
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if ip == "" {
                template_read = true;
                continue;
            }

            if template_read {
                if let Option::Some(capture) = regex.captures(&ip) {
                    let mut from = capture.get(1).unwrap().as_str().chars();
                    let mut to = capture.get(2).unwrap().as_str().chars();
                    instructions.push(InsertionRule {
                        start: from.next().unwrap(),
                        end: from.next().unwrap(),
                        result: to.next().unwrap(),
                    });
                }
            } else {
                template = String::from(ip);
            }
        }
    }

    (template, instructions)
}

pub fn part1(input: &(String, Vec<InsertionRule>)) -> i32 {
    let mut template = input.0.clone();
    let rules = &input.1;
    
    for _ in 0..10 {
        let mut new_template = String::new();
        let mut last_char: char = 0 as char;
    
        for c in template.chars() {
            if last_char != 0 as char {
                for rule in rules {
                    // println!("{}{} -> {}", rule.start, rule.end, rule.result);
                    if rule.start == last_char && rule.end == c {
                        new_template.push(rule.result);
                        break;
                    }
                }
            }
    
            new_template.push(c);
            last_char = c;
        }
        
        template = new_template;
    }

    let mut count_per_char: HashMap<char, i32> = HashMap::new();
    template.chars().for_each(|c| {
        let count = count_per_char.entry(c).or_insert(0);
        *count += 1;
    });

    let mut max = 0;
    let mut min = i32::MAX;
    for (_, &count) in count_per_char.iter() {
        if count > max { max = count }
        if count < min { min = count }
    }

    max - min
}

pub fn part2(input: &(String, Vec<InsertionRule>)) -> u64 {
    let template = input.0.clone();
    let rules = &input.1;

    // prepare
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    let mut last_char: char = 0 as char;
    for c in template.chars() {
        if last_char != 0 as char {
            let e = pairs.entry((last_char, c)).or_insert(0);
            *e += 1;
        }

        last_char = c;
    }

    // run
    for _ in 0..40 {
        let mut new_pairs: HashMap<(char, char), u64> = HashMap::new();
    
        for ((start, end), count) in pairs {
            for rule in rules {
                // println!("{}{} -> {}", rule.start, rule.end, rule.result);
                if rule.start == start && rule.end == end {
                    let e = new_pairs.entry((start, rule.result)).or_insert(0);
                    *e += count;
                    let e = new_pairs.entry((rule.result, end)).or_insert(0);
                    *e += count;
                    break;
                }
            }
        }
        
        pairs = new_pairs;
    }

    let mut count_per_char: HashMap<char, u64> = HashMap::new();
    pairs.iter().for_each(|(&(start, end), count)| {
        let number = count_per_char.entry(start).or_insert(0);
        *number += count;
        let number = count_per_char.entry(end).or_insert(0);
        *number += count;
    });

    let first = count_per_char.entry(template.chars().nth(0).unwrap()).or_insert(0);
    *first += 1;
    let last = count_per_char.entry(template.chars().last().unwrap()).or_insert(0);
    *last += 1;

    let mut max = 0;
    let mut min = u64::MAX;
    for (_, &count) in count_per_char.iter() {
        let real_count = count / 2;
        if real_count > max { max = real_count }
        if real_count < min { min = real_count }
    }

    max - min
}

pub struct InsertionRule {
    start: char,
    end: char,
    result: char,
}
