use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

pub fn type_on_keypad(line: &str) -> usize {
    let line = line.as_bytes();
    let start1 = (3, 2);
    let start2 = (0, 2);
    let start3 = (0, 2);
    let start = (0, start1, start2, start3);
    let mut q = VecDeque::new();
    q.push_back((0, start));
    let mut visited = HashSet::new();
    while let Some((length, state)) = q.pop_front() {
        if !visited.insert(state) {
            continue;
        }
        let (typed, pos1, pos2, pos3) = state;
        // We type on:
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        // left
        if pos3 != (0, 1) && pos3 != (1, 0) {
            let new_pos3 = (pos3.0, pos3.1 - 1);
            let new_state = (typed, pos1, pos2, new_pos3);
            q.push_back((length + 1, new_state));
        }
        // right
        if pos3.1 != 2 {
            let new_pos3 = (pos3.0, pos3.1 + 1);
            let new_state = (typed, pos1, pos2, new_pos3);
            q.push_back((length + 1, new_state));
        }
        // up
        if pos3.0 != 0 && pos3 != (1, 0) {
            let new_pos3 = (pos3.0 - 1, pos3.1);
            let new_state = (typed, pos1, pos2, new_pos3);
            q.push_back((length + 1, new_state));
        }
        // down
        if pos3.0 != 1 {
            let new_pos3 = (pos3.0 + 1, pos3.1);
            let new_state = (typed, pos1, pos2, new_pos3);
            q.push_back((length + 1, new_state));
        }
        // A
        // Robot 3 types on:
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        match pos3 {
            (1, 0) => {
                // left
                if pos2 != (0, 1) && pos2 != (1, 0) {
                    let new_pos2 = (pos2.0, pos2.1 - 1);
                    let new_state = (typed, pos1, new_pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (1, 2) => {
                // right
                if pos2.1 != 2 {
                    let new_pos2 = (pos2.0, pos2.1 + 1);
                    let new_state = (typed, pos1, new_pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (0, 1) => {
                // up
                if pos2.0 != 0 && pos2 != (1, 0) {
                    let new_pos2 = (pos2.0 - 1, pos2.1);
                    let new_state = (typed, pos1, new_pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (1, 1) => {
                // down
                if pos2.0 != 1 {
                    let new_pos2 = (pos2.0 + 1, pos2.1);
                    let new_state = (typed, pos1, new_pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (0, 2) => {
                // A
            }
            _ => panic!("Invalid pos3: {:?}", pos3),
        }
        // Robot 2 types on:
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        match pos2 {
            (1, 0) => {
                // left
                if pos1.1 != 0 && pos1 != (3, 1) {
                    let new_pos1 = (pos1.0, pos1.1 - 1);
                    let new_state = (typed, new_pos1, pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (1, 2) => {
                // right
                if pos1.1 != 2 {
                    let new_pos1 = (pos1.0, pos1.1 + 1);
                    let new_state = (typed, new_pos1, pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (0, 1) => {
                // up
                if pos1.0 != 0 {
                    let new_pos1 = (pos1.0 - 1, pos1.1);
                    let new_state = (typed, new_pos1, pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (1, 1) => {
                // down
                if pos1.0 != 3 && pos1 != (2, 0) {
                    let new_pos1 = (pos1.0 + 1, pos1.1);
                    let new_state = (typed, new_pos1, pos2, pos3);
                    q.push_back((length + 1, new_state));
                }
                continue;
            }
            (0, 2) => {
                // A
            }
            _ => panic!("Invalid pos1: {:?}", pos1),
        }
        // Robot 1 types on:
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        const KEYPAD: [&[u8]; 4] = [b"789", b"456", b"123", b" 0A"];
        if KEYPAD[pos1.0][pos1.1] != line[typed] {
            continue;
        }
        if typed + 1 == line.len() {
            return length + 1;
        }
        let new_state = (typed + 1, pos1, pos2, pos3);
        q.push_back((length + 1, new_state));
    }
    panic!("No solution found");
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypad(line);
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
}
