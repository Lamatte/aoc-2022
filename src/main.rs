use std::fs;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

type Board = Vec<Vec<char>>;

struct Move {
    from: u8,
    to: u8,
    count: u8,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let board = parts[1].lines()
        .filter_map(|s| s.parse::<Move>().ok())
        .fold(parse_board(parts[0]), |board: Board, m| m.handle_move(board));

    board.iter()
        .filter_map(|s| s.last())
        .collect()
}

impl Move {
    fn handle_move(&self, mut board: Board) -> Board {
        let mut tmp = vec![];
        (1..self.count + 1).for_each(|_| {
            let x = board[self.from as usize - 1].pop().unwrap();
            tmp.push(x)
        });
        (1..self.count + 1).for_each(|_| {
            let x = tmp.pop().unwrap();
            board[self.to as usize - 1].push(x);
        });
        board
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = MOVE_REGEX.captures(s).unwrap();
        Ok(Move {
            count: captures[1].parse::<u8>().unwrap(),
            from: captures[2].parse::<u8>().unwrap(),
            to: captures[3].parse::<u8>().unwrap(),
        })
    }
}

fn parse_board(input: &str) -> Board {
    input.lines()
        .rev()
        .map(|l| get_crates_on_line(l))
        .fold(empty_board(), |board: Board, crates| push_crates_on_board(board, crates))
}

fn push_crates_on_board(mut board: Board, crates: Vec<Option<char>>) -> Board {
    (0..9).into_iter()
        .for_each(|i| {
            if let Some(c) = crates[i] {
                board[i].push(c);
            }
        });
    board
}

fn get_crates_on_line(l: &str) -> Vec<Option<char>> {
    (0..9).into_iter()
        .map(|i| get_crate_at(l, i))
        .collect::<Vec<Option<char>>>()
}

fn empty_board() -> Board {
    (0..9).into_iter()
        .map(|_| vec![])
        .collect()
}

fn get_crate_at(l: &str, index: usize) -> Option<char> {
    if l.len() >= 1 + index * 4 {
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
".to_string()), "MCD");
}

