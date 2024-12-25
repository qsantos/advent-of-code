use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for keylock in input.split("\n\n") {
        let grid: Vec<&[u8]> = keylock.as_bytes().split(|b| *b == b'\n').collect();
        let mut values = [0; 5];
        if grid[0][0] == b'#' {
            // lock
            for j in 0..5 {
                values[j] = (0..7).position(|i| grid[i][j] != b'#').unwrap() as u8 - 1;
            }
            locks.push(values);
        } else {
            // key
            for j in 0..5 {
                values[j] = (0..7).rev().position(|i| grid[i][j] != b'#').unwrap() as u8 - 1;
            }
            keys.push(values);
        }
    }
    let mut fit = 0;
    for lock in &locks {
        for key in &keys {
            if (0..5).all(|i| lock[i] + key[i] < 6) {
                fit += 1;
            }
        }
    }
    fit
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day25.txt");
    const INPUT: &str = include_str!("../inputs/day25.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "3");
        assert_eq!(part1(INPUT).to_string(), "3133");
    }
}
