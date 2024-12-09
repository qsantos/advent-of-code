use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let numbers = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    assert_ne!(numbers.len() % 2, 0);
    let mut res = 0;
    let mut forward_index = 0;
    let mut backward_index = numbers.len() - 1;
    let mut remaining = numbers[backward_index];
    let mut position = 0;
    while forward_index < backward_index {
        // occupied blocks
        let file_size = numbers[forward_index];
        let file_index = forward_index / 2;
        for _ in 0..file_size {
            res += position * file_index;
            position += 1;
        }
        forward_index += 1;
        // free space
        let free_size = numbers[forward_index];
        for _ in 0..free_size {
            if remaining == 0 {
                backward_index -= 2;
                if forward_index >= backward_index {
                    break;
                }
                remaining = numbers[backward_index];
            }
            let file_index = backward_index / 2;
            res += position * file_index;
            position += 1;
            remaining -= 1;
        }
        forward_index += 1;
    }
    while remaining > 0 {
        let file_index = backward_index / 2;
        res += position * file_index;
        position += 1;
        remaining -= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day9.txt");
    const INPUT: &str = include_str!("../inputs/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "1928");
        assert_eq!(part1(INPUT).to_string(), "6310675819476");
    }
}
