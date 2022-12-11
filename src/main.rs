use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    modulus: u64,
    target: Box<dyn Fn(u64) -> u64>,
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
    let mut monkeys = input.split("\n\n")
        .filter_map(|s| s.parse::<Monkey>().ok())
        .collect::<Vec<Monkey>>();
    let modulus_product: u64 = monkeys.iter()
        .map(|m| m.modulus)
        .product();
    let mut inspections = [0 as u64; 10];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let mut val = monkeys[i].items.pop_front().unwrap();
                inspections[i] += 1;
                val = (monkeys[i].operation)(val % modulus_product);
                let other_monkey = (monkeys[i].target)(val) as usize;
                monkeys[other_monkey].items.push_back(val);
            }
        }
    }
    inspections.iter()
        .sorted().rev()
        .take(2)
        .product::<u64>() as usize
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(&str::to_string).collect::<Vec<String>>();
        let items = lines[1].split_once(": ").unwrap().1.split(", ").filter_map(|s| s.parse::<u64>().ok()).collect::<VecDeque<u64>>();
        let vec = &lines[2].split_once(" = ").unwrap().1.split(" ").map(&str::to_string).collect::<Vec<String>>();
        let operation: Box<dyn Fn(u64) -> u64> = if vec[2] == "old" {
            match vec[1].as_str() {
                "*" => Box::new(move |val: u64| {
                    val * val
                }),
                "+" => Box::new(move |val: u64| {
                    val + val
                }),
                &_ => unimplemented!()
            }
        } else {
            let i = vec[2].parse::<u64>().unwrap();
            match vec[1].as_str() {
                "*" => Box::new(move |val: u64| {
                    val * i
                }),
                "+" => Box::new(move |val: u64| {
                    val + i
                }),
                &_ => unimplemented!()
            }
        };
        let modulus = lines[3].split_once("by ").unwrap().1.parse::<u64>().unwrap();
        let monkey_when_true = lines[4].split_once("monkey ").unwrap().1.parse::<u64>().unwrap();
        let monkey_when_false = lines[5].split_once("monkey ").unwrap().1.parse::<u64>().unwrap();
        let target: Box<dyn Fn(u64) -> u64> = Box::new(move |val| {
            if val % modulus == 0 {
                monkey_when_true
            } else {
                monkey_when_false
            }
        });


        Ok(Monkey {
            items,
            operation,
            target,
            modulus,
        })
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(&r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
".to_string()), 2713310158);
}

