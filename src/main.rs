use std::collections::HashMap;
use std::{fs, usize};
use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=([\d]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
);

#[derive(Debug, Clone)]
struct Grid {
    all_nodes: Vec<Node>,
    opened_valves: Vec<usize>,
    start: usize,
    time_left: usize,
    current_flow: usize,
    total_flow: usize,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    value: u32,
    leads_to: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let result = execute(&input);
    println!("{}", result);
    eprintln!("Benching...");
    let start = Instant::now();
    for _ in 0..10 {
        execute(&input);
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed() / 10);
}

fn execute(input: &String) -> usize {
    let grid = parse(input);

    // Let's assume that the elephant and I will each take at most half of the valves...
    let count = grid.all_nodes.iter().filter(|node| node.value > 0).count();
    let results = grid.evaluate(0, count as i32 /2);

    // Then try each pair of disjoined valves to find the best result...
    let mut max_pressure = 0;
    for i in 0..results.len() {
        for j in i..results.len() {
            if are_disjoined(&results[i].opened_valves, &results[j].opened_valves) {
                let pressure = results[i].total_flow() + results[j].total_flow();
                if pressure > max_pressure {
                    max_pressure = pressure;
                }
            }
        }
    }
    max_pressure
}

impl Grid {
    fn evaluate(&self, depth: usize, max_depth: i32) -> Vec<Grid> {
        if depth as i32 == max_depth {
            return vec![self.clone()];
        }
        let mut new = vec![];
        let distances = self.distances(self.start);
        let mut valve_found = false;
        for i in 0..self.all_nodes.len() {
            let d = *distances.get(&i).unwrap();
            if self.all_nodes[i].value > 0 && d < self.time_left {
                valve_found = true;
                let mut new_grid = self.clone();
                new_grid.start = i;
                new_grid.time_left -= d + 1;
                new_grid.total_flow += new_grid.current_flow * (d + 1);
                new_grid.current_flow += new_grid.all_nodes[i].value as usize;
                new_grid.all_nodes[i].value = 0;
                new_grid.opened_valves.push(i);
                new.append(&mut new_grid.evaluate(depth + 1, max_depth));
            }
        }
        if !valve_found {
            return vec![self.clone()];
        } else {
            new
        }
    }

    fn distances(&self, node: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        self.explore(vec![node], &mut distances, 0);
        distances
    }

    fn explore(&self, current: Vec<usize>, distances: &mut HashMap<usize, usize>, distance: usize) {
        let mut new = vec![];
        current.iter().for_each(|c| {
            if match distances.get(c) {
                None => true,
                Some(current_distance) => current_distance > &distance,
            } {
                distances.insert(*c, distance);
                new.append(&mut self.all_nodes[*c].leads_to.clone());
            }
        });
        if new.len() > 0 {
            self.explore(new, distances, distance + 1);
        }
    }

    fn total_flow(&self) -> usize {
        self.total_flow + self.current_flow * self.time_left
    }
}

fn parse(input: &String) -> Grid {
    let mut start = 0;
    let mut all_nodes: Vec<Node> = vec![];
    let mut children = HashMap::new();
    input.lines()
        .filter(|line| line.len() > 0)
        .map(|s| {
            let captures = RE.captures(s).unwrap();
            (captures[1].to_string(), captures[2].to_string().parse::<u32>().unwrap(), captures[3].to_string())
        })
        .for_each(|(name, value, others)| {
            if name == "AA" {
                start = all_nodes.len();
            }
            children.insert(name.clone(), others);
            add_node(Node {
                name,
                value,
                leads_to: vec![],
            }, &mut all_nodes);
        });
    children.iter().for_each(|(name, others)| {
        let mut v = vec![];
        others.split(", ").into_iter().for_each(|other| {
            v.push(get_node(&other.to_string(), &all_nodes));
        });
        let i = get_node(name, &all_nodes);
        all_nodes[i].leads_to = v;
    });
    let grid = Grid { all_nodes, opened_valves: vec![], start, time_left: 26, current_flow: 0, total_flow: 0 };
    grid
}

fn add_node(node: Node, all_nodes: &mut Vec<Node>) -> usize {
    all_nodes.push(node);
    all_nodes.len() - 1
}

fn get_node(name: &String, all_nodes: &Vec<Node>) -> usize {
    for i in 0..all_nodes.len() {
        if all_nodes[i].name == *name {
            return i;
        }
    };
    unimplemented!()
}

fn are_disjoined(v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
    for i in 0..v1.len() {
        if v2.contains(&v1[i]) {
            return false;
        }
    }
    true
}

#[test]
fn test_data() {
    assert_eq!(execute(&r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
".to_string()), 1707);
}

