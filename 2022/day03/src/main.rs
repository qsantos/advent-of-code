use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::{prelude::BufRead, BufReader};

fn char_priority(c: char) -> u32 {
    match c {
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        _ => unreachable!(),
    }
}

fn priority_of_filename(filename: &str) -> u32 {
    let mut total_priority = 0;
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
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

fn badges_of_filename(filename: &str) -> u32 {
    let mut total_priority = 0;
    let contents = read_to_string(filename).expect("Could not read file");
    let lines: Vec<_> = contents.lines().collect();
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

fn puzzle1() {
    assert_eq!(priority_of_filename("example"), 157);
    assert_eq!(priority_of_filename("input"), 8493);
}

fn puzzle2() {
    assert_eq!(badges_of_filename("example"), 70);
    assert_eq!(badges_of_filename("input"), 2552);
}

fn main() {
    puzzle1();
    puzzle2();
}
