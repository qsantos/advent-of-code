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

pub fn part1(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let (mut i, mut j) = find_start(&grid);
    let (mut di, mut dj) = (-1, 0); // up
    let mut visited = HashSet::new();
    while (0..rows).contains(&i) && (0..cols).contains(&j) {
        println!("{i} {j} {di} {dj}");
        visited.insert((i, j));
        loop {
            let (ni, nj) = (i + di, j + dj);
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                (i, j) = (ni, nj);
                break;
            }
            if grid[ni as usize][nj as usize] != b'#' {
                (i, j) = (ni, nj);
                break;
            }
            (di, dj) = (dj, -di)
        }
    }
    visited.len()
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
}
