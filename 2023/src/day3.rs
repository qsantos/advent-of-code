use std::collections::{HashMap, HashSet};

macro_rules! test_is_engine {
    ($b:ident, $lines:ident, $i:expr, $j:expr, $di:expr, $dj:expr) => {{
        let i = $i;
        let j = $j;
        let di = $di;
        let dj = $dj;
        if let Some(ni) = i.checked_add_signed(di) {
            if let Some(nj) = j.checked_add_signed(dj) {
                if ni < $lines.len() && nj < $lines[i].len() {
                    if !$lines[ni][nj].is_ascii_digit() && $lines[ni][nj] != b'.' {
                        $b = true;
                    }
                }
            }
        }
    }};
}

pub fn part1(input: &str) -> u32 {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut sum = 0;
    for (i, &line) in lines.iter().enumerate() {
        let mut j = 0;
        loop {
            while j < line.len() && !line[j].is_ascii_digit() {
                j += 1;
            }
            if j >= line.len() {
                break;
            }
            let mut number: u32 = 0;
            let mut is_engine_part = false;
            while j < line.len() && line[j].is_ascii_digit() {
                number = 10 * number + (line[j] - b'0') as u32;
                test_is_engine!(is_engine_part, lines, i, j, -1, -1);
                test_is_engine!(is_engine_part, lines, i, j, -1, 0);
                test_is_engine!(is_engine_part, lines, i, j, -1, 1);
                test_is_engine!(is_engine_part, lines, i, j, 0, -1);
                test_is_engine!(is_engine_part, lines, i, j, 0, 0);
                test_is_engine!(is_engine_part, lines, i, j, 0, 1);
                test_is_engine!(is_engine_part, lines, i, j, 1, -1);
                test_is_engine!(is_engine_part, lines, i, j, 1, 0);
                test_is_engine!(is_engine_part, lines, i, j, 1, 1);
                j += 1;
            }
            if is_engine_part {
                sum += number;
            }
        }
    }
    sum
}

macro_rules! add_to_gears {
    ($gears:ident, $lines:ident, $i:expr, $j:expr, $di:expr, $dj:expr) => {{
        let i = $i;
        let j = $j;
        let di = $di;
        let dj = $dj;
        if let Some(ni) = i.checked_add_signed(di) {
            if let Some(nj) = j.checked_add_signed(dj) {
                if ni < $lines.len() && nj < $lines[i].len() {
                    if !$lines[ni][nj].is_ascii_digit() && $lines[ni][nj] == b'*' {
                        $gears.push((ni, nj));
                    }
                }
            }
        }
    }};
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut gear_ratios = HashMap::new();
    for (i, &line) in lines.iter().enumerate() {
        let mut j = 0;
        loop {
            while j < line.len() && !line[j].is_ascii_digit() {
                j += 1;
            }
            if j >= line.len() {
                break;
            }
            let mut number: u32 = 0;
            let mut gears = Vec::new();
            while j < line.len() && line[j].is_ascii_digit() {
                number = 10 * number + (line[j] - b'0') as u32;
                add_to_gears!(gears, lines, i, j, -1, -1);
                add_to_gears!(gears, lines, i, j, -1, 0);
                add_to_gears!(gears, lines, i, j, -1, 1);
                add_to_gears!(gears, lines, i, j, 0, -1);
                add_to_gears!(gears, lines, i, j, 0, 0);
                add_to_gears!(gears, lines, i, j, 0, 1);
                add_to_gears!(gears, lines, i, j, 1, -1);
                add_to_gears!(gears, lines, i, j, 1, 0);
                add_to_gears!(gears, lines, i, j, 1, 1);
                j += 1;
            }
            for gear in gears {
                gear_ratios
                    .entry(gear)
                    .and_modify(|e: &mut HashSet<u32>| {
                        e.insert(number);
                    })
                    .or_insert(HashSet::from([number]));
            }
        }
    }
    gear_ratios
        .into_values()
        .map(|v| {
            if v.len() != 2 {
                0
            } else {
                v.into_iter().product()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day3.txt");
    const INPUT: &str = include_str!("../inputs/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4361);
        assert_eq!(part1(INPUT), 521515);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 467835);
        assert_eq!(part2(INPUT), 69527306);
    }
}
