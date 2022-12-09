use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;
use Move::{Left, Right, Up, Down};

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

type Coordinates = (i32, i32);

#[derive(Debug)]
struct Rope {
    segments: Vec<Coordinates>,
    visited: Vec<Coordinates>,
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
        .map(|(m, count)| (m.parse::<Move>().unwrap(), count.parse::<u32>().unwrap()))
        .flat_map(|(m, count)| (0..count).map(move |_| m))
        .fold(Rope::new(), |rope, m| rope.execute(m));
    rope.visited.iter().unique().count()
}

impl Rope {
    pub(crate) fn execute(&self, m: Move) -> Rope {
        let mut new_segments = vec![self.new_head(&m)];
        for i in 1..self.segments.len() {
            new_segments.push(Self::move_segment(self.segments[i], new_segments[i - 1]));
        };
        let mut visited = self.visited.clone();
        visited.push(new_segments[9]);
        Rope { segments: new_segments, visited }
    }

    fn move_segment(element: Coordinates, previous_element: Coordinates) -> Coordinates {
        if (element.0 - previous_element.0).abs() <= 1 && (element.1 - previous_element.1).abs() <= 1 {
            element
        } else if (element.0 - previous_element.0).abs() == 2 && (element.1 - previous_element.1).abs() != 2 {
            ((element.0 + previous_element.0) / 2, previous_element.1)
        } else if (element.0 - previous_element.0).abs() != 2 && (element.1 - previous_element.1).abs() == 2 {
            (previous_element.0, (element.1 + previous_element.1) / 2)
        } else {
            ((element.0 + previous_element.0) / 2, (element.1 + previous_element.1) / 2)
        }
    }

    fn new_head(&self, m: &Move) -> Coordinates {
        match m {
            Up => (self.segments[0].0, self.segments[0].1 + 1),
            Left => (self.segments[0].0 - 1, self.segments[0].1),
            Right => (self.segments[0].0 + 1, self.segments[0].1),
            Down => (self.segments[0].0, self.segments[0].1 - 1),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Up,
            "L" => Left,
            "R" => Right,
            "D" => Down,
            _ => unimplemented!()
        })
    }
}

impl Rope {
    fn new() -> Rope {
        Rope { segments: vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)], visited: vec![] }
    }
}

#[test]
fn move_right() {
    let rope = Rope::new()
        .execute(Right)
        .execute(Right);
    assert_eq!(rope.segments, vec![(2, 0), (1, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
}

#[test]
fn move_left() {
    let rope = Rope::new()
        .execute(Left)
        .execute(Left);
    assert_eq!(rope.segments, vec![(-2, 0), (-1, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
}

#[test]
fn tail_dont_move() {
    let rope = Rope {
        segments: vec![(1, 1), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2)],
        visited: vec![],
    }
        .execute(Right)
        .execute(Right);
    assert_eq!(rope.segments, vec![(3, 1), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2)],
    );
}

#[test]
fn move_up() {
    let rope = Rope::new()
        .execute(Up)
        .execute(Up)
        .execute(Up);
    assert_eq!(rope.segments, vec![(0, 3), (0, 2), (0, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
}

#[test]
fn move_up_diagonal() {
    let rope = Rope {
        segments: vec![(1, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        visited: vec![],
    }.execute(Up);
    assert_eq!(rope.segments, vec![(1, 2), (1, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
    let rope = rope.execute(Up);
    assert_eq!(rope.segments, vec![(1, 3), (1, 2), (1, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
}

#[test]
fn an_inner_segment_moves_in_diagonal() {
    let rope = Rope {
        segments: vec![(4, 3), (4, 2), (3, 1), (2, 1), (1, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        visited: vec![],
    }.execute(Up);
    assert_eq!(rope.segments, vec![(4, 4), (4, 3), (4, 2), (3, 2), (2, 2), (1, 1), (0, 0), (0, 0), (0, 0), (0, 0)]);
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
".to_string()), 1);
}

