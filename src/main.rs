use std::fs;
use std::str::FromStr;
use crate::Move::{Paper, Rock, Scissors};
use crate::PlayResult::{Draw, Lose, Win};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum PlayResult {
    Win,
    Draw,
    Lose,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "A" {
            Ok(Rock)
        } else if value == "B" {
            Ok(Paper)
        } else if value == "C" {
            Ok(Scissors)
        } else {
            Err(format!("Bad value for a move: {}", value.to_string()))
        }
    }
}

impl PlayResult {
    fn score(&self) -> i32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6
        }
    }
}

impl FromStr for PlayResult {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "X" {
            Ok(Lose)
        } else if value == "Y" {
            Ok(Draw)
        } else if value == "Z" {
            Ok(Win)
        } else {
            Err(format!("Bad value for a play result: {}", value.to_string()))
        }
    }
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|values| values.len() == 2)
        .map(|values| (values[0].parse::<Move>().unwrap(), values[1].parse::<PlayResult>().unwrap()))// Should filter here...
        .map(|(opponent_move, result)| (my_expected_move(result, opponent_move), result))
        .map(|(my_move, result)| my_move.score() + result.score())
        .sum::<i32>() as usize
}

fn my_expected_move(expected_result: PlayResult, opponent_move: Move) -> Move {
    match expected_result {
        Win => match opponent_move {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
        Draw => opponent_move,
        Lose => match opponent_move {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
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

