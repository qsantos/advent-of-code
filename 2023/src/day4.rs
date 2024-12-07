use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, got) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let got: HashSet<u32> = got.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let count = winning.intersection(&got).count() as u32;
        if count != 0 {
            score += 2u32.pow(count - 1);
        }
    }
    score
}

pub fn part2(input: &str) -> u32 {
    let mut total_card_count = 0;
    let mut card_counts = HashMap::new();
    let mut card = 1;
    for line in input.lines() {
        let card_count = *card_counts.entry(card).and_modify(|e| *e += 1).or_insert(1);
        total_card_count += card_count;
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, got) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let got: HashSet<u32> = got.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let winning_number_count = winning.intersection(&got).count() as u32;
        for offset in 1..=winning_number_count {
            card_counts.entry(card + offset).and_modify(|e| *e += card_count).or_insert(card_count);
        }
        card += 1;
    }
    total_card_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day4.txt");
    const INPUT: &str = include_str!("../inputs/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
        assert_eq!(part1(INPUT), 17803);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 30);
        assert_eq!(part2(INPUT), 5554894);
    }
}
