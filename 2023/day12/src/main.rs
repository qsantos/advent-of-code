use std::fs::File;
use std::io::{BufRead, BufReader};

fn arrangements(springs: &[u8], groups: &Vec<i32>) -> usize {
    fn f(springs: &[u8], i: usize, groups: &Vec<i32>, j: usize, current_group_size: i32) -> usize {
        if i == springs.len() {
            if j == groups.len() || (j == groups.len() - 1 && current_group_size == *groups.last().unwrap()) {
                return 1;
            } else {
                return 0;
            }
        } else if j == groups.len() {
            if springs[i..].iter().any(|s| *s == b'#') {
                return 0;
            } else {
                return 1;
            }
        }
        let spring = springs[i];
        let mut count = 0;
        if spring == b'.' || spring == b'?' {
            if current_group_size != 0 {
                let group = groups[j];
                if current_group_size == group {
                    count += f(springs, i + 1, groups, j + 1, 0);
                }
            } else {
                count += f(springs, i + 1, groups, j, 0);
            }
        }
        if spring == b'#' || spring == b'?' {
            count += f(springs, i + 1, groups, j, current_group_size + 1);
        }
        count
    }
    f(springs, 0, groups, 0, 0)
}

fn part1(filename: &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.as_bytes();
        let groups: Vec<i32> = groups.split(',').map(|g| g.parse().unwrap()).collect();
        let c = arrangements(springs, &groups);
        sum += c;
        buf.clear();
    }
    sum
}

fn main() {
    assert_eq!(part1("example"), 21);
    assert_eq!(part1("input"), 6852);
}
