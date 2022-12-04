use std::fs;
use std::ops::Range;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    eprintln!("{}", execute(input));
}

fn execute(input: String) -> usize {
    input.lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.split(","))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(|(s1, s2) | (parse_range(s1), parse_range(s2)))
        .filter(|(s1, s2) | overlap(s1, s2))
        .count()
}

fn overlap(s1: &Range<u8>, s2: &Range<u8>) -> bool {
    (s1.start <= s2.start && s1.end >= s2.end) || (s2.start <= s1.start && s2.end >= s1.end)
}

fn parse_range(s: &str) -> Range<u8> {
    let mut split = s.split("-");
    split.next().unwrap().parse::<u8>().unwrap()..split.next().unwrap().parse::<u8>().unwrap()
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
".to_string()), 2);
}

