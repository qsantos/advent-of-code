use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    const fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }

    const fn dist(&self, other: Self) -> i32 {
        (self.i - other.i).abs() + (self.j - other.j).abs()
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
const NUM_KEYPAD: [&[u8]; 4] = [b"789", b"456", b"123", b" 0A"];
const NUM_START: Point = Point::new(3, 2);

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
const DIR_KEYPAD: [&[u8]; 2] = [b" ^A", b"<v>"];
const DIR_START: Point = Point::new(0, 2);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    typed: i32,
    numerical_pos: Point,
    directional_pos: [Point; N],
}

impl<const N: usize> State<N> {
    const fn new() -> Self {
        Self {
            typed: 0,
            numerical_pos: NUM_START,
            directional_pos: [DIR_START; N],
        }
    }

    fn act(&self, line: &[u8], mut action: u8) -> Option<Self> {
        let mut new_state = self.clone();
        let mut depth = 0;
        while depth < N && action == b'A' {
            let p = self.directional_pos[depth];
            action = DIR_KEYPAD[p.i as usize][p.j as usize];
            depth += 1;
        }
        if action == b'A' {
            // type on numeric keypad
            assert_eq!(depth, N);
            let pos = self.numerical_pos;
            let c = NUM_KEYPAD[pos.i as usize][pos.j as usize];
            if c != line[self.typed as usize] {
                return None;
            }
            new_state.typed += 1;
            Some(new_state)
        } else if depth == N {
            // move robot on numeric keypad
            let (di, dj) = match action {
                b'<' => (0, -1),
                b'>' => (0, 1),
                b'^' => (-1, 0),
                b'v' => (1, 0),
                _ => panic!("Invalid action '{}'", action as char),
            };
            let pos = &mut new_state.numerical_pos;
            let i = pos.i + di;
            if !(0..4).contains(&i) {
                return None;
            }
            let j = pos.j + dj;
            if !(0..3).contains(&j) {
                return None;
            }
            if NUM_KEYPAD[i as usize][j as usize] == b' ' {
                return None;
            }
            new_state.numerical_pos = Point::new(i, j);
            Some(new_state)
        } else {
            // move robot on directional keypad
            let (di, dj) = match action {
                b'<' => (0, -1),
                b'>' => (0, 1),
                b'^' => (-1, 0),
                b'v' => (1, 0),
                _ => panic!("Invalid action '{}'", action as char),
            };
            let pos = new_state.directional_pos[depth];
            let i = pos.i + di;
            if !(0..2).contains(&i) {
                return None;
            }
            let j = pos.j + dj;
            if !(0..3).contains(&j) {
                return None;
            }
            if DIR_KEYPAD[i as usize][j as usize] == b' ' {
                return None;
            }
            new_state.directional_pos[depth] = Point::new(i, j);
            Some(new_state)
        }
    }
}

#[test]
fn test_state_act() {
    fn evaluate_state<const N: usize>(line: &[u8], actions: &[u8]) {
        let mut state: State<N> = State::new();
        for &action in actions {
            state = state.act(line, action).unwrap();
        }
        assert_eq!(state.typed, line.len() as i32);
    }
    evaluate_state::<0>(b"029A", b"<A^A>^^AvvvA");
    evaluate_state::<0>(b"029A", b"<A^A^>^AvvvA");
    evaluate_state::<0>(b"029A", b"<A^A^^>AvvvA");
    evaluate_state::<1>(b"029A", b"v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    evaluate_state::<2>(
        b"029A",
        b"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CostState<const N: usize> {
    cost: i32, // accumulated cost so far
    state: State<N>,
}

impl<const N: usize> CostState<N> {
    fn new() -> Self {
        Self {
            cost: 0,
            state: State::new(),
        }
    }

    /// Lower-bound estimate of the total cost
    fn f(&self) -> i32 {
        self.g() + self.h()
    }

    /// Accumulated cost so far
    fn g(&self) -> i32 {
        self.cost
    }

    /// Lower-bound estimate of the remaining cost to reach the goal
    fn h(&self) -> i32 {
        -self.state.typed
            + self.state.numerical_pos.dist(NUM_START)
            + self
                .state
                .directional_pos
                .iter()
                .map(|pos| pos.dist(DIR_START))
                .sum::<i32>()
    }
}

impl<const N: usize> PartialOrd for CostState<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for CostState<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f().cmp(&self.f())
    }
}

pub fn type_on_keypad<const N: usize>(line: &str) -> usize {
    let line = line.as_bytes();
    let mut q: BinaryHeap<CostState<N>> = BinaryHeap::new();
    q.push(CostState::new());
    let mut visited = HashSet::new();
    while let Some(CostState { cost, state }) = q.pop() {
        if !visited.insert(state.clone()) {
            continue;
        }
        for &action in b"<>^vA" {
            let new_cost = cost + 1;
            if let Some(new_state) = state.act(line, action) {
                if new_state.typed == line.len() as i32 {
                    return new_cost as usize;
                }
                q.push(CostState {
                    cost: new_cost,
                    state: new_state,
                });
            }
        }
    }
    panic!("No solution found");
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypad::<2>(line);
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        total += length * value;
    }
    total
}

pub fn part2(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypad::<25>(line);
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        total += length * value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day21.txt");
    const INPUT: &str = include_str!("../inputs/day21.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "126384");
        assert_eq!(part1(INPUT).to_string(), "162740");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).to_string(), "");
    }
}
