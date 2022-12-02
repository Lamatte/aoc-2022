use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
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

fn score(my_move: &str, opponent_move: &str) -> i32 {
    move_score(my_move) + match_score(my_move, opponent_move)
}

fn move_score(my_move: &str) -> i32 {
    if my_move == ROCK {
        1
    } else if my_move == PAPER {
        2
    } else {
        3
    }
}

fn match_score(my_move: &str, opponent_move: &str) -> i32 {
    if my_move == opponent_move {
        3
    } else if (my_move == ROCK && opponent_move == PAPER)
        || (my_move == PAPER && opponent_move == SCISSORS)
        || (my_move == SCISSORS && opponent_move == ROCK){
        0
    } else {
        6
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
A Y
B X
C Z
".to_string()), 12);
}

