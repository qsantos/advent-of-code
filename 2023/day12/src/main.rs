use std::fs::File;
use std::io::{BufRead, BufReader};

fn arrangements(springs: &[u8], groups: &Vec<i32>) -> usize {
    // c[i][j] is the number of arrangements when looking at the first i springs and j groups
    let mut c = Vec::new();

    {
        let mut first_row = Vec::new();
        first_row.push(1);
        first_row.resize(groups.len() + 1, 0);
        c.push(first_row);
    }

    for i in 1..=springs.len() {
        let spring = springs[i - 1];
        let mut row = Vec::new();
        for j in 0..=groups.len() {
            let mut count = 0;
            // consume no group
            if spring == b'.' || spring == b'?' {
                count += c[i - 1][j];
            }
            // consume a single group
            if j > 0 {
                let group_size = groups[j - 1] as usize;
                // group elements
                if i >= group_size && springs[(i - 1) - (group_size - 1)..=(i - 1)].iter().all(|s| *s == b'#' || *s == b'?') {
                    if i == group_size {
                        count += c[0][j - 1];
                    } else {
                        // group separator
                        let s = springs[(i - 1) - (group_size - 1) - 1];
                        if s == b'.' || s == b'?' {
                            count += c[i - group_size - 1][j - 1];
                        }
                    }
                }
            }
            row.push(count);
        }
        c.push(row);
    }
    c[springs.len()][groups.len()]
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

fn part2(filename: &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.as_bytes();
        let springs = [springs, springs, springs, springs, springs].join(&b'?');
        let groups: Vec<i32> = groups.split(',').map(|g| g.parse().unwrap()).collect();
        let groups = groups.repeat(5);
        let c = arrangements(&springs, &groups);
        dbg!(line, c);
        sum += c;
        buf.clear();
    }
    sum
}

fn main() {
    assert_eq!(part1("example"), 21);
    assert_eq!(part1("input"), 6852);

    assert_eq!(part2("example"), 525152);
    assert_eq!(part2("input"), 8475948826693);
}
