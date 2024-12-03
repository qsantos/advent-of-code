use std::fmt::Display;

fn parse_number(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, c| acc * 10 + (*c - b'0') as u64)
}

pub fn handle_mut(input: &str, mut offset: usize) -> (Option<u64>, usize) {
    let input = input.as_bytes();
    offset += 4;
    if offset >= input.len() {
        return (None, offset);
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
        return (None, offset);
    }
    if input[offset] != b',' {
        offset += 1;
        return (None, offset);
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
        return (None, offset);
    }
    if input[offset] != b')' {
        offset += 1;
        return (None, offset);
    }
    offset += 1;
    let left = parse_number(&input[left_start..left_stop]);
    let right = parse_number(&input[right_start..right_stop]);
    (Some(left * right), offset)
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    let mut offset = 0;
    while let Some(pos) = input[offset..].find("mul(") {
        let (v, new_offset) = handle_mut(input, offset + pos);
        if let Some(v) = v {
            total += v;
        }
        offset = new_offset;
    }
    total
}

pub fn part2(input: &str) -> impl Display {
    let mut next_do = input.find("do()");
    let mut next_dont = input.find("don't()");
    let mut next_mul = input.find("mul(").unwrap();
    let mut do_mul = true;
    let mut total = 0;
    // loop until no more next_mul
    loop {
        if let Some(pos) = next_do {
            if (next_dont.is_none() || next_do < next_dont) && pos < next_mul {
                do_mul = true;
                next_do = input[pos + 4..].find("do()").map(|p| pos + 4 + p);
                continue;
            }
        }
        if let Some(pos) = next_dont {
            // previous block handled do()
            if pos < next_mul {
                do_mul = false;
                next_dont = input[pos + 7..].find("don't()").map(|p| pos + 7 + p);
                continue;
            }
        }
        // previous blocks handled do() and don't()
        if !do_mul {
            if next_mul + 4 >= input.len() {
                break;
            } else if let Some(pos) = input[next_mul + 4..].find("mul(") {
                next_mul += 4 + pos;
                continue;
            } else {
                break;
            }
        }
        let (v, offset) = handle_mut(input, next_mul);
        if let Some(v) = v {
            total += v;
        }
        if let Some(pos) = input[offset..].find("mul(") {
            next_mul = offset + pos;
        } else {
            break;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day3-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day3-2.txt");
    const INPUT: &str = include_str!("../inputs/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "161");
        assert_eq!(part1(INPUT).to_string(), "170068701");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2).to_string(), "48");
        assert_eq!(part2(INPUT).to_string(), "78683433");
    }
}
