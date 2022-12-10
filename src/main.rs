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
    input.lines().flat_map(|line| {
        if line == "noop" {
            vec![0]
        } else {
            let x = line.split_once(' ').unwrap();
            vec![0, x.1.parse::<i32>().unwrap()]
        }
    })
        .scan((1, 1), |(cycle, register), val| {
            *register += val;
            *cycle += 1;
            Some((*cycle, *register))
        })
        .inspect(|(cycle, register)| eprintln!("{} - {}", cycle, register))
        .filter(|(cycle, register)| *cycle == 20 || (20 + *cycle) % 40 == 0)
        //.inspect(|(cycle, register)| eprintln!("{} - {}", cycle, register))
        .map(|(cycle, register)| cycle*register)
        .sum::<i32>() as usize
}

#[test]
fn test_data() {
    assert_eq!(execute(r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
".to_string()), 13140);
}

