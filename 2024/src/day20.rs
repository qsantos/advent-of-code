use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::fmt::Display;

fn find(grid: &[&[u8]], c: u8) -> (isize, isize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("Did not find '{}'", c as char);
}

fn dijkstra(grid: &[&[u8]], start: (isize, isize), end: (isize, isize)) -> HashMap<(isize, isize), isize> {
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut visited = HashMap::new();
    while let Some((steps, (i, j))) = queue.pop_front() {
        if let Entry::Vacant(entry) = visited.entry((i, j)) {
            entry.insert(steps);
        } else {
            continue;
        }
        if (i, j) == end {
            continue;
        }
        for &(i, j) in &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if grid[i as usize][j as usize] != b'#' {
                queue.push_back((steps + 1, (i, j)));
            }
        }
    }
    visited
}

fn dijkstra_cheat(grid: &[&[u8]], start: (isize, isize), max_d: isize) -> HashMap<(isize, isize), isize> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut visited = HashMap::new();
    while let Some((steps, (i, j))) = queue.pop_front() {
        if steps > max_d {
            break;
        }
        let entry = visited.entry((i, j));
        match entry {
            Entry::Occupied(_) => {
                continue;
            }
            Entry::Vacant(entry) => {
                entry.insert(steps);
            }
        }
        for &(i, j) in &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if (0..rows).contains(&i) && (0..cols).contains(&j) {
                queue.push_back((steps + 1, (i, j)));
            }
        }
    }
    visited
}

pub fn cheat(input: &str, max_length: isize, threshold: isize) -> usize {
    let grid: Vec<&[u8]> = input
        .trim()
        .as_bytes()
        .split(|&b| b == b'\n')
        .collect();
    let start = find(&grid, b'S');
    let end = find(&grid, b'E');
    let steps_from_start = dijkstra(&grid, start, end);
    let steps_from_end = dijkstra(&grid, end, start);
    let no_cheat_steps = steps_from_start[&end];
    let mut count = 0;
    for (s, steps_f) in steps_from_start {
        let cheats = dijkstra_cheat(&grid, s, max_length);
        for ((i, j), steps_c) in cheats {
            if grid[i as usize][j as usize] == b'#' {
                continue;
            }
            let Some(steps_b) = steps_from_end.get(&(i, j)) else {
                continue;
            };
            let gained = no_cheat_steps - (steps_f + steps_c + steps_b);
            if gained >= threshold {
                count += 1;
            }
        }
    }
    count
}

pub fn part1(input: &str) -> impl Display {
    cheat(input, 2, 100)
}

pub fn part2(input: &str) -> impl Display {
    cheat(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day20.txt");
    const INPUT: &str = include_str!("../inputs/day20.txt");

    #[test]
    fn test_part1() {
        assert_eq!(cheat(EXAMPLE, 2, 2).to_string(), "44");
        assert_eq!(cheat(EXAMPLE, 2, 4).to_string(), "30");
        assert_eq!(cheat(EXAMPLE, 2, 6).to_string(), "16");
        assert_eq!(cheat(EXAMPLE, 2, 8).to_string(), "14");
        assert_eq!(cheat(EXAMPLE, 2, 10).to_string(), "10");
        assert_eq!(cheat(EXAMPLE, 2, 12).to_string(), "8");
        assert_eq!(cheat(EXAMPLE, 2, 20).to_string(), "5");
        assert_eq!(cheat(EXAMPLE, 2, 36).to_string(), "4");
        assert_eq!(cheat(EXAMPLE, 2, 38).to_string(), "3");
        assert_eq!(cheat(EXAMPLE, 2, 40).to_string(), "2");
        assert_eq!(cheat(EXAMPLE, 2, 64).to_string(), "1");
        assert_eq!(part1(INPUT).to_string(), "1311");
    }

    #[test]
    fn test_part2() {
        assert_eq!(cheat(EXAMPLE, 20, 50).to_string(), "285");
        assert_eq!(cheat(EXAMPLE, 20, 52).to_string(), "253");
        assert_eq!(cheat(EXAMPLE, 20, 54).to_string(), "222");
        assert_eq!(cheat(EXAMPLE, 20, 56).to_string(), "193");
        assert_eq!(cheat(EXAMPLE, 20, 58).to_string(), "154");
        assert_eq!(cheat(EXAMPLE, 20, 60).to_string(), "129");
        assert_eq!(cheat(EXAMPLE, 20, 62).to_string(), "106");
        assert_eq!(cheat(EXAMPLE, 20, 64).to_string(), "86");
        assert_eq!(cheat(EXAMPLE, 20, 66).to_string(), "67");
        assert_eq!(cheat(EXAMPLE, 20, 68).to_string(), "55");
        assert_eq!(cheat(EXAMPLE, 20, 70).to_string(), "41");
        assert_eq!(cheat(EXAMPLE, 20, 72).to_string(), "29");
        assert_eq!(cheat(EXAMPLE, 20, 74).to_string(), "7");
        assert_eq!(cheat(EXAMPLE, 20, 76).to_string(), "3");
        assert_eq!(part2(INPUT).to_string(), "961364");
    }
}
