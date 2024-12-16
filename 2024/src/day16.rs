use std::collections::{BinaryHeap, HashMap, HashSet};
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
}

impl State {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j, di: 0, dj: 1 }
    }

    fn rev(&self) -> Self {
        Self {
            i: self.i,
            j: self.j,
            di: -self.di,
            dj: -self.dj,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StateWithScore {
    score: i32,
    state: State,
}

// NOTE: inverted ordering to make it a min-heap
impl PartialOrd for StateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl Ord for StateWithScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn dijkstra(grid: &[&[u8]], start: &[State]) -> HashMap<State, i32> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut q = BinaryHeap::new();
    for state in start {
        q.push(StateWithScore {
            score: 0,
            state: state.clone(),
        });
    }
    let mut visited = HashMap::new();
    while let Some(StateWithScore { score, state }) = q.pop() {
        let State { i, j, di, dj } = state;
        if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
            continue;
        }
        let c = grid[i as usize][j as usize];
        if c == b'#' {
            continue;
        }
        if visited.contains_key(&state) {
            continue;
        }
        visited.insert(state.clone(), score);
        q.push(StateWithScore {
            score: score + 1,
            state: State {
                i: i + di,
                j: j + dj,
                di,
                dj,
            },
        });
        q.push(StateWithScore {
            score: score + 1000,
            state: State {
                i,
                j,
                di: dj,
                dj: -di,
            },
        });
        q.push(StateWithScore {
            score: score + 1000,
            state: State {
                i,
                j,
                di: -dj,
                dj: di,
            },
        });
    }
    visited
}

fn get_best_score(scores: &HashMap<State, i32>, end: (i32, i32)) -> i32 {
    scores
        .iter()
        .filter_map(|(state, &score)| {
            if (state.i, state.j) == end {
                Some(score)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let start = find_pos(&grid, b'S');
    let end = find_pos(&grid, b'E');
    let start = State::new(start.0, start.1);
    let scores = dijkstra(&grid, &[start]);
    get_best_score(&scores, end)
}

pub fn part2(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let orientations = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    // forward
    let start = find_pos(&grid, b'S');
    let start = State::new(start.0, start.1);
    let forward = dijkstra(&grid, &[start]);
    // backward
    let end = find_pos(&grid, b'E');
    let ends: Vec<_> = orientations
        .iter()
        .map(|&(di, dj)| State {
            i: end.0,
            j: end.1,
            di,
            dj,
        })
        .collect();
    let backward = dijkstra(&grid, &ends);
    // count best spots
    let best_score = get_best_score(&forward, end);
    let mut spots = HashSet::new();
    for (state, f) in forward {
        let b = backward[&state.rev()];
        if f + b == best_score {
            spots.insert((state.i, state.j));
        }
    }
    spots.len()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1).to_string(), "45");
        assert_eq!(part2(EXAMPLE2).to_string(), "64");
        assert_eq!(part2(INPUT).to_string(), "");
    }
}
