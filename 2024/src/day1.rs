use std::collections::BTreeMap;
use std::fmt::Display;

fn parse_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
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

pub fn part1(input: &str) -> impl Display {
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl Display {
    let (left, right) = parse_lists(input);
    let mut counts = BTreeMap::new();
    for v in right.into_iter() {
        *counts.entry(v).or_insert(0) += 1;
    }
    left.into_iter()
        .map(|v| v * counts.get(&v).unwrap_or(&0))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;

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
