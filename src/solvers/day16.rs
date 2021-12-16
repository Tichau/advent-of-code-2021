use std::fs::File;
use std::io::{self, BufRead};

pub fn parser(input_file: io::BufReader<File>) -> String {
    for line in input_file.lines() {
        if let Ok(ip) = line {
            let mut input = String::new();
            for c in ip.chars() {
                let value = u8::from_str_radix(&c.to_string(), 16).unwrap();
                let string_value = format!("{:04b}", value);
                input.push_str(string_value.as_str())
            }

            return input;
        }
    }

    panic!("No input found")
}

pub fn part1(input: &String) -> i32 {
    let data = input.as_str();
    let (packet, _, _) = Packet::parse(data);
    count_version(&packet)
}

pub fn part2(input: &String) -> i64 {
    let data = input.as_str();
    let (packet, _, _) = Packet::parse(data);
    packet.evaluate()
}

fn count_version(packet: &Packet) -> i32 {
    let mut count: i32 = packet.version as i32;
    for sub_packet in packet.sub_packets.iter() {
        count += count_version(sub_packet);
    }

    count
}

enum Type {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    Equal,
}

impl Type {
    fn from(type_id:u8) -> Self {
        match type_id {
            0 => Type::Sum,
            1 => Type::Product,
            2 => Type::Minimum,
            3 => Type::Maximum,
            4 => Type::Literal,
            5 => Type::GreaterThan,
            6 => Type::LessThan,
            7 => Type::Equal,
            _ => panic!("Unknown type id {}", type_id),
        }
    }
}

struct Packet {
    // Common
    version: u8,
    type_id: Type,
    
    // Literal
    literal_content: i64,

    // Operator
    sub_packets: Box<[Packet]>,
}

impl Packet {
    fn parse(data: &str) -> (Packet, &str, usize) {
        let mut data = data;
        let (string, next_data) = data.split_at(3);
        let version = u8::from_str_radix(string, 2).unwrap();
        data = next_data;
        let (string, next_data) = data.split_at(3);
        let type_id = Type::from(u8::from_str_radix(string, 2).unwrap());
        data = next_data;

        let mut packet_length = 6;
        let packet;
        match type_id {
            Type::Literal => {
                // Literal packet
                let mut content_string = String::new();
                loop {
                    let (string, next_data) = data.split_at(1);
                    let last = string.chars().next().unwrap() == '0';
                    data = next_data;
        
                    let (string, next_data) = data.split_at(4);
                    content_string.push_str(string);
                    data = next_data;
        
                    packet_length += 5;
                    if last {
                        break;
                    }
                }
        
                let content = i64::from_str_radix(content_string.as_str(), 2).unwrap();

                packet = Packet {
                    version: version,
                    type_id: type_id,
                    literal_content: content,

                    sub_packets: Default::default(),
                };
            }
            _ => {
                // Operator packet
                let (string, next_data) = data.split_at(1);
                let length_type_id = string.chars().next().unwrap() == '1';
                data = next_data;
                packet_length += 1;

                let mut sub_packets: Vec<Packet> = Vec::new();
                if length_type_id {
                    let (string, next_data) = data.split_at(11);
                    let subpacket_count = i32::from_str_radix(string, 2).unwrap();
                    data = next_data;
                    packet_length += 11;

                    for _ in 0..subpacket_count {
                        let (sub_packet, next_data, subpacket_length) = Packet::parse(data);
                        sub_packets.push(sub_packet);
                        data = next_data;
                        packet_length += subpacket_length;
                    }
                } else {
                    let (string, next_data) = data.split_at(15);
                    let total_length = usize::from_str_radix(string, 2).unwrap();
                    data = next_data;
                    packet_length += 15;

                    let mut sub_packets_length = 0;
                    while sub_packets_length < total_length {
                        let (sub_packet, next_data, subpacket_length) = Packet::parse(data);
                        sub_packets.push(sub_packet);
                        data = next_data;
                        sub_packets_length += subpacket_length;
                    }
                    
                    packet_length += sub_packets_length;
                }

                packet = Packet {
                    version: version,
                    type_id: type_id,
                    literal_content: Default::default(),

                    sub_packets: sub_packets.into_boxed_slice(),
                };
            }
        }

        return (packet, data, packet_length);
    }

    fn evaluate(&self) -> i64 {
        match self.type_id {
            Type::Sum => self.sub_packets.iter().fold(0, |res, p| res + p.evaluate()),
            Type::Product => self.sub_packets.iter().fold(1, |res, p| res * p.evaluate()),
            Type::Minimum => self.sub_packets.iter().fold(i64::MAX, |res, p| res.min(p.evaluate())),
            Type::Maximum => self.sub_packets.iter().fold(i64::MIN, |res, p| res.max(p.evaluate())),
            Type::Literal => self.literal_content,
            Type::GreaterThan => if self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate() { 1 } else { 0 },
            Type::LessThan => if self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate() { 1 } else { 0 },
            Type::Equal => if self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate() { 1 } else { 0 },
        }
    }
}
