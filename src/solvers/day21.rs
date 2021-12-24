use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub fn parser(input_file: io::BufReader<File>) -> (u8, u8) {
    let regex = Regex::new(r"^Player ([12]) starting position: ([0-9]+)$").unwrap();
    let mut player1_start = u8::MAX;
    let mut player2_start = u8::MAX;
    for line in input_file.lines() {
        if let Ok(ip) = line {
            if let Option::Some(capture) = regex.captures(&ip) {
                let player = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let start = capture.get(2).unwrap().as_str().parse::<u8>().unwrap();
                match player {
                    1 => player1_start = start,
                    2 => player2_start = start,
                    _ => panic!("Unknown player"),
                }
            }
        }
    }

    (player1_start, player2_start)
}

pub fn part1(input: &(u8, u8)) -> u32 {
    let mut players = [PlayerState { position: input.0 as u16, score: 0}, PlayerState { position: input.1 as u16, score: 0}];
    let mut current_player = 0;
    let mut dice = 0;
    while players[0].score < 1000 && players[1].score < 1000 {
        for _ in 0..3 {
            dice += 1;
            players[current_player].position += dice;
        }

        players[current_player].position = (players[current_player].position - 1) % 10 + 1;
        players[current_player].score += players[current_player].position as u32;

        current_player = if current_player == 0 { 1 } else { 0 };
    }

    println!("Dice: {} ", dice);
    println!("Player 1 @ {} score: {}", players[0].position, players[0].score);
    println!("Player 2 @ {} score: {}", players[1].position, players[1].score);

    let smallest = if players[0].score > players[1].score { players[1].score } else { players[0].score };

    smallest * dice as u32
}

pub fn part2(input: &(u8, u8)) -> u64 {
    let game_state = GameState {
        players: [PlayerState { position: input.0 as u16, score: 0}, PlayerState { position: input.1 as u16, score: 0}],
        current_player: 0,
    };

    let mut weight = [0u64; 10];
    for d1 in 1..4 {
        for d2 in 1..4 {
            for d3 in 1..4 {
                weight[d1+d2+d3] += 1;
            }
        }
    }

    let mut wins = (0, 0);
    for dices in 3usize..10 {
        let branch = play_turn(game_state.clone(), dices, &weight);
        wins.0 += branch.0 * weight[dices];
        wins.1 += branch.1 * weight[dices];
    }
    
    if wins.0 > wins.1 { wins.0 } else { wins.1 }
}

fn play_turn(mut game_state: GameState, dice_value: usize, weight: &[u64]) -> (u64, u64) {
    let current_player = &mut game_state.players[game_state.current_player];
    current_player.position += dice_value as u16;
    current_player.position = (current_player.position - 1) % 10 + 1;
    current_player.score += current_player.position as u32;
    if current_player.score >= 21 {
        // end of a game
        if game_state.current_player == 0 {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    game_state.current_player = if game_state.current_player == 0 { 1 } else { 0 };

    let mut wins = (0, 0);
    for dices in 3usize..10 {
        let branch = play_turn(game_state.clone(), dices, weight);
        wins.0 += branch.0 * weight[dices];
        wins.1 += branch.1 * weight[dices];
    }

    wins
}


#[derive(Copy, Clone)]
struct PlayerState {
    position: u16,
    score: u32,
}

#[derive(Copy, Clone)]
struct GameState {
    players: [PlayerState; 2],
    current_player: usize,
}