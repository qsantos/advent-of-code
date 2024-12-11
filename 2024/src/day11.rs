use std::collections::HashMap;
use std::fmt::Display;

pub fn part12(input: &str, count: usize) -> u64 {
    let numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut numbers: HashMap<u64, u64> = numbers.into_iter().map(|n| (n, 1)).collect();
    for _ in 0..count {
        let mut new_numbers = HashMap::new();
        for (number, count) in numbers {
            if number == 0 {
                new_numbers
                    .entry(1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                continue;
            }
            let digits = number.ilog10() + 1;
            if digits % 2 == 0 {
                let divider = 10u64.pow(digits / 2);
                new_numbers
                    .entry(number / divider)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                new_numbers
                    .entry(number % divider)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                new_numbers
                    .entry(number * 2024)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        numbers = new_numbers;
    }
    numbers.into_values().sum()
}

pub fn part1(input: &str) -> impl Display {
    part12(input, 25)
}

pub fn part2(input: &str) -> impl Display {
    part12(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day11.txt");
    const INPUT: &str = include_str!("../inputs/day11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "55312");
        assert_eq!(part1(INPUT).to_string(), "239714");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).to_string(), "284973560658514");
    }
}
