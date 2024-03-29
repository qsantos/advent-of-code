use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn part1(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut score = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, got) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let got: HashSet<u32> = got.split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let count = winning.intersection(&got).count() as u32;
        if count != 0 {
            score += 2u32.pow(count - 1);
        }
        buf.clear();
    }
    score
}

fn part2(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut total_card_count = 0;
    let mut card_counts = HashMap::new();
    let mut card = 1;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
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
        buf.clear();
        card += 1;
    }
    total_card_count
}

fn main() {
    assert_eq!(part1("example"), 13);
    assert_eq!(part1("input"), 17803);

    assert_eq!(part2("example"), 30);
    assert_eq!(part2("input"), 5554894);
}
