use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;

fn find_pos(grid: &[&[u8]], target: u8) -> (i32, i32) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == target {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("Target not found");
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    score: i32,
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
}

impl State {
    fn new(i: i32, j: i32) -> Self {
        Self {
            score: 0,
            i,
            j,
            di: 0,
            dj: 1,
        }
    }
}

// NOTE: inverted ordering to make it a min-heap
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

pub fn part1(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let start = find_pos(&grid, b'S');
    let mut q = BinaryHeap::new();
    let (i, j) = start;
    q.push(State::new(i, j));
    let mut visited = HashSet::new();
    while let Some(state) = q.pop() {
        let State {
            score,
            i,
            j,
            di,
            dj,
        } = state;
        if !visited.insert((i, j, di, dj)) {
            continue;
        }
        if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
            continue;
        }
        let c = grid[i as usize][j as usize];
        if c == b'E' {
            return score;
        } else if c == b'.' || c == b'S' {
            q.push(State {
                score: score + 1,
                i: i + di,
                j: j + dj,
                di,
                dj,
            });
            q.push(State {
                score: score + 1000,
                i,
                j,
                di: dj,
                dj: -di,
            });
            q.push(State {
                score: score + 1000,
                i,
                j,
                di: -dj,
                dj: di,
            });
        } else if c == b'#' {
        } else {
            panic!("Invalid cell '{}'", c as char);
        }
    }
    panic!("No path found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day16-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day16-2.txt");
    const INPUT: &str = include_str!("../inputs/day16.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "7036");
        assert_eq!(part1(EXAMPLE2).to_string(), "11048");
        assert_eq!(part1(INPUT).to_string(), "83444");
    }
}
