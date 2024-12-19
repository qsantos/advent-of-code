use std::collections::HashMap;
use std::fmt::Display;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect();
    let designs = designs.lines().collect();
    (towels, designs)
}

fn count_combinations<'a>(towels: &[&str], design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = memo.get(design) {
        return count;
    }
    let mut count = 0;
    for towel in towels {
        if let Some(suffix) = design.strip_prefix(towel) {
            count += count_combinations(towels, suffix, memo);
        }
    }
    memo.insert(design, count);
    count
}

pub fn part1(input: &str) -> impl Display {
    let (towels, designs) = parse(input);

    let mut memo = HashMap::new();
    let mut count = 0;
    for design in designs {
        if count_combinations(&towels, design, &mut memo) != 0 {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> impl Display {
    let (towels, designs) = parse(input);
    let mut memo = HashMap::new();
    let mut count = 0;
    for design in designs {
        count += count_combinations(&towels, design, &mut memo);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day19.txt");
    const INPUT: &str = include_str!("../inputs/day19.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "6");
        assert_eq!(part1(INPUT).to_string(), "216");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "16");
        assert_eq!(part2(INPUT).to_string(), "603191454138773");
    }
}
