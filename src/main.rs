use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    eprintln!("{}", execute(input));
    eprintln!("Elapsed time: {:?}", start.elapsed());
}

const LEN: usize = 14;

fn execute(input: String) -> usize {
    for i in LEN..input.len() {
        if input[i - LEN..i].as_bytes().iter().unique().count() == LEN {
            return i;
        }
    }
    0
}

#[test]
fn test_data() {
    assert_eq!(execute(r"mjqjpqmgbljsphdztnvjfqwrcgsmlb
".to_string()), 19);
}

