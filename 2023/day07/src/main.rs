use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const STRENGTHS1: &[u8; 13] = b"AKQJT98765432";
const STRENGTHS2: &[u8; 13] = b"AKQT98765432J";

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards1(cards: &[u8; 5]) -> HandType {
        let mut counts = HashMap::new();
        for card in cards {
            counts.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut counts: Vec<i32> = counts.into_values().collect();
        counts.sort();
        counts.reverse();
        match counts[..] {
                [5] => HandType::FiveOfAKind,
                [4, 1] => HandType::FourOfAKind,
                [3, 2] => HandType::FullHouse,
                [3, 1, 1] => HandType::ThreeOfAKind,
                [2, 2, 1] => HandType::TwoPair,
                [2, 1, 1, 1] => HandType::OnePair,
                [1, 1, 1, 1, 1] => HandType::HighCard,
                _ => panic!("Unexpected card counts {:?}", counts),
        }
    }

    fn from_cards2(cards: &[u8; 5]) -> HandType {
        let mut counts = HashMap::new();
        for card in cards {
            counts.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }
        let jokers = counts.remove(&b'J').unwrap_or(0);
        let mut counts: Vec<i32> = counts.into_values().collect();
        counts.sort();
        counts.reverse();
        match jokers {
            0 => match counts[..] {
                [5] => HandType::FiveOfAKind,
                [4, 1] => HandType::FourOfAKind,
                [3, 2] => HandType::FullHouse,
                [3, 1, 1] => HandType::ThreeOfAKind,
                [2, 2, 1] => HandType::TwoPair,
                [2, 1, 1, 1] => HandType::OnePair,
                [1, 1, 1, 1, 1] => HandType::HighCard,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            1 => match counts[..] {
                [4] => HandType::FiveOfAKind,
                [3, 1] => HandType::FourOfAKind,
                [2, 2] => HandType::FullHouse,
                [2, 1, 1] => HandType::ThreeOfAKind,
                [1, 1, 1, 1] => HandType::OnePair,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            2 => match counts[..] {
                [3] => HandType::FiveOfAKind,
                [2, 1] => HandType::FourOfAKind,
                [1, 1, 1] => HandType::ThreeOfAKind,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            3 => match counts[..] {
                [2] => HandType::FiveOfAKind,
                [1, 1] => HandType::FourOfAKind,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            4 => match counts[..] {
                [1] => HandType::FiveOfAKind,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            5 => match counts[..] {
                [] => HandType::FiveOfAKind,
                _ => panic!("Unexpected card counts {:?} for {jokers} jokers", counts),
            },
            _ => panic!("Too many jokers: {jokers}"),
        }
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Hand {
    type_: HandType,
    strengths: [u8; 5],
}

impl Hand {
    fn new1(cards: [u8; 5]) -> Hand {
        Hand {
            type_: HandType::from_cards1(&cards),
            strengths: cards
                .into_iter()
                .map(|card| STRENGTHS1.iter().position(|o| *o == card).unwrap() as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }

    fn new2(cards: [u8; 5]) -> Hand {
        Hand {
            type_: HandType::from_cards2(&cards),
            strengths: cards
                .into_iter()
                .map(|card| STRENGTHS2.iter().position(|o| *o == card).unwrap() as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }
}

fn part1(filename: &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut hands = Vec::new();
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards: [u8; 5] = cards.as_bytes().try_into().unwrap();
        let bid: usize = bid.parse().unwrap();
        hands.push((Hand::new1(cards), bid));
        buf.clear();
    }
    hands.sort();
    hands.reverse();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn part2(filename: &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut hands = Vec::new();
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards: [u8; 5] = cards.as_bytes().try_into().unwrap();
        let bid: usize = bid.parse().unwrap();
        hands.push((Hand::new2(cards), bid));
        buf.clear();
    }
    hands.sort();
    hands.reverse();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn main() {
    assert_eq!(part1("example"), 6440);
    assert_eq!(part1("input"), 253205868);

    assert_eq!(part2("example"), 5905);
    assert_eq!(part2("input"), 253205868);
}
