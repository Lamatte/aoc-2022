use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> usize {
    1
}

#[test]
fn test_data() {
    assert_eq!(execute(r"
".to_string()), 0);
}

