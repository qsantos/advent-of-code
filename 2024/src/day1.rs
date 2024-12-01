use std::collections::HashMap;
use std::fmt::Display;

fn analyze_digits(input: &str) -> Option<usize> {
    let line = input.lines().next()?;
    let line_length = line.len();
    let left_digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
    let right_digits = line
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .count();
    if left_digits != right_digits {
        return None;
    }
    let digits = left_digits;
    // check if input is multiple of line length
    // +1 for newline
    if input.len() % (line_length + 1) != 0 {
        // not a regular line length
        return None;
    }
    // check format of first 3 lines
    for line in input.lines().take(3) {
        let line = line.as_bytes();
        if line.len() != line_length {
            // not a regular line length
            return None;
        }
        if line[..digits].iter().any(|c| !c.is_ascii_digit()) {
            // not just digits in the left column
            return None;
        }
        if line[digits..line_length - digits]
            .iter()
            .any(|c| *c != b' ')
        {
            // not just spaces in the middle column
            return None;
        }
        if line[line_length - digits..]
            .iter()
            .any(|c| !c.is_ascii_digit())
        {
            // not just digits in the right column
            return None;
        }
    }
    Some(digits)
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

macro_rules! make_parse_digits {
    ($name:ident, $digits:expr) => {
        fn $name(input: &str) -> (Vec<u64>, Vec<u64>) {
            let mut left = Vec::new();
            let mut right = Vec::new();
            for line in input.lines() {
                let line = line.as_bytes();
                let l: u64 = parse_number(&line[..$digits]);
                let r: u64 = parse_number(&line[line.len() - $digits..]);
                left.push(l);
                right.push(r);
            }
            (left, right)
        }
    };
}

// generate parse functions for 1 to 20 digits
make_parse_digits!(parse_1digit, 1);
make_parse_digits!(parse_2digits, 2);
make_parse_digits!(parse_3digits, 3);
make_parse_digits!(parse_4digits, 4);
make_parse_digits!(parse_5digits, 5);
make_parse_digits!(parse_6digits, 6);
make_parse_digits!(parse_7digits, 7);
make_parse_digits!(parse_8digits, 8);
make_parse_digits!(parse_9digits, 9);
make_parse_digits!(parse_10digits, 10);
make_parse_digits!(parse_11digits, 11);
make_parse_digits!(parse_12digits, 12);
make_parse_digits!(parse_13digits, 13);
make_parse_digits!(parse_14digits, 14);
make_parse_digits!(parse_15digits, 15);
make_parse_digits!(parse_16digits, 16);
make_parse_digits!(parse_17digits, 17);
make_parse_digits!(parse_18digits, 18);
make_parse_digits!(parse_19digits, 19);
make_parse_digits!(parse_20digits, 20);


fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    match analyze_digits(input) {
        Some(1) => parse_1digit(input),
        Some(2) => parse_2digits(input),
        Some(3) => parse_3digits(input),
        Some(4) => parse_4digits(input),
        Some(5) => parse_5digits(input),
        Some(6) => parse_6digits(input),
        Some(7) => parse_7digits(input),
        Some(8) => parse_8digits(input),
        Some(9) => parse_9digits(input),
        Some(10) => parse_10digits(input),
        Some(11) => parse_11digits(input),
        Some(12) => parse_12digits(input),
        Some(13) => parse_13digits(input),
        Some(14) => parse_14digits(input),
        Some(15) => parse_15digits(input),
        Some(16) => parse_16digits(input),
        Some(17) => parse_17digits(input),
        Some(18) => parse_18digits(input),
        Some(19) => parse_19digits(input),
        Some(20) => parse_20digits(input),
        _ => parse_slow(input),
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
