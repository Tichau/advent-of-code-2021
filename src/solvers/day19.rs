use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use ndarray::{Array1, arr1, Array2, arr2};

pub fn parser(input_file: io::BufReader<File>) -> Vec<ScannerInput> {
    let scanner_regex = Regex::new(r"^--- scanner ([0-9]+) ---$").unwrap();
    let beacon_regex = Regex::new(r"^(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)$").unwrap();
    let mut scanner = ScannerInput::default();

    let mut inputs: Vec<ScannerInput> = Vec::new();
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if let Option::Some(capture) = scanner_regex.captures(&ip) {
                assert!(scanner.id == u16::MAX);
                scanner.id = capture.get(1).unwrap().as_str().parse::<u16>().unwrap();
                continue;
            }

            if let Option::Some(capture) = beacon_regex.captures(&ip) {
                assert!(scanner.id != u16::MAX);
                let x = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let z = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
                scanner.beacons.push(arr1(&[x, y, z]));
                continue;
            }

            if ip == "" {
                inputs.push(scanner);
                scanner = ScannerInput::default();
                continue;
            }
        }
    }
    
    inputs.push(scanner);

    inputs
}

pub fn part1(inputs: &Vec<ScannerInput>) -> usize {
    let scanners = analyse_scanner(inputs);

    let mut beacons: Vec<Array1<i32>> = Vec::new();
    for scanner in &scanners {
        assert!(scanner.defined);

        for beacon in &scanner.input.beacons {
            let ref_pos = scanner.orientation.dot(beacon) + &scanner.position;
            if !beacons.contains(&ref_pos) {
                beacons.push(ref_pos);
            }
        }
    }

    beacons.len()
}

pub fn part2(inputs: &Vec<ScannerInput>) -> i32 {
    let scanners = analyse_scanner(inputs);

    let mut max_distance = 0;
    for scanner1 in &scanners {
        assert!(scanner1.defined);
        for scanner2 in &scanners {
            assert!(scanner2.defined);
            let diff = &scanner1.position - &scanner2.position;
            let distance = diff[0].abs() + diff[1].abs() + diff[2].abs();
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    
    max_distance
}

fn analyse_scanner<'a>(inputs: &'a Vec<ScannerInput>) -> Vec<Scanner<'a>> {
    let mut scanners: Vec<Scanner> = Vec::new();

    for input in inputs {
        let scanner = Scanner {
            input: input,
            defined: if input.id == 0 { true } else { false },
            position: arr1(&[0, 0, 0]),
            orientation: arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
        };

        scanners.push(scanner);
    }

    let rotation_matrixes = generate_rotation_matrixes();

    loop {
        for reference_index in 0..scanners.len() {
            for scanner_index in 1..scanners.len() {
                if reference_index == scanner_index {
                    continue;
                }
    
                if !scanners[reference_index].defined || scanners[scanner_index].defined {
                    continue;
                }
    
                let (matches, position, orientation) = scanners[scanner_index].find_orientation_and_offset(&rotation_matrixes, &scanners[reference_index]);
                if matches >= 12 {
                    scanners[scanner_index].position = &scanners[reference_index].position + scanners[reference_index].orientation.dot(&position);
                    scanners[scanner_index].orientation = scanners[reference_index].orientation.dot(&orientation);
                    scanners[scanner_index].defined = true;
                }
            }
        }

        if scanners.iter().fold(0, |sum, scanner| sum + if scanner.defined { 1 } else { 0 }) == scanners.len() {
            break;
        }
    }

    scanners
}

fn generate_rotation_matrixes() -> Vec<Array2<i32>> {
    let x_rot_matrix : Vec<Array2<i32>> = vec!(
        arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
        arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]),
        arr2(&[[1, 0, 0], [0, -1, 0], [0, 0, -1]]),
        arr2(&[[1, 0, 0], [0, 0, 1], [0, -1, 0]]),
    );

    let y_rot_matrix : Vec<Array2<i32>> = vec!(
        arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
        arr2(&[[0, 0, 1], [0, 1, 0], [-1, 0, 0]]),
        arr2(&[[-1, 0, 0], [0, 1, 0], [0, 0, -1]]),
        arr2(&[[0, 0, -1], [0, 1, 0], [1, 0, 0]]),
    );

    let z_rot_matrix : Vec<Array2<i32>> = vec!(
        arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
        arr2(&[[0, -1, 0], [1, 0, 0], [0, 0, 1]]),
        arr2(&[[-1, 0, 0], [0, -1, 0], [0, 0, 1]]),
        arr2(&[[0, 1, 0], [-1, 0, 0], [0, 0, 1]]),
    );

    let mut matrixes: Vec<Array2<i32>> = Vec::new();
    for xrot in &x_rot_matrix {
        for yrot in &y_rot_matrix {
            for zrot in &z_rot_matrix {
                let rot_matrix: Array2<i32> = xrot.dot(yrot).dot(zrot);
                if !matrixes.contains(&rot_matrix) {
                    matrixes.push(rot_matrix);
                }
            }
        }
    }

    matrixes
}

pub struct Scanner<'a> {
    input: &'a ScannerInput,
    defined: bool,
    position: Array1<i32>,
    orientation: Array2<i32>,
}

impl<'a> Scanner<'a> {
    fn find_orientation_and_offset(&self, rotation_matrixes: &Vec<Array2<i32>>, reference: &Self) -> (i32, Array1<i32>, Array2<i32>) {
        let mut orientation = arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        let mut offset = arr1(&[0, 0, 0]);
        let mut matches = 0;

        // Test rotations.
        for rotation_matrix in rotation_matrixes {
            let (count, offset_pos) = self.compute_offset_with_reference(reference, rotation_matrix);
            if count >= 12 {
                matches = count;
                offset = offset_pos;
                orientation = rotation_matrix.clone();
                break;
            }
        }

        (matches, offset, orientation)
    }

    fn count_match(&self, orientation: &Array2<i32>, offset: &Array1<i32>, reference: &Self) -> i32 {
        let mut count = 0;
        for beacon_index in 0..self.input.beacons.len() {
            let beacon = &orientation.dot(&self.input.beacons[beacon_index]) + offset;
            for reference_beacon in &reference.input.beacons {
                if beacon == reference_beacon {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    fn compute_offset_with_reference(&self, reference: &Self, orientation: &Array2<i32>) -> (i32, Array1<i32>) {
        for start_beacon_index in 0..self.input.beacons.len() {
            let start_beacon = &orientation.dot(&self.input.beacons[start_beacon_index]);
            for end_beacon_index in start_beacon_index+1..self.input.beacons.len() {
                let end_beacon = &orientation.dot(&self.input.beacons[end_beacon_index]);
                let distance = start_beacon - end_beacon;
                for ref_start_beacon_index in 0..reference.input.beacons.len() {
                    for ref_end_beacon_index in ref_start_beacon_index+1..reference.input.beacons.len() {
                        let ref_distance = &reference.input.beacons[ref_end_beacon_index] - &reference.input.beacons[ref_start_beacon_index];
                        if distance == ref_distance {
                            let offset;
                            let start_start_offset = &reference.input.beacons[ref_start_beacon_index] - start_beacon;
                            let end_end_offset = &reference.input.beacons[ref_end_beacon_index] - end_beacon;
                            let start_end_offset = &reference.input.beacons[ref_start_beacon_index] - end_beacon;
                            let end_start_offset = &reference.input.beacons[ref_end_beacon_index] - start_beacon;
                            if start_start_offset == end_end_offset {
                                offset = start_start_offset;
                            } else if start_end_offset == end_start_offset {
                                offset = start_end_offset;
                            } else {
                                panic!("should have found an offset"); 
                            }

                            let count = self.count_match(orientation, &offset, reference);
                            if count >= 12 {
                                return (count, offset);
                            }
                        }
                    }
                }
            }
        }

        (0, arr1(&[0, 0, 0]))
    }
}

pub struct ScannerInput {
    id: u16,
    beacons: Vec<Array1<i32>>,
}

impl Default for ScannerInput {
    fn default() -> Self {
        ScannerInput {
            id: u16::MAX,
            beacons: Vec::default(),
        }
    }
}
