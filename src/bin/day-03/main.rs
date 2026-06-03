use regex::Regex;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("=== Part 1 ===");

    let input = std::fs::read_to_string("src/bin/day-03/input.txt").expect("Unable to read file");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let captures = re.captures_iter(&input);

    let mut sum = 0;

    for capture in captures {
        let a = capture
            .get(1)
            .expect("Unable to find first operand")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse first operand");

        let b = capture
            .get(2)
            .expect("Unable to find second operand")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse second operand");

        let value = a * b;
        sum += value;
    }

    println!("Sum: {sum}");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    position: u32,
    inst: Instruction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2() {
    println!("=== Part 2 ===");

    let input = std::fs::read_to_string("src/bin/day-03/input.txt").expect("Unable to read file");

    let muls = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let muls_captures = muls.captures_iter(&input);

    let dos = Regex::new(r"do\(\)").unwrap();
    let dos_captures = dos.captures_iter(&input);

    let donts = Regex::new(r"don't\(\)").unwrap();
    let donts_captures = donts.captures_iter(&input);

    let mut queue = BinaryHeap::new();

    for capture in muls_captures {
        let a = capture
            .get(1)
            .expect("Unable to find first operand")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse first operand");

        let b = capture
            .get(2)
            .expect("Unable to find second operand")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse second operand");

        queue.push(Node {
            position: capture.get(0).unwrap().start() as u32,
            inst: Instruction::Mul(a, b),
        });
    }

    for capture in dos_captures {
        queue.push(Node {
            position: capture.get(0).unwrap().start() as u32,
            inst: Instruction::Do,
        });
    }

    for capture in donts_captures {
        queue.push(Node {
            position: capture.get(0).unwrap().start() as u32,
            inst: Instruction::Dont,
        });
    }

    let mut _do = true;
    let mut sum = 0;

    for node in queue.into_sorted_vec() {
        match node.inst {
            Instruction::Mul(a, b) => {
                if _do {
                    sum += a * b;
                }
            }
            Instruction::Do => {
                _do = true;
            }
            Instruction::Dont => {
                _do = false;
            }
        }
    }

    println!("Sum: {}", sum);
}
