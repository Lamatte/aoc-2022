use std::fs;

const ROCK: &'static str = "A";
const PAPER: &'static str = "B";
const SCISSORS: &'static str = "C";

const LOSE: &'static str = "X";
const WIN: &'static str = "Z";

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|values| values.len() == 2)
        .map(|values| (my_expected_move(values[1], values[0]), values[0]))
        .map(|(my_move, opponent_move)| score(my_move, opponent_move))
        .sum::<i32>() as usize
}

// Boooh!!
fn my_expected_move<'a>(expected_result: &'a str, opponent_move: &'a str) -> &'a str {
    if expected_result == LOSE {
        if opponent_move == ROCK {
            SCISSORS
        } else if opponent_move == PAPER {
            ROCK
        } else {
            PAPER
        }
    } else if expected_result == WIN {
        if opponent_move == ROCK {
            PAPER
        } else if opponent_move == PAPER {
            SCISSORS
        } else {
            ROCK
        }
    } else { // DRAWN
        opponent_move
    }
}

// So ugly! :D
fn score(my_move: &str, opponent_move: &str) -> i32 {
    if my_move == ROCK && opponent_move == ROCK {
        1 + 3
    } else if my_move == PAPER && opponent_move == PAPER {
        2 + 3
    } else if my_move == SCISSORS && opponent_move == SCISSORS {
        3 + 3
    } else if my_move == ROCK && opponent_move == PAPER {
        1 + 0
    } else if my_move == ROCK && opponent_move == SCISSORS {
        1 + 6
    } else if my_move == PAPER && opponent_move == ROCK {
        2 + 6
    } else if my_move == PAPER && opponent_move == SCISSORS {
        2 + 0
    } else if my_move == SCISSORS && opponent_move == ROCK {
        3 + 0
    } else if my_move == SCISSORS && opponent_move == PAPER {
        3 + 6
    } else { 0 }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
A Y
B X
C Z
".to_string()), 12);
}

