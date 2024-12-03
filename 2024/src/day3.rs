use std::fmt::Display;

fn parse_number(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, c| acc * 10 + (*c - b'0') as u64)
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    let mut offset = 0;
    while let Some(pos) = input[offset..].find("mul(") {
        let input = input.as_bytes();
        offset += pos + 4;
        if offset >= input.len() {
            break;
        }
        let left_start = offset;
        for _ in 0..3 {
            if offset >= input.len() {
                break;
            }
            if !input[offset].is_ascii_digit() {
                break;
            }
            offset += 1;
        }
        let left_stop = offset;
        if offset >= input.len() {
            break;
        }
        if input[offset] != b',' {
            offset += 1;
            continue;
        }
        offset += 1;
        let right_start = offset;
        for _ in 0..3 {
            if offset >= input.len() {
                break;
            }
            if !input[offset].is_ascii_digit() {
                break;
            }
            offset += 1;
        }
        let right_stop = offset;
        if offset >= input.len() {
            break;
        }
        if input[offset] != b')' {
            offset += 1;
            continue;
        }
        offset += 1;
        let left = parse_number(&input[left_start..left_stop]);
        let right = parse_number(&input[right_start..right_stop]);
        total += left * right;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day3.txt");
    const INPUT: &str = include_str!("../inputs/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "161");
        assert_eq!(part1(INPUT).to_string(), "170068701");
    }
}
