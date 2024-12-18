use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

pub fn fixed_fall(input: &str, size: i64, fallen: usize) -> usize {
    let bytes: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let fallen_bytes = &bytes[..fallen];
    let start = (0, 0);
    let end = (size, size);
    let mut q = VecDeque::new();
    q.push_back((0, start));
    let mut visited = HashSet::new();
    while let Some((steps, pos)) = q.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        if pos == end {
            return steps;
        }
        let (x, y) = pos;
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if !(0..=size).contains(&nx) || !(0..=size).contains(&ny) {
                continue;
            }
            if fallen_bytes.contains(&(nx, ny)) {
                continue;
            }
            q.push_back((steps + 1, (nx, ny)));
        }
    }
    unreachable!("No path found");
}

pub fn part1_example(input: &str) -> impl Display {
    fixed_fall(input, 6, 12)
}

pub fn part1(input: &str) -> impl Display {
    fixed_fall(input, 70, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day18.txt");
    const INPUT: &str = include_str!("../inputs/day18.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1_example(EXAMPLE).to_string(), "22");
        assert_eq!(part1(INPUT).to_string(), "308");
    }
}
