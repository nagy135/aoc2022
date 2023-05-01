#[allow(dead_code)]
use std::fs;

fn main() {
    let result = day2();
    println!("result: {}", result);
}

fn day1() -> u32 {
    let contents = fs::read_to_string("assets/real/day1").expect("Missing file");

    let mut buckets: Vec<u32> = vec![];
    let mut current_bucket: u32 = 0;
    for line in contents.lines() {
        match line.is_empty() {
            true => {
                buckets.push(current_bucket);
                current_bucket = 0;
            }
            false => {
                current_bucket += line.parse::<u32>().expect("Line cant be parsed");
            }
        }
    }
    buckets
        .iter()
        .max()
        .expect("Nothing returned as max value")
        .to_owned()
}

fn day1_part2() -> u32 {
    let contents = fs::read_to_string("assets/real/day1").expect("Missing file");

    let mut buckets: Vec<u32> = vec![];
    let mut current_bucket: u32 = 0;
    for line in contents.lines() {
        match line.is_empty() {
            true => {
                buckets.push(current_bucket);
                current_bucket = 0;
            }
            false => {
                current_bucket += line.parse::<u32>().expect("Line cant be parsed");
            }
        }
    }

    buckets.push(current_bucket);

    buckets.sort();
    buckets.iter().rev().take(3).sum()
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn compare(&self, other: &Move) -> u32 {
        match &self {
            Move::Rock => match other {
                Move::Rock => 1 + 3,
                Move::Paper => 1 + 0,
                Move::Scissors => 1 + 6,
            },
            Move::Paper => match other {
                Move::Rock => 2 + 6,
                Move::Paper => 2 + 3,
                Move::Scissors => 2 + 0,
            },
            Move::Scissors => match other {
                Move::Rock => 3 + 0,
                Move::Paper => 3 + 6,
                Move::Scissors => 3 + 3,
            },
        }
    }
}

fn day2() -> u32 {
    let contents = fs::read_to_string("assets/real/day2").expect("Missing file");
    let mut sum = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let theirs = match parts[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("unknown option"),
        };
        let ours = match parts[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("unknown option"),
        };
        sum += ours.compare(&theirs);
    }
    sum
}
