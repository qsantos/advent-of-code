use std::collections::HashSet;

fn part1(filename: &str, steps: usize) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .position(|c| *c == b'S')
                .map(|j| (i as isize, j as isize))
        })
        .next()
        .unwrap();

    let mut previous = HashSet::new();
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert(start);
    let mut odds = 0;
    let mut evens = 1; // {start} at step 0
    for step in 1..=steps {
        next.clear();
        for (i, j) in current.iter().copied() {
            for (ni, nj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if !(0 <= ni && ni < rows && 0 <= nj && nj < cols) {
                    continue;
                }
                if grid[ni as usize][nj as usize] == b'#' {
                    continue;
                }
                if previous.contains(&(ni, nj)) {
                    continue;
                }
                next.insert((ni, nj));
            }
        }
        (previous, current, next) = (current, next, previous);
        if step % 2 == 0 {
            evens += current.len();
        } else {
            odds += current.len();
        }
    }
    if steps % 2 == 0 {
        evens
    } else {
        odds
    }
}

fn part2(filename: &str, steps: usize) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .position(|c| *c == b'S')
                .map(|j| (i as isize, j as isize))
        })
        .next()
        .unwrap();

    let mut previous = HashSet::new();
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert(start);
    let mut odds = 0;
    let mut evens = 1; // {start} at step 0
    for step in 1..=steps {
        next.clear();
        for (i, j) in current.iter().copied() {
            for (ni, nj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if grid[ni.rem_euclid(rows) as usize][nj.rem_euclid(cols) as usize] == b'#' {
                    continue;
                }
                if previous.contains(&(ni, nj)) {
                    continue;
                }
                next.insert((ni, nj));
            }
        }
        (previous, current, next) = (current, next, previous);
        if step % 2 == 0 {
            evens += current.len();
        } else {
            odds += current.len();
        }
    }
    if steps % 2 == 0 {
        evens
    } else {
        odds
    }
}

fn main() {
    assert_eq!(part1("example", 6), 16);
    assert_eq!(part1("input", 64), 3666);

    assert_eq!(part2("example", 6), 16);
    assert_eq!(part2("example", 10), 50);
    assert_eq!(part2("example", 50), 1594);
    assert_eq!(part2("example", 100), 6536);
    assert_eq!(part2("example", 500), 167004);
    assert_eq!(part2("example", 1000), 668697);
    assert_eq!(part2("example", 5000), 16733044);
    assert_eq!(part2("input", 26501365), 0);
}
