use std::collections::HashMap;
use std::fmt::Display;

fn is_5digits(input: &str) -> bool {
    // 5 digits per number, 3 digits per space, 5 digits per number
    const EXPECTED_LINE_LENGTH: usize = 5 + 3 + 5;
    // +1 for newline
    if input.len() % (EXPECTED_LINE_LENGTH + 1) != 0 {
        return false;
    }
    // check format of first 3 lines
    for line in input.lines().take(3) {
        if line.len() != EXPECTED_LINE_LENGTH {
            return false;
        }
        if line[..5].chars().any(|c| !c.is_digit(10)) {
            return false;
        }
        if &line[5..8] != "   " {
            return false;
        }
        if line[8..].chars().any(|c| !c.is_digit(10)) {
            return false;
        }
    }
    true
}

fn parse_number(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, c| acc * 10 + (*c - b'0') as u64)
}

fn parse_slow(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let l: u64 = parts.next().unwrap().parse().unwrap();
        let r: u64 = parts.next().unwrap().parse().unwrap();
        left.push(l);
        right.push(r);
    }
    (left, right)
}

fn parse_5digits(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let line = line.as_bytes();
        let l: u64 = parse_number(&line[..5]);
        let r: u64 = parse_number(&line[8..]);
        left.push(l);
        right.push(r);
    }
    (left, right)
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    if is_5digits(input) {
        parse_5digits(input)
    } else {
        parse_slow(input)
    }
}

pub fn part1(input: &str) -> impl Display {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>()
}

fn part2_slow(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let mut counts = HashMap::new();
    for v in right.into_iter() {
        *counts.entry(v).or_insert(0) += 1;
    }
    left.into_iter()
        .map(|v| v * counts.get(&v).unwrap_or(&0))
        .sum::<u64>()
}

fn part2_fast(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let mut counts = Vec::new();
    let size = *right.iter().max().unwrap() as usize + 1;
    if counts.try_reserve(size).is_err() {
        return part2_slow(left, right);
    }
    counts.resize(size, 0u64);
    for v in right.into_iter() {
        counts[v as usize] += 1;
    }
    left.into_iter()
        .map(|v| v * counts[v as usize])
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl Display {
    let (left, right) = parse(input);
    part2_fast(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day1.txt");
    const INPUT: &str = include_str!("../inputs/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "11");
        assert_eq!(part1(INPUT).to_string(), "1660292");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "31");
        assert_eq!(part2(INPUT).to_string(), "22776016");
    }
}
