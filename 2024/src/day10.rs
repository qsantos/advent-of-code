use std::collections::HashSet;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != b'0' {
                continue;
            }
            let mut q = vec![(i, j)];
            let mut visited = HashSet::new();
            while let Some((i, j)) = q.pop() {
                if !visited.insert((i, j)) {
                    continue;
                }
                let v = grid[i][j];
                if v == b'9' {
                    score += 1;
                    continue;
                }
                if i > 0 && grid[i - 1][j] == v + 1 {
                    q.push((i - 1, j));
                }
                if i < rows - 1 && grid[i + 1][j] == v + 1 {
                    q.push((i + 1, j));
                }
                if j > 0 && grid[i][j - 1] == v + 1 {
                    q.push((i, j - 1));
                }
                if j < cols - 1 && grid[i][j + 1] == v + 1 {
                    q.push((i, j + 1));
                }
            }
        }
    }
    score
}

pub fn part2(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != b'0' {
                continue;
            }
            let mut q = vec![(i, j)];
            while let Some((i, j)) = q.pop() {
                let v = grid[i][j];
                if v == b'9' {
                    score += 1;
                    continue;
                }
                if i > 0 && grid[i - 1][j] == v + 1 {
                    q.push((i - 1, j));
                }
                if i < rows - 1 && grid[i + 1][j] == v + 1 {
                    q.push((i + 1, j));
                }
                if j > 0 && grid[i][j - 1] == v + 1 {
                    q.push((i, j - 1));
                }
                if j < cols - 1 && grid[i][j + 1] == v + 1 {
                    q.push((i, j + 1));
                }
            }
        }
    }
    score
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day10-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day10-2.txt");
    const EXAMPLE3: &str = include_str!("../examples/day10-3.txt");
    const EXAMPLE4: &str = include_str!("../examples/day10-4.txt");
    const EXAMPLE5: &str = include_str!("../examples/day10-5.txt");
    const EXAMPLE6: &str = include_str!("../examples/day10-6.txt");
    const EXAMPLE7: &str = include_str!("../examples/day10-7.txt");
    const INPUT: &str = include_str!("../inputs/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "2");
        assert_eq!(part1(EXAMPLE2).to_string(), "4");
        assert_eq!(part1(EXAMPLE3).to_string(), "3");
        assert_eq!(part1(EXAMPLE4).to_string(), "36");
        assert_eq!(part1(INPUT).to_string(), "778");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE5).to_string(), "3");
        assert_eq!(part2(EXAMPLE6).to_string(), "13");
        assert_eq!(part2(EXAMPLE7).to_string(), "227");
        assert_eq!(part2(EXAMPLE4).to_string(), "81");
        assert_eq!(part2(INPUT).to_string(), "1925");
    }
}
