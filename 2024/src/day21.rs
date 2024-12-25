use std::collections::{BinaryHeap, HashMap, HashSet};
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
            ret.push(Action {
                di: ti - i,
                dj: tj - j,
                a: 1,
            });
            i = ti;
            j = tj;
        }
        ret
    }

    /// Return the higher level actions needed to perform the given action.
    fn perform(&self) -> Vec<Action> {
        let &Action { di, dj, a } = self;
        let mut ret = Vec::new();
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        if di < 0 {
            // Move from A to ^: <, followed by -di presses of A
            ret.push(Action {
                di: 0,
                dj: -1,
                a: -di,
            });
            if dj < 0 {
                // Move from ^ to <: v<, followed by -dj presses of A
                ret.push(Action {
                    di: 1,
                    dj: -1,
                    a: -dj,
                });
                // Move from < to A: >>^, followed by a presses of A
                ret.push(Action { di: -1, dj: 2, a });
            } else if dj == 0 {
                // Move from ^ to A: >, followed by a presses of A
                ret.push(Action { di: 0, dj: 1, a });
            } else {
                // Move from ^ to >: v>, followed by dj presses of A
                ret.push(Action {
                    di: 1,
                    dj: 1,
                    a: dj,
                });
                // Move from > to A: ^, followed by a presses of A
                ret.push(Action { di: -1, dj: 0, a });
            }
        } else if di == 0 {
            if dj < 0 {
                // Move from A to <: v<<, followed by -dj presses of A
                ret.push(Action {
                    di: 1,
                    dj: -2,
                    a: -dj,
                });
                // Move from < to A: >>^, followed by a presses of A
                ret.push(Action { di: -1, dj: 2, a });
            } else if dj == 0 {
                // Don't move, followed by a presses of A
                ret.push(Action { di: 0, dj: 0, a });
            } else {
                // Move from A to >: v, followed by dj presses of A
                ret.push(Action {
                    di: 1,
                    dj: 0,
                    a: dj,
                });
                // Move from > to A: ^, followed by a presses of A
                ret.push(Action { di: -1, dj: 0, a });
            }
        } else {
            // Move from A to v: v<, followed by -di presses of A
            ret.push(Action {
                di: 1,
                dj: -1,
                a: di,
            });
            if dj < 0 {
                // Move from v to <: <, followed by -dj presses of A
                ret.push(Action {
                    di: 0,
                    dj: -1,
                    a: -dj,
                });
                // Move from < to A: >>^, followed by a presses of A
                ret.push(Action { di: -1, dj: 2, a });
            } else if dj == 0 {
                // Move from v to A: >^, followed by a presses of A
                ret.push(Action { di: -1, dj: 1, a });
            } else {
                // Move from v to >: >, followed by dj presses of A
                ret.push(Action {
                    di: 0,
                    dj: 1,
                    a: dj,
                });
                // Move from > to A: ^, followed by a presses of A
                ret.push(Action { di: -1, dj: 0, a });
            }
        }
        ret
    }

    fn presses(&self) -> usize {
        (self.di.abs() + self.dj.abs() + self.a.abs()) as usize
    }
}

// Fails on last line of example:

// [^A,                 ^^<<A,                                     >>A,               vvvA]
// [<A,         >A,     <AA,         v<AA,           ^>>A,         vAA,       ^A,     v<AAA,           ^>A]
// [v<<A, ^>>A, vA, ^A, v<<A, ^>>AA, v<A, <A, ^>>AA, <A, v>AA, ^A, v<A, ^>AA, <A, >A, v<A, <A, ^>>AAA, <A, v>A, ^A]
//
// [^A,                 <<^^A,                                 >>A,               vvvA
// [<A,         >A,     v<<AA,           >^AA,         >A,     vAA,       ^A,     <vAAA,           >^A]
// [<v<A, >>^A, vA, ^A, <vA, <AA, >>^AA, vA, <^A, >AA, vA, ^A, <vA, >^AA, <A, >A, <v<A, >A, >^AAA, vA, <^A, >A]

// [^^<<A,
// [<AA,         v<AA,           ^>>A
// [v<<A, ^>>AA, v<A, <A, ^>>AA, <A, v>AA, ^A
//  9            10              8
//
// [<<^^A,
// [v<<AA,           >^AA,         >A
// [<vA, <AA, >>^AA, vA, <^A, >AA, vA, ^A
//  11               8             4

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Action { di, dj, a } = self;
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

// [^A, ^^<<A, >>A, vvvA]
// [<A, >A, <AA, v<AA, ^>>A, vAA, ^A, v<AAA, ^>A]
// [v<<A, ^>>A, vA, ^A, v<<A, ^>>AA, v<A, <A, ^>>AA, <A, v>AA, ^A, v<A, ^>AA, <A, >A, v<A, <A, ^>>AAA, <A, v>A, ^A]

pub fn type_on_keypad(line: &str, n_robots: usize) -> usize {
    let mut actions = Action::from_line(line);
    println!("{:?}", actions);
    for _ in 0..n_robots {
        let mut new_actions = Vec::new();
        for action in &actions {
            new_actions.extend(action.perform());
        }
        actions = new_actions;
        println!("{:?}", actions);
    }
    actions.into_iter().map(|action| action.presses()).sum()
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
        let length = type_on_keypad(line, 25);
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
