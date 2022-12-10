use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let result = execute(&input);
    println!("{}", result);
    eprintln!("Benching...");
    let start = Instant::now();
    for _ in 0..1000 {
        execute(&input);
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed()/1000);
}

fn execute(input: &String) -> usize {
    1
}

#[test]
fn test_data() {
    assert_eq!(execute(&r"
".to_string()), 0);
}

