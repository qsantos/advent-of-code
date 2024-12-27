use std::collections::HashMap;
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

// NOTES:
// - When typing a number, all the robots have to be pressing "A". This ensures that typing the
//   numbers is is independent from the others.
// - Similarly, the state at an intermediate layer only depends on what has been typed so far.
// - When going from one button to another, the shortest path always uses one of up/down and
//   one of left/right. Additional buttons are never needed, even to optimize the presses of
//   higher-level robots.
// - The optimal series of up/down and left/right presses is either, all up/down, then all
//   left/right, then back to press A, or all left/right, then all up/down, then back to press
//   A.
// - The two approaches are always equivalent in terms of number of presses, since moves are
//   time-symmetric. For instance:
//   - A, move to ">", press 3 times, move to "^", press 2 times, move to "A", press "A".
//   - A, move to "^", press 2 times, move to ">", press 3 times, move to "A", press "A".
// The first two points allow us to take a greedy approach at each level. The last two points
// allow us to efficiently remember the needed information at each level to find the number of
// required steps.

// For instance, to press '5', we can do '<^^A'. For this, we need to do the following on the
// level above:
// - 'v' once
// - '<' twice
// - 'A' once
// - '>' once
// - '^' once
// - 'A' twice
// - '>' once
// - 'A' once
// In other words, '<^^A' is mapped to 'v<A', '>^AA', '>A'.

// di > 0 corresponds to the number of v
// di < 0 corresponds to the number of ^
// dj > 0 corresponds to the number of >
// dj < 0 corresponds to the number of <
// a > 0 corresponds to the number of A
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Action {
    i: i8,
    j: i8,
    di: i8,
    dj: i8,
    a: i8,
}

impl Action {
    fn from_line(line: &str) -> Vec<Action> {
        let mut ret = Vec::new();
        let mut i = 3;
        let mut j = 2;
        for &c in line.as_bytes() {
            let (ti, tj) = NUM_KEYPAD
                .iter()
                .enumerate()
                .find_map(|(di, row)| {
                    row.iter()
                        .position(|&x| x == c)
                        .map(|dj| (di as i8, dj as i8))
                })
                .unwrap();
            let di = ti - i;
            let dj = tj - j;
            let a = 1;
            use std::cmp::Ordering;
            let moves = match (di.cmp(&0), dj.cmp(&0)) {
                (Ordering::Less, Ordering::Less) => {
                    // Pressing <^A needs ↙←A↗A→A
                    // In turn, these three actions require 6, 4 and 2 moves respectively
                    // Pressing ^<A needs ←A↙A→↗A
                    // In turn, these three actions require 6, 6 and 4 moves respectively
                    // The first is more efficient.
                    // We can only use this when it would not go through the lower left corner
                    if i != 3 || tj != 0 {
                        "<^A"
                    } else {
                        "^<A"
                    }
                }
                (Ordering::Less, Ordering::Equal) => {
                    // No choice
                    "^A"
                }
                (Ordering::Less, Ordering::Greater) => {
                    // Pressing >^A needs ↓A↖A→A
                    // In turn, these three actions require 2, 6 and 2 moves respectively
                    // Pressing ^>A needs ←A↘A↑A
                    // In turn, these three actions require 6, 4 and 2 moves respectively
                    // The first is more efficient.
                    // This never goes through the lower left corner
                    "^>A"
                    // TODO: wrong
                }
                (Ordering::Equal, Ordering::Less) => {
                    // No choice
                    "<A"
                }
                (Ordering::Equal, Ordering::Equal) => {
                    // No choice
                    "A"
                }
                (Ordering::Equal, Ordering::Greater) => {
                    // No choice
                    ">A"
                }
                (Ordering::Greater, Ordering::Less) => {
                    // <vA would need ↙←A→A↑A
                    // In turn, these three actions require 6, 2 and 2 moves respectively
                    // v<A would need ↙A←A→↗A
                    // In turn, these three actions require 6, 6 and 4 moves respectively
                    // The first is more efficient.
                    // This never goes through the lower right corner
                    "<vA"
                }
                (Ordering::Greater, Ordering::Equal) => {
                    // No choice
                    "vA"
                }
                (Ordering::Greater, Ordering::Greater) => {
                    // v>A would need ↙A→A↑A
                    // In turn, these three actions require 6, 2 and 2 moves respectively
                    // >vA would need ↓A←A↗A
                    // In turn, these three actions require 2, 6 and 4 moves respectively
                    // The first is more efficient.
                    // We can only do this when it would not go through the lower left corner
                    if j != 0 || ti != 3 {
                        "v>A"
                    } else {
                        ">vA"
                    }
                }
            };
            let (mut dir_i, mut dir_j) = (0, 2);
            for &move_ in moves.as_bytes() {
                let (dir_ti, dir_tj) = DIR_KEYPAD
                    .iter()
                    .enumerate()
                    .find_map(|(di, row)| {
                        row.iter()
                            .position(|&x| x == move_)
                            .map(|dj| (di as i8, dj as i8))
                    })
                    .unwrap();
                let dir_di = dir_ti - dir_i;
                let dir_dj = dir_tj - dir_j;
                let a = match move_ {
                    b'^' => -di,
                    b'v' => di,
                    b'<' => -dj,
                    b'>' => dj,
                    b'A' => a,
                    _ => unreachable!(),
                };
                ret.push(Action {
                    i: dir_i,
                    j: dir_j,
                    di: dir_di,
                    dj: dir_dj,
                    a,
                });
                (dir_i, dir_j) = (dir_ti, dir_tj);
            }
            assert_eq!((dir_i, dir_j), (0, 2));
            (i, j) = (ti, tj);
        }
        ret
    }

    /// Return the higher level actions needed to perform the given action.
    fn perform(&self) -> Vec<Action> {
        let &Action { i, j, di, dj, a } = self;
        let mut ret = Vec::new();
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        use std::cmp::Ordering;
        let moves = match (di.cmp(&0), dj.cmp(&0)) {
            (Ordering::Less, Ordering::Less) => {
                // <^A would need ↙←A↗A→A
                // In turn, these three actions require 6, 4 and 2 moves respectively
                // ^<A would need ←A↙A→↗A
                // In turn, these three actions require 6, 6 and 4 moves respectively
                // The first is more efficient.
                // This never goes through the upper left corner
                "<^A"
            }
            (Ordering::Less, Ordering::Equal) => {
                // No choice
                "^A"
            }
            (Ordering::Less, Ordering::Greater) => {
                // >^A would need ↓A↖A→A
                // In turn, these three actions require 2, 6 and 2 moves respectively
                // ^>A would need ←A↘A↑A
                // In turn, these three actions require 6, 4 and 2 moves respectively
                // The first is more efficient.
                // We can only do this when it would not go through the upper left corner
                if j != 0 {
                    "^>A"
                } else {
                    ">^A"
                }
                //TODO: wrong
            }
            (Ordering::Equal, Ordering::Less) => {
                // No choice
                "<A"
            }
            (Ordering::Equal, Ordering::Equal) => {
                // No choice
                "A"
            }
            (Ordering::Equal, Ordering::Greater) => {
                // No choice
                ">A"
            }
            (Ordering::Greater, Ordering::Less) => {
                // <vA would need ↙←A→A↗A
                // In turn, these three actions require 6, 2 and 4 moves respectively
                // v<A would need ↙A←A→↗A
                // In turn, these three actions require 6, 6 and 4 moves respectively
                // The first is more efficient.
                // We can only do this when it would not go through the upper left corner
                if j + dj != 0 {
                    "<vA"
                } else {
                    "v<A"
                }
            }
            (Ordering::Greater, Ordering::Equal) => {
                // No choice
                "vA"
            }
            (Ordering::Greater, Ordering::Greater) => {
                // v>A would need ↙A→A↑A
                // In turn, these three actions require 6, 2 and 2 moves respectively
                // >vA would need ↓A←A↗A
                // In turn, these three actions require 2, 6 and 4 moves respectively
                // The first is more efficient.
                // This never goes through the upper left corner
                "v>A"
            }
        };
        let (mut dir_i, mut dir_j) = (0, 2);
        for &move_ in moves.as_bytes() {
            let (dir_ti, dir_tj) = DIR_KEYPAD
                .iter()
                .enumerate()
                .find_map(|(di, row)| {
                    row.iter()
                        .position(|&x| x == move_)
                        .map(|dj| (di as i8, dj as i8))
                })
                .unwrap();
            let dir_di = dir_ti - dir_i;
            let dir_dj = dir_tj - dir_j;
            let a = match move_ {
                b'^' => -di,
                b'v' => di,
                b'<' => -dj,
                b'>' => dj,
                b'A' => a,
                _ => unreachable!(),
            };
            ret.push(Action {
                i: dir_i,
                j: dir_j,
                di: dir_di,
                dj: dir_dj,
                a,
            });
            (dir_i, dir_j) = (dir_ti, dir_tj);
        }
        assert_eq!((dir_i, dir_j), (0, 2));
        assert_eq!(ret.iter().map(|action| action.di).sum::<i8>(), 0);
        assert_eq!(ret.iter().map(|action| action.dj).sum::<i8>(), 0);
        assert!(ret.iter().all(|action| action.a > 0));
        ret
    }

    fn presses(&self) -> usize {
        (self.di.abs() + self.dj.abs() + self.a.abs()) as usize
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Action { i, j, di, dj, a } = self;
        for _ in 0..di.abs() {
            if di > 0 {
                write!(f, "v")?;
            } else {
                write!(f, "^")?;
            }
        }
        for _ in 0..dj.abs() {
            if dj > 0 {
                write!(f, ">")?;
            } else {
                write!(f, "<")?;
            }
        }
        for _ in 0..a.abs() {
            write!(f, "A")?;
        }
        Ok(())
    }
}

pub fn type_on_keypad(line: &str, n_robots: usize) -> usize {
    let mut actions = Action::from_line(line);
    println!("{:?}", actions);
    for _ in 1..n_robots {
        let mut new_actions = Vec::new();
        for action in &actions {
            new_actions.extend(action.perform());
        }
        actions = new_actions;
        //println!(
        //    "{:?} ({})",
        //    actions,
        //    actions
        //        .iter()
        //        .map(|&action| action.presses())
        //        .sum::<usize>()
        //);
    }
    actions.into_iter().map(|action| action.presses()).sum()
}

pub fn type_on_keypad_fast(line: &str, n_robots: usize) -> usize {
    let mut actions = Action::from_line(line);
    let mut counts = HashMap::new();
    for action in actions {
        *counts.entry(action).or_insert(0) += 1;
    }
    for _ in 1..n_robots {
        let mut new_counts = HashMap::new();
        for (&action, &count) in &counts {
            for action in action.perform() {
                *new_counts.entry(action).or_insert(0) += count;
            }
        }
        counts = new_counts;
    }
    counts
        .into_iter()
        .map(|(action, count)| action.presses() * count)
        .sum()
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypad(line, 2);
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        println!("{length} * {value} = {}", length * value);
        total += length * value;
    }
    total
}

pub fn part2(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypad_fast(line, 25);
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
        assert_eq!(part2(INPUT).to_string(), "203640915832208");
    }
}
