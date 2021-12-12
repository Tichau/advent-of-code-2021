use std::fs::File;
use std::io;
use std::fmt::{Display, Formatter, Result};
use crate::helpers;

pub fn parser(input_file: io::BufReader<File>) -> Graph {
    let inputs = helpers::parse_file_to_list(input_file, |line| {
        let mut parts = line.split('-');
        (String::from(parts.next().unwrap()), String::from(parts.next().unwrap()))
    });

    let mut graph: Graph = Graph::new();
    
    inputs.iter().for_each(|(start, end)| {
        let start_type = Type::from(String::from(start));
        let end_type = Type::from(String::from(end));

        // Nodes        
        let start_index;
        if let Some(index) = graph.caves.iter().position(|c| c.cave_type == start_type) {
            start_index = index;
        } else {
            graph.caves.push(Cave::new(start_type));
            start_index = graph.caves.len() - 1;
        }

        let end_index;
        if let Some(index) = graph.caves.iter().position(|c| c.cave_type == end_type) {
            end_index = index;
        } else {
            graph.caves.push(Cave::new(end_type));
            end_index = graph.caves.len() - 1;
        }

        // Edges
        if !graph.caves[start_index].neighbours.contains(&end_index) {
            graph.caves[start_index].neighbours.push(end_index);
        }

        if !graph.caves[end_index].neighbours.contains(&start_index) {
            graph.caves[end_index].neighbours.push(start_index);
        }
    });

    graph
}

pub fn part1(graph: &Graph) -> usize {
    let mut path: Vec<usize> = Vec::new();
    path.push(graph.caves.iter().position(|c| c.cave_type == Type::Start).unwrap());
    graph.explore_path(path, false).len()
}

pub fn part2(graph: &Graph) -> usize {
    let mut path: Vec<usize> = Vec::new();
    path.push(graph.caves.iter().position(|c| c.cave_type == Type::Start).unwrap());
    graph.explore_path(path, true).len()
}

pub struct Graph {
    caves: Vec<Cave>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            caves: Vec::new(),
        }
    }
    
    fn explore_path(&self, path: Vec<usize>, twice_for_one_small: bool) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = Vec::new();
        let &index = path.last().unwrap();
        for &neighbour_index in &(self.caves[index]).neighbours {
            let neighbour = &self.caves[neighbour_index];
            let (valid, explore, twice) = match &neighbour.cave_type {
                Type::Start => (false, false, twice_for_one_small),
                Type::End => (true, false, twice_for_one_small),
                Type::Big(_) => (true, true, twice_for_one_small),
                Type::Small(_) => {
                    if path.contains(&neighbour_index) {
                        if twice_for_one_small {
                            (true, true, false)
                        } else {
                            (false, false, false)
                        }
                    } else {
                        (true, true, twice_for_one_small)
                    }
                }
            };

            if valid {
                let mut npath = path.clone();
                npath.push(neighbour_index);

                if explore {
                    for np in self.explore_path(npath, twice) {
                        paths.push(np);
                    }
                } else {
                    paths.push(npath);
                }
            }
        }

        paths
    }
}

impl Default for Graph {
    fn default() -> Graph {
        Graph::new()
    }
}

struct Cave {
    cave_type: Type,
    neighbours: Vec<usize>
}

impl Cave {
    fn new(cave_type: Type) -> Cave {
        Cave {
            cave_type,
            neighbours: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
enum Type {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Type {
    fn from(name: String) -> Type {
        match name.as_str() {
            "start" => Type::Start,
            "end" => Type::End,
            _ => {
                if name.chars().last().unwrap().is_uppercase() {
                    Type::Big(name)
                } else {
                    Type::Small(name)
                }
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Start => write!(f, "start"),
            Type::End => write!(f, "end"),
            Type::Big(name) => write!(f, "{}", name),
            Type::Small(name) => write!(f, "{}", name),
        }
    }
}
