use std::collections::HashSet;

fn char_priority(c: char) -> u32 {
    match c {
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> u32 {
    let mut total_priority = 0;
    for line in input.lines() {
        assert_eq!(line.len() % 2, 0);
        let (r1, r2) = line.split_at(line.len() / 2);
        let [s1, s2] = [r1, r2].map(|r| r.chars().collect::<HashSet<_>>());
        let d: Vec<_> = s1.intersection(&s2).copied().collect();
        assert_eq!(d.len(), 1);
        let c = *d.first().unwrap();
        total_priority += char_priority(c);
    }
    total_priority
}

pub fn part2(input: &str) -> u32 {
    let mut total_priority = 0;
    let lines: Vec<_> = input.lines().collect();
    for group in lines.chunks_exact(3) {
        let d = group
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap();
        assert_eq!(d.len(), 1);
        let c = *d.iter().next().unwrap();
        total_priority += char_priority(c);
    }
    total_priority
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day3.txt");
    const INPUT: &str = include_str!("../inputs/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 157);
        assert_eq!(part1(INPUT), 8493);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 70);
        assert_eq!(part2(INPUT), 2552);
    }
}
