use std::collections::HashSet;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut computers = HashSet::new();
    let mut links = HashSet::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        computers.insert(left);
        computers.insert(right);
        links.insert((left, right));
        links.insert((right, left));
    }
    let computers: Vec<_> = computers.into_iter().collect();
    let mut count = 0;
    for (i, &a) in computers.iter().enumerate() {
        for (j, &b) in computers[i + 1..].iter().enumerate() {
            if !links.contains(&(a, b)) {
                continue;
            }
            for &c in computers[i + 1 + j + 1..].iter() {
                if !links.contains(&(a, c)) || !links.contains(&(b, c)) {
                    continue;
                }
                if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
                    continue;
                }
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day23.txt");
    const INPUT: &str = include_str!("../inputs/day23.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "7");
        assert_eq!(part1(INPUT).to_string(), "1175");
    }
}
