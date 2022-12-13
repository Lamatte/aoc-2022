use std::cmp::{min, Ordering};
use std::time::Instant;
use Val::{Int, Vector};

mod data;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Val {
    Int(u32),
    Vector(Vec<Val>),
}

fn main() {
    let result = execute(data::get_data());
    println!("{}", result);
    eprintln!("Benching...");
    let start = Instant::now();
    for _ in 0..1000 {
        execute(data::get_data());
    }
    eprintln!("Average elapsed time: {:?}", start.elapsed() / 1000);
}


fn execute(input: Vec<(Val, Val)>) -> usize {
    input.iter()
        .map(|(v1, v2)| v1.cmp(&v2))
        .enumerate()
        //.inspect(|(i, v)| eprintln!("{}: {:?}", i, v))
        .filter(|(_, v)| *v == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

impl Val {
    fn cmp(&self, other: &Val) -> Ordering {
        match (self, other) {
            (Int(i1), Int(i2)) => {
                i1.cmp(i2)
            }
            (Vector(_), Int(i2)) => {
                self.cmp(&Vector(vec![Int(*i2)]))
            }
            (Int(i1), v2) => {
                Vector(vec![(Int(*i1))]).cmp(v2)
            }
            (Vector(v1), Vector(v2)) => {
                let mut res = Ordering::Equal;
                for i in 0..min(v1.len(), v2.len()) {
                    match v1[i].cmp(&v2[i]) {
                        Ordering::Less => {
                            res = Ordering::Less;
                            break;
                        }
                        Ordering::Equal => {
                            // Skip
                        }
                        Ordering::Greater => {
                            res = Ordering::Greater;
                            break;
                        }
                    }
                }
                if res == Ordering::Equal {
                    v1.len().cmp(&v2.len())
                } else {
                    res
                }
            }
        }
    }
}

#[test]
fn case_1() {
    assert_eq!(Vector(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]).cmp(&Vector(vec![Int(1), Int(1), Int(5), Int(1), Int(1)])), Ordering::Less);
}

#[test]
fn case_2() {
    assert_eq!(Vector(vec![Vector(vec![Int(1)]), Vector(vec![Int(2), Int(3), Int(4)])]).cmp(&Vector(vec![Vector(vec![Int(1)]), Int(4)])), Ordering::Less);
}

#[test]
fn case_3() {
    assert_eq!(Vector(vec![Int(9)]).cmp(&Vector(vec![Vector(vec![Int(8), Int(7), Int(6)])])), Ordering::Greater);
}

#[test]
fn case_4() {
    assert_eq!(Vector(vec![Vector(vec![Int(4), Int(4)]), Int(4), Int(4)]).cmp(&Vector(vec![Vector(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)])), Ordering::Less);
}

#[test]
fn case_5() {
    assert_eq!(Vector(vec![Int(7), Int(7), Int(7), Int(7)]).cmp(&Vector(vec![Int(7), Int(7), Int(7)])), Ordering::Greater);
}

#[test]
fn case_6() {
    assert_eq!(Vector(vec![]).cmp(&Vector(vec![Int(3)])), Ordering::Less);
}

#[test]
fn case_7() {
    assert_eq!(Vector(vec![Vector(vec![Vector(vec![])])]).cmp(&Vector(vec![Vector(vec![])])), Ordering::Greater);
}

#[test]
fn case_8() {
    assert_eq!(Vector(vec![Int(1), Vector(vec![Int(2), Vector(vec![Int(3), Vector(vec![Int(4), Vector(vec![Int(5), Int(6), Int(7)])])])]), Int(8), Int(9)]).cmp(&Vector(vec![Int(1), Vector(vec![Int(2), Vector(vec![Int(3), Vector(vec![Int(4), Vector(vec![Int(5), Int(6), Int(0)])])])]), Int(8), Int(9)])), Ordering::Greater);
}

#[test]
fn case_xx() {
    assert_eq!(Vector(vec![Vector(vec![Int(1)]), Vector(vec![Int(2), Int(3), Int(4)])]).cmp(&Vector(vec![Vector(vec![Int(1)]), Int(4)])), Ordering::Less);
}

#[test]
fn case_yy() {
    assert_eq!(Vector(vec![Vector(vec![Int(1)]), Int(4)]).cmp(&Vector(vec![Vector(vec![Int(1)]), Vector(vec![Int(2), Int(3), Int(4)])])), Ordering::Greater);
}

#[test]
fn test_data() {
    assert_eq!(execute(data::get_test_data()), 13);
}
