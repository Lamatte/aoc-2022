use std::fs;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|values| values.len() == 2)
        .map(|values| score(values[1], values[0]))
        .sum::<i32>() as usize
}

// So ugly! :D
fn score(me: &str, opponent: &str) -> i32 {
    if me == "X" && opponent == "A" {
        1 + 3
    } else if me == "Y" && opponent == "B" {
        2 + 3
    } else if me == "Z" && opponent == "C" {
        3 + 3
    } else if me == "X" && opponent == "B" {
        1 + 0
    } else if me == "X" && opponent == "C" {
        1 + 6
    } else if me == "Y" && opponent == "A" {
        2 + 6
    } else if me == "Y" && opponent == "C" {
        2 + 0
    } else if me == "Z" && opponent == "A" {
        3 + 0
    } else if me == "Z" && opponent == "B" {
        3 + 6
    } else { 0 }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
A Y
B X
C Z
".to_string()), 15);
}

