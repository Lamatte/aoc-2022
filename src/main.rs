use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

type Coordinates = (usize, usize);

struct Grid {
    raw: Vec<Vec<u8>>,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    grid.all_coordinates().iter()
        .map(|&coordinates| grid.scenic_score(coordinates))
        .sorted()
        .last().unwrap()
}

impl Grid {
    fn scenic_score(&self, coordinates: Coordinates) -> usize {
        self.visible_left_count(coordinates) * self.visible_right_count(coordinates) * self.visible_top_count(coordinates) * self.visible_bottom_count(coordinates)
    }

    fn size(&self) -> usize {
        self.raw.len()
    }

    fn all_coordinates(&self) -> Vec<Coordinates> {
        (0..self.size()).flat_map(|line| (0..self.size()).map(move |column| (line, column))).collect::<Vec<Coordinates>>()
    }

    fn visible_left_count(&self, coordinates: Coordinates) -> usize {
        Self::count(
            (0..coordinates.1).rev().map(|column| self.raw[coordinates.0][column]).collect(),
            self.raw[coordinates.0][coordinates.1],
        )
    }

    fn visible_top_count(&self, coordinates: Coordinates) -> usize {
        Self::count(
            (0..coordinates.0).rev().map(|line| self.raw[line][coordinates.1]).collect(),
            self.raw[coordinates.0][coordinates.1],
        )
    }

    fn visible_right_count(&self, coordinates: Coordinates) -> usize {
        Self::count(
            (coordinates.1 + 1..self.size()).map(|column| self.raw[coordinates.0][column]).collect(),
            self.raw[coordinates.0][coordinates.1],
        )
    }

    fn visible_bottom_count(&self, coordinates: Coordinates) -> usize {
        Self::count(
            (coordinates.0 + 1..self.size()).map(|line| self.raw[line][coordinates.1]).collect(),
            self.raw[coordinates.0][coordinates.1],
        )
    }

    fn count(values: Vec<u8>, current_value: u8) -> usize {
        let mut res = 0;
        for value in values {
            res = res + 1;
            if value >= current_value {
                break;
            }
        }
        res
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.lines()
            .map(|l| l.chars()
                .map(|c| c as u8 - 48)
                .collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        Ok(Grid { raw })
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"30373
25512
65332
33549
35390
".to_string()), 8);
}

