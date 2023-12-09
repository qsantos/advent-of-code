use std::fs::File;
use std::io::{BufRead, BufReader};

fn next_of(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.iter().skip(1).zip(numbers.iter()).map(|(a, b)| a - b).collect();
        numbers.last().unwrap() + next_of(&diffs)
    }
}

fn part1(filename: &str) -> i64 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        sum += next_of(&numbers);
        buf.clear();
    }
    sum
}

fn prev_of(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.iter().skip(1).zip(numbers.iter()).map(|(a, b)| a - b).collect();
        numbers[0] - prev_of(&diffs)
    }
}

fn part2(filename: &str) -> i64 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        sum += prev_of(&numbers);
        buf.clear();
    }
    sum
}

fn main() {
    assert_eq!(part1("example"), 114);
    assert_eq!(part1("input"), 1995001648);

    assert_eq!(part2("example"), 2);
    assert_eq!(part2("input"), 988);
}
