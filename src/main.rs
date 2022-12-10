use std::fs;
use std::time::Instant;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Computer {
    cycle: i32,
    register: i32,
}

impl Computer {
    fn new() -> Computer {
        Computer { cycle: 0, register: 1 }
    }

    fn execute(&mut self, instruction_value: i32) {
        self.register += instruction_value;
        self.cycle += 1;
    }

    fn pixel(&self) -> char {
        if self.current_cycle_is_on_cursor() { '#' } else { '.' }
    }

    fn current_cycle_is_on_cursor(&self) -> bool {
        self.cycle % 40 >= self.register - 1 && self.cycle % 40 <= self.register + 1
    }
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Could not read file");
    let start = Instant::now();
    let result = execute(input);
    eprintln!("Elapsed time: {:?}", start.elapsed());
    println!("{}", result);
}

fn execute(input: String) -> String {
    instructions(input).into_iter()
        .scan(Computer::new(), |computer, instruction_value| {
            let current_computer = computer.clone();
            computer.execute(instruction_value);
            Some(current_computer)
        })
        .map(|computer| computer.pixel())
        .chunks(40).into_iter()
        .map(|line_chunk| line_chunk.into_iter().collect::<String>())
        .join("\n")
}

fn instructions(input: String) -> Vec<i32> {
    input.lines()
        .flat_map(|line| parse_instruction(line))
        .collect()
}

fn parse_instruction(line: &str) -> Vec<i32> {
    if line == "noop" {
        vec![0]
    } else {
        let split = line.split_once(' ').unwrap();
        vec![0, split.1.parse::<i32>().unwrap()]
    }
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
".to_string()), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
}

