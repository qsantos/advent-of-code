use std::collections::HashSet;

fn part1(filename: &str, steps: usize) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|c| *c == b'S').map(|j| (i, j)))
        .next()
        .unwrap();

    let mut q = vec![start];
    for _ in 0..steps {
        let mut next = HashSet::new();
        for (i, j) in q {
            let mut candidates = Vec::new();
            if i > 0 {
                candidates.push((i - 1, j));
            }
            if j > 0 {
                candidates.push((i, j - 1));
            }
            if i < rows - 1 {
                candidates.push((i + 1, j));
            }
            if j < cols - 1 {
                candidates.push((i, j + 1));
            }
            for (ni, nj) in candidates {
                if grid[ni][nj] != b'#' {
                    next.insert((ni, nj));
                }
            }
        }
        q = next.into_iter().collect();
    }
    q.len()
}

fn main() {
    assert_eq!(part1("example", 6), 16);
    assert_eq!(part1("input", 64), 3666);
}
