use std::collections::{BinaryHeap, HashMap};
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

fn dijkstra(grid: &[&[u8]], start: u8, end: u8) -> HashMap<(isize, isize), isize> {
    let start = find(grid, start);
    let end = find(grid, end);
    let mut queue = BinaryHeap::new();
    queue.push((0, start));
    let mut visited = HashMap::new();
    while let Some((steps, (i, j))) = queue.pop() {
        let entry = visited.entry((i, j));
        match entry {
            Entry::Occupied(_) => {
                continue;
            }
            Entry::Vacant(entry) => {
                entry.insert(steps);
            }
        }
        if (i, j) == end {
            continue;
        }
        for &(i, j) in &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if grid[i as usize][j as usize] != b'#' {
                queue.push((steps + 1, (i, j)));
            }
        }
    }
    visited
}

pub fn cheat(input: &str, threshold: isize) -> usize {
    let grid: Vec<&[u8]> = input
        .trim()
        .as_bytes()
        .split(|&b| b == b'\n')
        .collect();
    let end = find(&grid, b'E');
    let steps_from_start = dijkstra(&grid, b'S', b'E');
    let steps_from_end = dijkstra(&grid, b'E', b'S');
    let no_cheat_steps = steps_from_start[&end];
    let mut count = 0;
    for ((i, j), steps_f) in steps_from_start {
        for (di, dj) in &[(2, 0), (-2, 0), (0, 2), (0, -2)] {
            if let Some(steps_b) = steps_from_end.get(&(i + di, j + dj)) {
                let gained = no_cheat_steps - (steps_f + steps_b + 2);
                if gained >= threshold {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part1(input: &str) -> impl Display {
    cheat(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day20.txt");
    const INPUT: &str = include_str!("../inputs/day20.txt");

    #[test]
    fn test_part1() {
        assert_eq!(cheat(EXAMPLE, 2).to_string(), "44");
        assert_eq!(cheat(EXAMPLE, 4).to_string(), "30");
        assert_eq!(cheat(EXAMPLE, 6).to_string(), "16");
        assert_eq!(cheat(EXAMPLE, 8).to_string(), "14");
        assert_eq!(cheat(EXAMPLE, 10).to_string(), "10");
        assert_eq!(cheat(EXAMPLE, 12).to_string(), "8");
        assert_eq!(cheat(EXAMPLE, 20).to_string(), "5");
        assert_eq!(cheat(EXAMPLE, 36).to_string(), "4");
        assert_eq!(cheat(EXAMPLE, 38).to_string(), "3");
        assert_eq!(cheat(EXAMPLE, 40).to_string(), "2");
        assert_eq!(cheat(EXAMPLE, 64).to_string(), "1");
        assert_eq!(part1(INPUT).to_string(), "1311");
    }
}
