use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=([\d]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
);

#[derive(Debug, Clone)]
struct Grid {
    all_nodes: Vec<Node>,
    start: usize,
    time_left: usize,
    flow: usize,
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
    for _ in 0..1000 {
        //execute(&input);
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed() / 1000);
}

fn execute(input: &String) -> usize {
    let grid = parse(input);
    eprintln!("{}", grid.start);
    grid.all_nodes.iter().for_each(|node| eprintln!("{:?}", node));

    let results = grid.evaluate(0);
    results.iter().map(|grid| grid.total_flow + grid.flow*grid.time_left).max().unwrap()
}

impl Grid {
    fn distances(&self, node: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let current = vec![node];
        self.explore(current, &mut distances, 0);
        distances
    }

    fn explore(&self, current: Vec<usize>, distances: &mut HashMap<usize, usize>, distance: usize) {
        let mut new = vec![];
        //eprintln!("Exploring {:?} ({})", current, distance);
        current.iter().for_each(|c| {
            if distances.contains_key(c) {
                if distances.get(c).unwrap() > &distance {
                    distances.insert(*c, distance);
                    new.append(&mut self.all_nodes[*c].leads_to.clone());
                }
            } else {
                distances.insert(*c, distance);
                new.append(&mut self.all_nodes[*c].leads_to.clone());
            }
        });
        if new.len() > 0 {
            self.explore(new, distances, distance + 1);
        }
    }

    fn evaluate(&self, depth: usize) -> Vec<Grid> {
        let mut new = vec![];
        let distances = self.distances(self.start);
        //eprintln!("{:?}", distances);
        let mut valves_found = false;
        for i in 0..self.all_nodes.len() {
            let d = *distances.get(&i).unwrap();
            if self.all_nodes[i].value > 0 && d < self.time_left {
                valves_found = true;
                let mut new_grid = self.clone();
                new_grid.start = i;
                new_grid.time_left -= d + 1;
                new_grid.total_flow += new_grid.flow * (d + 1);
                new_grid.flow += new_grid.all_nodes[i].value as usize;
                new_grid.all_nodes[i].value = 0;
                //eprintln!("{} - Going to {} (time left {})", depth, self.all_nodes[i].name, new_grid.time_left);
                //eprintln!("{} - open {} total {}", depth, new_grid.flow, new_grid.total_flow);
                new.append(&mut new_grid.evaluate(depth+1));
            }
        }
        if !valves_found {
            return vec![self.clone()];
        } else {
            //eprintln!("{}: {}", depth, new.len());
            new
        }
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
            //eprintln!("{}({}) -> {}", name, value, others);
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
    let grid = Grid { all_nodes, start, time_left: 30, flow: 0, total_flow: 0 };
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
".to_string()), 1651);
}

