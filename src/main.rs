use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

type Position = (i32, i32); // (line, column)

struct Grid {
    cells: HashSet<Position>,
    depth: i32,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let result = execute(&input);
    println!("{}", result);
    eprintln!("Benching...");
    let start = Instant::now();
    for _ in 0..1000 {
        execute(&input);
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed() / 1000);
}

fn execute(input: &String) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();
    //grid.print();
    let mut count = 0;
    loop {
        match grid.try_and_drop_sand_at((0, 500)) {
            None => break,
            Some(_) => count += 1,
        }
    }
    count
}

impl Grid {
    fn try_and_drop_sand_at(&mut self, position: (i32, i32)) -> Option<(i32, i32)> {
        if self.cells.contains(&(position.0, position.1)) {
            return None;
        }
        let mut sand_position = position;
        loop {
            if sand_position.0 == self.depth + 1 {
                // Reached floor
                break;
            } else if !self.cells.contains(&(sand_position.0 + 1, sand_position.1)) {
                sand_position.0 += 1;
            } else if !self.cells.contains(&(sand_position.0 + 1, sand_position.1 - 1)) {
                sand_position.0 += 1;
                sand_position.1 -= 1;
            } else if !self.cells.contains(&(sand_position.0 + 1, sand_position.1 + 1)) {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                // Resting here!
                break;
            }
        }
        self.cells.insert(sand_position);
        Some(sand_position)
    }

    fn print(&self) {
        let min_column = min_column(&self.cells);
        let max_column = max_column(&self.cells);
        (0..self.depth + 2).for_each(|line| {
            let line_str: String = (min_column - 10..max_column + 12).map(|column| if self.cells.contains(&(line, column)) { '#' } else { '.' }).collect();
            eprintln!("{}", line_str);
        });
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .flat_map(|line| parse_path(line))
            .collect::<HashSet<(i32, i32)>>();
        let depth = max_line(&map);
        Ok(Grid { cells: map, depth })
    }
}

fn parse_path(line: &str) -> Vec<(i32, i32)> {
    line
        .split(" -> ")
        .map(|point| {
            let s = point.split_once(",").unwrap();
            (s.1.parse::<i32>().unwrap(), s.0.parse::<i32>().unwrap())
        })
        .fold(vec![], |mut path, point| {
            let last_point = path.last();
            match last_point {
                None => path.push(point),
                Some(&(line, column)) => {
                    if line == point.0 {
                        (min(column, point.1)..max(column, point.1) + 1).into_iter().for_each(|column| path.push((line, column)));
                    } else {
                        (min(line, point.0)..max(line, point.0) + 1).into_iter().for_each(|line| path.push((line, column)));
                    }
                    path.push(point);
                }
            }
            path
        })
}

fn max_column(map: &HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(_, col)| col).sorted().last().unwrap()
}

fn min_column(map: &HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(_, col)| col).sorted().rev().last().unwrap()
}

fn max_line(map: &HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(line, _)| line).sorted().last().unwrap()
}


#[test]
fn test_data() {
    assert_eq!(execute(&r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
".to_string()), 93);
}

