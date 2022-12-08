use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    let grid = parse_input(input);
    (0..grid.len())
        .flat_map(|line| (0..grid.len()).map(move |column| (line, column)))
        .map(|(line, column)| scenic_score(&grid, line, column))
        .sorted()
        .last().unwrap()
}

fn scenic_score(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    visible_left_count(grid, line, column) * visible_right_count(grid, line, column) * visible_top_count(grid, line, column) * visible_bottom_count(grid, line, column)
}

fn visible_left_count(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    count(
        (0..column).rev().map(|column| grid[line][column]).collect(),
        grid[line][column],
    )
}

fn visible_top_count(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    count(
        (0..line).rev().map(|line| grid[line][column]).collect(),
        grid[line][column],
    )
}

fn visible_right_count(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    count(
        (column + 1..grid.len()).map(|column| grid[line][column]).collect(),
        grid[line][column],
    )
}

fn visible_bottom_count(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    count(
        (line + 1..grid.len()).map(|line| grid[line][column]).collect(),
        grid[line][column],
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

fn parse_input(input: String) -> Vec<Vec<u8>> {
    let grid = input.lines()
        .map(|l| l.chars()
            .map(|c| c as u8 - 48)
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    grid
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

