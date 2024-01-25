use std::collections::HashSet;

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();

    fn aux(
        grid: &[&[u8]],
        (x, y): (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> Option<usize> {
        let rows = grid.len();
        let cols = grid[0].len();
        if (x, y) == (rows - 1, cols - 2) {
            return Some(0);
        }
        if visited.contains(&(x, y)) {
            return None;
        }
        visited.insert((x, y));
        let mut candidates = Vec::new();
        if x > 0 {
            let c = grid[x - 1][y];
            // NOTE: no ^ in example or input
            if c == b'.' || c == b'^' {
                if let Some(v) = aux(grid, (x - 1, y), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if x < rows - 1 {
            let c = grid[x + 1][y];
            if c == b'.' || c == b'v' {
                if let Some(v) = aux(grid, (x + 1, y), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if y > 0 {
            let c = grid[x][y - 1];
            // NOTE: no < in example or input
            if c == b'.' || c == b'<' {
                if let Some(v) = aux(grid, (x, y - 1), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if y < cols - 1 {
            let c = grid[x][y + 1];
            if c == b'.' || c == b'>' {
                if let Some(v) = aux(grid, (x, y + 1), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        visited.remove(&(x, y));
        candidates.into_iter().max()
    }

    let mut visited = HashSet::new();
    aux(&grid, (0, 1), &mut visited).unwrap()
}

fn main() {
    assert_eq!(part1("example"), 94);
    assert_eq!(part1("input"), 2182);
}
