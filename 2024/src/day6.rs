use std::collections::HashSet;
use std::fmt::Display;

fn find_start(grid: &[&[u8]]) -> (i64, i64) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == b'^' {
                return (i as i64, j as i64);
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().as_bytes().split(|&b| b == b'\n').collect()
}

fn count_steps(grid: &[&[u8]]) -> usize {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let (mut i, mut j) = find_start(grid);
    let (mut di, mut dj) = (-1, 0); // up
    let mut visited = HashSet::new();
    while (0..rows).contains(&i) && (0..cols).contains(&j) {
        visited.insert((i as usize, j as usize));
        loop {
            let (ni, nj) = (i + di, j + dj);
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                (i, j) = (ni, nj);
                break;
            }
            let (si, sj) = (ni as usize, nj as usize);
            if grid[si][sj] != b'#' {
                (i, j) = (ni, nj);
                break;
            }
            (di, dj) = (dj, -di)
        }
    }
    visited.len()
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse(input);
    count_steps(&grid)
}

fn loops_with_obstacle(
    grid: &[&[u8]],
    pos: (i64, i64),
    dir: (i64, i64),
    obstacle: (i64, i64),
) -> bool {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let (mut i, mut j) = pos;
    let (mut di, mut dj) = dir;
    let mut states = HashSet::new();
    while (0..rows).contains(&i) && (0..cols).contains(&j) {
        if !states.insert((i, j, di, dj)) {
            return true;
        }
        loop {
            let (ni, nj) = (i + di, j + dj);
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                (i, j) = (ni, nj);
                break;
            }
            if (ni, nj) != obstacle && grid[ni as usize][nj as usize] != b'#' {
                (i, j) = (ni, nj);
                break;
            }
            (di, dj) = (dj, -di)
        }
    }
    false
}

fn count_loops(grid: &[&[u8]]) -> usize {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let (mut i, mut j) = find_start(grid);
    let (mut di, mut dj) = (-1, 0); // up
    let mut count = 0;
    let mut visited = HashSet::new();
    while (0..rows).contains(&i) && (0..cols).contains(&j) {
        visited.insert((i, j));
        let (ni, nj) = loop {
            let (ni, nj) = (i + di, j + dj);
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                break (ni, nj);
            }
            let (si, sj) = (ni as usize, nj as usize);
            if grid[si][sj] != b'#' {
                break (ni, nj);
            }
            (di, dj) = (dj, -di)
        };
        // try to put an obstacle on the next cell (unless we already visited it)
        if !visited.contains(&(ni, nj)) && loops_with_obstacle(grid, (i, j), (di, dj), (ni, nj)) {
            count += 1;
        }
        (i, j) = (ni, nj);
    }
    count
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse(input);
    count_loops(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day6.txt");
    const INPUT: &str = include_str!("../inputs/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "41");
        assert_eq!(part1(INPUT).to_string(), "4665");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "6");
        assert_eq!(part2(INPUT).to_string(), "1688");
    }
}
