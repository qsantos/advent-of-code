use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let needle = b"XMAS";
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let mut count = 0;
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let dirs = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, 1],
        [1, -1],
        [-1, 1],
        [-1, -1],
    ];
    for i in 0..rows {
        for j in 0..cols {
            for dir in dirs {
                count += 1;
                for (a, &n) in needle.iter().enumerate() {
                    let ni = i + a as i64 * dir[0];
                    let nj = j + a as i64 * dir[1];
                    if !(0..rows).contains(&ni)
                        || !(0..cols).contains(&nj)
                        || grid[ni as usize][nj as usize] != n
                    {
                        count -= 1;
                        break;
                    }
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> impl Display {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if grid[i][j] != b'A' {
                continue;
            }
            let (a, b) = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
            if (a, b) != (b'M', b'S') && (a, b) != (b'S', b'M') {
                continue;
            }
            let (a, b) = (grid[i - 1][j + 1], grid[i + 1][j - 1]);
            if (a, b) != (b'M', b'S') && (a, b) != (b'S', b'M') {
                continue;
            }
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day4.txt");
    const INPUT: &str = include_str!("../inputs/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "18");
        assert_eq!(part1(INPUT).to_string(), "2532");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "9");
        assert_eq!(part2(INPUT).to_string(), "1941");
    }
}
