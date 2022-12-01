use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input
        .split("\n\n")
        .map(|l| l.split("\n").map(|i| i.parse::<i32>()).filter_map(Result::ok).sum::<i32>())
        .sorted().rev().take(3).sum::<i32>() as usize
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
".to_string()), 45000);
}

