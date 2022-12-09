use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;
use Move::Down;
use crate::Move::{Left, Right, Up};

#[derive(Debug)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

type Coordinates = (i32, i32);

#[derive(Debug)]
struct Rope {
    head: Coordinates,
    tail: Coordinates,
    visited: Vec<Coordinates>,
}

impl Rope {
    pub(crate) fn execute(&self, m: Move) -> Rope {
        let head = self.move_head(&m);
        let tail = self.move_tail(head);
        let mut visited = self.visited.clone();
        visited.push(tail);
        let rope = Rope { head, tail, visited };
        rope
    }

    fn move_tail(&self, head: Coordinates) -> Coordinates {
        if (self.tail.0 - head.0).abs() <= 1 && (self.tail.1 - head.1).abs() <= 1 {
            self.tail
        } else {
            if (self.tail.0 - head.0).abs() == 2 {
                ((self.tail.0 + head.0) / 2, head.1)
            } else if (self.tail.1 - head.1).abs() == 2 {
                (head.0, (self.tail.1 + head.1) / 2)
            } else {
                ((self.tail.0 + head.0) / 2, (self.tail.1 + head.1) / 2)
            }
        }
    }

    fn move_head(&self, m: &Move) -> Coordinates {
        match m {
            Up => (self.head.0, self.head.1 + 1),
            Left => (self.head.0 - 1, self.head.1),
            Right => (self.head.0 + 1, self.head.1),
            Down => (self.head.0, self.head.1 - 1),
        }
    }
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    let rope = input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(" ").unwrap())
        .flat_map(|(m, count)| (0..count.parse::<u32>().unwrap()).map(move |_| m.parse::<Move>().unwrap()))
        .fold(Rope::new(), |rope, m| rope.execute(m));
    rope.visited.iter().unique().count()
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Move::Up,
            "L" => Move::Left,
            "R" => Move::Right,
            "D" => Down,
            _ => unimplemented!()
        })
    }
}

impl Rope {
    fn new() -> Rope {
        Rope { head: (0, 0), tail: (0, 0), visited: vec![] }
    }
}

#[test]
fn move_right() {
    let rope = Rope {
        head: (2, 0),
        tail: (1, 0),
        visited: vec![],
    }.execute(Right);
    assert_eq!(rope.head, (3, 0));
    assert_eq!(rope.tail, (2, 0));
}

#[test]
fn move_left() {
    let rope = Rope {
        head: (2, 0),
        tail: (1, 0),
        visited: vec![],
    }.execute(Left);
    assert_eq!(rope.head, (1, 0));
    assert_eq!(rope.tail, (1, 0));
}

#[test]
fn tail_dont_move() {
    let rope = Rope {
        head: (0, 0),
        tail: (1, 1),
        visited: vec![],
    }.execute(Right).execute(Right);
    assert_eq!(rope.head, (2, 0));
    assert_eq!(rope.tail, (1, 1));
}

#[test]
fn move_up() {
    let rope = Rope {
        head: (2, 0),
        tail: (1, 0),
        visited: vec![],
    }.execute(Up);
    assert_eq!(rope.head, (2, 1));
    assert_eq!(rope.tail, (1, 0));
}

#[test]
fn move_up_diagonal() {
    let rope = Rope {
        head: (2, 1),
        tail: (1, 0),
        visited: vec![],
    }.execute(Up);
    assert_eq!(rope.head, (2, 2));
    assert_eq!(rope.tail, (2, 1));
}

#[test]
fn test_data() {
    assert_eq!(execute(r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
".to_string()), 13);
}

