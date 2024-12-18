use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn fixed_fall(bytes: &[(i64, i64)], size: i64) -> Option<usize> {
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
            return Some(steps);
        }
        let (x, y) = pos;
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if !(0..=size).contains(&nx) || !(0..=size).contains(&ny) {
                continue;
            }
            if bytes.contains(&(nx, ny)) {
                continue;
            }
            q.push_back((steps + 1, (nx, ny)));
        }
    }
    None
}

fn first_blocker(bytes: &[(i64, i64)], size: i64) -> (i64, i64) {
    // note would be nice to have partition_point on range, not just on slice
    let mut start = 0;
    let mut stop = bytes.len();
    while stop - start > 1 {
        let mid = (start + stop) / 2;
        if fixed_fall(&bytes[..mid], size).is_none() {
            stop = mid;
        } else {
            start = mid;
        }
    }
    bytes[start]
}

pub fn part1_example(input: &str) -> impl Display {
    let bytes = parse(input);
    fixed_fall(&bytes[..12], 6).unwrap()
}

pub fn part1(input: &str) -> impl Display {
    let bytes = parse(input);
    fixed_fall(&bytes[..1024], 70).unwrap()
}

pub fn part2_example(input: &str) -> impl Display {
    let bytes = parse(input);
    let (x, y) = first_blocker(&bytes, 6);
    format!("{x},{y}")
}

pub fn part2(input: &str) -> impl Display {
    let bytes = parse(input);
    let (x, y) = first_blocker(&bytes, 70);
    format!("{x},{y}")
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

    #[test]
    fn test_part2() {
        assert_eq!(part2_example(EXAMPLE).to_string(), "6,1");
        assert_eq!(part2(INPUT).to_string(), "46,28");
    }
}
