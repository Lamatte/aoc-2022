use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    let grid = input.lines()
        .map(|l| l.chars()
            .map(|c| c as u8 - 48)
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut res = 0;
    for line in 1..grid.len()-1 {
        for column in 1..grid.len()-1 {
            if is_visible(&grid, line, column) {
                eprintln!("{}/{} is visible", line, column);
                res = res + 1;
            }
        }
    }
    res + edges(grid)
}

fn edges(grid: Vec<Vec<u8>>) -> usize {
    grid.len()*4-4
}

fn is_visible(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    is_visible_left(grid, line, column) || is_visible_right(grid, line, column) || is_visible_top(grid, line, column) || is_visible_bottom(grid, line, column)
}

fn is_visible_left(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    for other_column in 0..column {
        if grid[line][other_column] >= grid[line][column] {
            return false;
        }
    };
    true
}

fn is_visible_right(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    for other_column in column+1..grid.len() {
        if grid[line][other_column] >= grid[line][column] {
            return false;
        }
    };
    true
}

fn is_visible_top(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    for other_line in 0..line {
        if grid[other_line][column] >= grid[line][column] {
            return false;
        }
    };
    true
}


fn is_visible_bottom(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    for other_line in line+1..grid.len() {
        if grid[other_line][column] >= grid[line][column] {
            return false;
        }
    };
    true
}

#[test]
fn test_data() {
    assert_eq!(execute(r"30373
25512
65332
33549
35390
".to_string()), 21);
}

