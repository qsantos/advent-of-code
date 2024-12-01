use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let l: u64 = parts.next().unwrap().parse().unwrap();
        let r: u64 = parts.next().unwrap().parse().unwrap();
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
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
}
