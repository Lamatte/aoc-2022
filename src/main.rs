use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    let mut count = 0;
    let mut last_chars: VecDeque<u8> = VecDeque::new();
    for c in input.as_bytes() {
        count = count + 1;
        if last_chars.len() >= 14 {
            last_chars.pop_front();
        }
        last_chars.push_back(*c);
        if last_chars.iter().unique().count() == 14 {
            return count;
        }
    }
    0
}

#[test]
fn test_data() {
    assert_eq!(execute(r"mjqjpqmgbljsphdztnvjfqwrcgsmlb
".to_string()), 19);
}

