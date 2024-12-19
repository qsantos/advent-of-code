use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();
    towels.sort_by_key(|towel| -(towel.len() as isize));

    fn aux(towels: &[&str], design: &str) -> bool {
        if design.is_empty() {
            return true;
        }
        for towel in towels {
            if design.starts_with(towel) && aux(towels, &design[towel.len()..]) {
                return true;
            }
        }
        false
    }

    let mut count = 0;
    for design in designs {
        if aux(&towels, design) {
            count += 1;
        }
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
}
