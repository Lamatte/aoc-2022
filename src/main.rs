use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use itertools::{Itertools, sorted};

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let result = execute(&input);
    println!("{}", result);
    eprintln!("Benching...");
    let start = Instant::now();
    for _ in 0..1000 {
        //execute(&input);
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed() / 1000);
}

fn execute(input: &String) -> usize {
    let mut map = input.lines()
        .flat_map(|line| parse_path(line))
        .collect::<HashSet<(i32, i32)>>();
    print(&mut map);
    let mut count = 0;
    loop {
        let position = add_sand(&mut map);
        match position {
            None => {
                break;
            }
            Some(_) => {
                count += 1;
            }
        }
    }
    count
}

fn print(mut map: &mut HashSet<(i32, i32)>) {
    (0..max_line(&mut map) + 1).for_each(|line| {
        let line_str: String = (min_column(&mut map)..max_column(&mut map) + 1).map(|column| if map.contains(&(line, column)) { '#' } else { '.' }).collect();
        eprintln!("{}", line_str);
    });
}

fn max_column(map: &mut &mut HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(_, col)| col).sorted().last().unwrap()
}

fn min_column(map: &mut &mut HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(_, col)| col).sorted().rev().last().unwrap()
}

fn max_line(map: &mut &mut HashSet<(i32, i32)>) -> i32 {
    *map.iter().map(|(line, _)| line).sorted().last().unwrap()
}

fn add_sand(mut map: &mut HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let mut sand_position = (0, 500);
    loop {
        if sand_position.0 >= max_line(&mut map) {
            return None;
        }
        if !map.contains(&(sand_position.0 + 1, sand_position.1)) {
            sand_position.0 += 1;
        } else if !map.contains(&(sand_position.0 + 1, sand_position.1 - 1)) {
            sand_position.0 += 1;
            sand_position.1 -= 1;
        } else if !map.contains(&(sand_position.0 + 1, sand_position.1 + 1)) {
            sand_position.0 += 1;
            sand_position.1 += 1;
        } else {
            break;
        }
    }
    map.insert(sand_position);
    //eprintln!("{:?}", sand_position);
    //print(&mut map);
    Some(sand_position)
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
            eprintln!("drawing line from {:?} to {:?}", last_point, point);
            match last_point {
                None => path.push(point),
                Some(&(line, column)) => {
                    //eprintln!("{} to {}", line, point.1);
                    if line == point.0 {
                        eprintln!("same line");
                        (min(column, point.1)..max(column, point.1) + 1).into_iter().for_each(|column| path.push((line, column)));
                    } else {
                        eprintln!("same col");
                        (min(line, point.0)..max(line, point.0) + 1).into_iter().for_each(|line| path.push((line, column)));
                    }
                    path.push(point);
                }
            }
            path
        })
}

#[test]
fn test_data() {
    assert_eq!(execute(&r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
".to_string()), 24);
}

