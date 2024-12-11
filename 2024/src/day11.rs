use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for _ in 0..25 {
        let mut new_numbers = Vec::new();
        for number in numbers {
            if number == 0 {
                new_numbers.push(1);
                continue;
            }
            let digits = number.ilog10() + 1;
            if digits % 2 == 0 {
                let divider = 10u64.pow(digits / 2);
                new_numbers.push(number / divider);
                new_numbers.push(number % divider);
            } else {
                new_numbers.push(number * 2024);
            }
        }
        numbers = new_numbers;
    }
    numbers.len()
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
}
