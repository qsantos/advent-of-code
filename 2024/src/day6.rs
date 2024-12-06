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

fn path(grid: &[&[u8]], obstable: Option<(usize, usize)>) -> Option<HashSet<(usize, usize)>> {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let (mut i, mut j) = find_start(grid);
    let (mut di, mut dj) = (-1, 0); // up
    let mut visited = HashSet::new();
    let mut states = HashSet::new();
    while (0..rows).contains(&i) && (0..cols).contains(&j) {
        if !states.insert((i, j, di, dj)) {
            return None;
        }
        visited.insert((i as usize, j as usize));
        loop {
            let (ni, nj) = (i + di, j + dj);
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                (i, j) = (ni, nj);
                break;
            }
            let (si, sj) = (ni as usize, nj as usize);
            if Some((si, sj)) != obstable && grid[si][sj] != b'#' {
                (i, j) = (ni, nj);
                break;
            }
            (di, dj) = (dj, -di)
        }
    }
    Some(visited)
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse(input);
    path(&grid, None).unwrap().len()
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse(input);
    let mut count = 0;
    let visited = path(&grid, None).unwrap();
    for (i, j) in visited {
        if path(&grid, Some((i, j))).is_none() {
            count += 1;
        }
    }
    count
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
