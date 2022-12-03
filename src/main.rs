use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input.lines()
        .filter(|l| l.len() > 0)
        .chunks(3).into_iter()
        .filter_map(|mut c| common_char(c.next().unwrap(), c.next().unwrap(), c.next().unwrap()))
        .map(|c| value(c) as u32)
        .sum::<u32>() as usize
}

fn common_char(s1: &str, s2: &str, s3: &str) -> Option<char> {
    s1.chars()
        .filter(|c| s2.contains(&c.to_string()) && s3.contains(&c.to_string()))
        .last()
}

fn value(c: char) -> u8 {
    let i = c as u8;
    if i < 97 {
        i - 64 + 26
    } else {
        i - 96
    }
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
".to_string()), 70);
}

