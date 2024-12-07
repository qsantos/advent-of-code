use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGIT_VALUES: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn part1(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut total = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let mut digits = line.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let value: u32 = format!("{first}{last}").parse().unwrap();
        total += value;
        buf.clear();
    }
    total
}

fn first_digit<I: IntoIterator<Item = usize>>(bytes: &[u8], range: I) -> u32 {
    range.into_iter().find_map(|i| {
        DIGIT_VALUES
            .iter()
            .find_map(|(digit, value)| bytes[i..].starts_with(digit.as_bytes()).then_some(*value) )
    })
    .unwrap()
}

pub fn part2(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut total = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let bytes = line.as_bytes();
        let first = first_digit(bytes, 0..bytes.len());
        let last = first_digit(bytes, (0..bytes.len()).rev());
        let value = first * 10 + last;
        total += value;
        buf.clear();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day1-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day1-2.txt");
    const INPUT: &str = include_str!("../inputs/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 142);
        assert_eq!(part1(INPUT), 54388);

        assert_eq!(part2(EXAMPLE2), 281);
        assert_eq!(part2(INPUT), 53515);
    }
}
