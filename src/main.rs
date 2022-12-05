use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let board = parts[1].lines()
        .map(|l| parse_move(l))
        .fold(parse_board(parts[0]), |mut acc: Vec<Vec<char>>, (count, from, to)| {
            (1..count+1).for_each(|_| {
                let x = acc[from as usize - 1].pop().unwrap();
                acc[to as usize - 1].push(x);
            });
            acc
        });
    board.iter()
        .filter_map(|s| s.last())
        .collect()

}

fn parse_board(input: &str) -> Vec<Vec<char>> {
    let board = input.lines()
        .map(|l| (0..9).into_iter().map(|i| get_element_at(l, i)).collect::<Vec<Option<char>>>())
        .rev()
        .fold(empty_game(), |mut acc: Vec<Vec<char>>, chars| {
            (0..9).into_iter()
                .for_each(|i| {
                    if let Some(c) = chars[i] {
                        acc[i].push(c);
                    }
                });
            acc
        });
    board
}

fn empty_game() -> Vec<Vec<char>> {
    (0..9).into_iter()
        .map(|_| vec![])
        .collect()
}

fn parse_move(l: &str) -> (u8, u8, u8) {
    let captures = MOVE_REGEX.captures(l).unwrap();
    (captures[1].parse::<u8>().unwrap(), captures[2].parse::<u8>().unwrap(), captures[3].parse::<u8>().unwrap())
}

fn get_element_at(l: &str, index: usize) -> Option<char> {
    if l.len() >= 1 + index*4 {
        let res = l.as_bytes()[1 + index * 4] as char;
        if res == ' ' {
            None
        } else {
            Some(res)
        }
    } else {
        None
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
".to_string()), "CMZ");
}

