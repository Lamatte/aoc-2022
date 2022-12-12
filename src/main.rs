use std::fs;
use std::time::Instant;
use itertools::{Itertools};

type Position = (usize, usize); // (line, column)

struct Hill {
    target: Position,
    cells: Vec<Vec<char>>,
    distances: Vec<Vec<Option<usize>>>,
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
    let mut hill = Hill::parse(input);
    hill.explore(vec![hill.target], 0);
    Hill::all_positions(&hill.cells).iter()
        .filter(|position| hill.cells[position.0][position.1] == 'a')
        .filter_map(|position| hill.distances[position.0][position.1])
        .min().unwrap()
}

impl Hill {
    fn parse(input: &String) -> Hill {
        let mut cells = input.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let distances = (0..cells.len()).map(|_| (0..cells[0].len()).map(|_| None).collect()).collect();
        let start = Self::position_of(&cells, 'S').unwrap();
        let target = Self::position_of(&cells, 'E').unwrap();
        cells[target.0][target.1] = 'z';
        cells[start.0][start.1] = 'a';
        Hill {
            target,
            cells,
            distances,
        }
    }

    fn position_of(cells: &Vec<Vec<char>>, x: char) -> Option<Position> {
        Hill::all_positions(cells).into_iter()
            .filter(|position| cells[position.0][position.1] == x)
            .last()
    }

    fn explore(&mut self, positions: Vec<Position>, distance: usize) {
        let mut neighbours: Vec<Position> = vec![];
        positions.iter().unique().for_each(|position| {
            let best_path_found = match self.distances[position.0][position.1] {
                None => true,
                Some(current_distance) => current_distance > distance,
            };
            if best_path_found {
                self.distances[position.0][position.1] = Some(distance);
                neighbours.append(&mut self.get_reachable_neighbours(position));
            }
        });
        if neighbours.len() > 0 {
            self.explore(neighbours, distance + 1);
        }
    }

    fn get_reachable_neighbours(&self, position: &Position) -> Vec<Position> {
        let mut neighbours = vec![];
        if position.0 >= 1 {
            neighbours.push((position.0 - 1, position.1));
        }
        if position.0 < self.cells.len() - 1 {
            neighbours.push((position.0 + 1, position.1));
        }
        if position.1 >= 1 {
            neighbours.push((position.0, position.1 - 1));
        }
        if position.1 < self.cells[0].len() - 1 {
            neighbours.push((position.0, position.1 + 1));
        }
        neighbours.into_iter()
            .filter(|neighbour| {
                self.cells[neighbour.0][neighbour.1] as u32 >= self.cells[position.0][position.1] as u32 - 1
            }).collect()
    }

    fn all_positions(cells: &Vec<Vec<char>>) -> Vec<Position> {
        (0..cells.len()).flat_map(|line| (0..cells[0].len()).into_iter().map(move |column| (line, column))).collect::<Vec<Position>>()
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(&r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
".to_string()), 29);
}

