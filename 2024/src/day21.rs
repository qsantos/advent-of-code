use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;

fn numeric_keypad_shortest(line: &str) -> String {
    let line = line.as_bytes();
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
    let start = (0, 3, 2);
    let mut q = Vec::new();
    q.push(start);
    let mut visited = HashMap::new();
    let mut new_states = Vec::new();
    while let Some(state) = q.pop() {
        let (typed, i, j) = state;
        new_states.clear();
        // left
        if j != 0 && (i, j) != (3, 1) {
            new_states.push(('<', (typed, i, j - 1)));
        }
        // right
        if j != 2 {
            new_states.push(('>', (typed, i, j + 1)));
        }
        // up
        if i != 0 {
            new_states.push(('^', (typed, i - 1, j)));
        }
        // down
        if i != 3 && (i, j) != (2, 0) {
            new_states.push(('v', (typed, i + 1, j)));
        }
        // press
        if line[typed] == KEYPAD[i][j] {
            if typed + 1 == line.len() {
                let mut path = "A".to_string();
                let mut state = (typed, i, j);
                while let Some(&(dir, prev_state)) = visited.get(&state) {
                    path.push(dir);
                    state = prev_state;
                    if state == start {
                        break;
                    }
                }
                path = path.chars().rev().collect();
                return path;
            }
            new_states.push(('A', (typed + 1, i, j)));
        }
        for &(dir, new_state) in new_states.iter() {
            match visited.entry(new_state) {
                Entry::Occupied(_) => continue,
                Entry::Vacant(e) => {
                    e.insert((dir, state));
                    q.push(new_state);
                }
            }
        }
    }
    panic!("No solution found");
}

#[test]
fn numeric_keypad_shortest_test() {
    assert_eq!(numeric_keypad_shortest("029A").len(), "<A^A>^^AvvvA".len());
}

fn directional_keypad_shortest(line: &str) -> String {
    let line = line.as_bytes();
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    const KEYPAD: [&[u8]; 2] = [b" ^A", b"<v>"];
    let start = (0, 0, 2);
    let mut q = Vec::new();
    q.push(start);
    let mut visited = HashMap::new();
    let mut new_states = Vec::new();
    while let Some(state) = q.pop() {
        let (typed, i, j) = state;
        new_states.clear();
        // left
        if (i, j) != (0, 1) && (i, j) != (1, 0) {
            new_states.push(('<', (typed, i, j - 1)));
        }
        // right
        if j != 2 {
            new_states.push(('>', (typed, i, j + 1)));
        }
        // up
        if i != 0 && (i, j) != (1, 0) {
            new_states.push(('^', (typed, i - 1, j)));
        }
        // down
        if i != 1 {
            new_states.push(('v', (typed, i + 1, j)));
        }
        // press
        if line[typed] == KEYPAD[i][j] {
            if typed + 1 == line.len() {
                let mut path = "A".to_string();
                let mut state = (typed, i, j);
                while let Some(&(dir, prev_state)) = visited.get(&state) {
                    path.push(dir);
                    state = prev_state;
                    if state == start {
                        break;
                    }
                }
                path = path.chars().rev().collect();
                return path;
            }
            new_states.push(('A', (typed + 1, i, j)));
        }
        for &(dir, new_state) in new_states.iter() {
            match visited.entry(new_state) {
                Entry::Occupied(_) => continue,
                Entry::Vacant(e) => {
                    e.insert((dir, state));
                    q.push(new_state);
                }
            }
        }
    }
    panic!("No solution found");
}

#[test]
fn directional_keypad_shortest_test() {
    assert_eq!(
        directional_keypad_shortest("<A^A>^^AvvvA").len(),
        "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len()
    );
    assert_eq!(
        directional_keypad_shortest("v<<A>>^A<A>AvA<^AA>A<vAAA>^A").len(),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    );
}

pub fn solve(input: &str, n_robots: usize) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let mut instructions = numeric_keypad_shortest(line);
        for _ in 0..n_robots {
            instructions = directional_keypad_shortest(&instructions);
        }
        let length = instructions.len();
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        total += length * value;
    }
    total
}

pub fn part1(input: &str) -> impl Display {
    solve(input, 3)
}

pub fn part2(input: &str) -> impl Display {
    solve(input, 25)
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
