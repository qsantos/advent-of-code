use std::collections::HashSet;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut frequencies: HashSet<char> = input.chars().collect();
    frequencies.remove(&'.');
    frequencies.remove(&'\n');
    let mut antinodes = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len() as i64;
    let cols = lines[0].len() as i64;
    for frequency in frequencies {
        let mut positions = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == frequency {
                    positions.push((i as i64, j as i64));
                }
            }
        }
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in positions[i + 1..].iter() {
                let di = pos2.0 - pos1.0;
                let dj = pos2.1 - pos1.1;
                let candidates = [(pos1.0 - di, pos1.1 - dj), (pos2.0 + di, pos2.1 + dj)];
                for (ai, aj) in candidates {
                    if (0..rows).contains(&ai) && (0..cols).contains(&aj) {
                        antinodes.insert((ai, aj));
                    }
                }
            }
        }
    }
    antinodes.len()
}

pub fn part2(input: &str) -> impl Display {
    let mut frequencies: HashSet<char> = input.chars().collect();
    frequencies.remove(&'.');
    frequencies.remove(&'\n');
    let mut antinodes = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len() as i64;
    let cols = lines[0].len() as i64;
    for frequency in frequencies {
        let mut positions = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == frequency {
                    positions.push((i as i64, j as i64));
                }
            }
        }
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in positions[i + 1..].iter() {
                let di = pos2.0 - pos1.0;
                let dj = pos2.1 - pos1.1;
                let mut n = 0;
                loop {
                    let ai = pos1.0 + n * di;
                    if !(0..rows).contains(&ai) {
                        break;
                    }
                    let aj = pos1.1 + n * dj;
                    if !(0..cols).contains(&aj) {
                        break;
                    }
                    antinodes.insert((ai, aj));
                    n += 1;
                }
                let mut n = 0;
                loop {
                    let ai = pos1.0 - n * di;
                    if !(0..rows).contains(&ai) {
                        break;
                    }
                    let aj = pos1.1 - n * dj;
                    if !(0..cols).contains(&aj) {
                        break;
                    }
                    antinodes.insert((ai, aj));
                    n += 1;
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day8.txt");
    const INPUT: &str = include_str!("../inputs/day8.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "14");
        assert_eq!(part1(INPUT).to_string(), "361");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "34");
        assert_eq!(part2(INPUT).to_string(), "1249");
    }
}
