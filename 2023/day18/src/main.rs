use std::collections::HashSet;

fn parse_dir(dir: &str) -> (i32, i32) {
    match dir.as_bytes()[0] {
        b'L' => (0, -1),
        b'U' => (-1, 0),
        b'R' => (0, 1),
        b'D' => (1, 0),
        _ => unreachable!(),
    }
}

fn parse_trench(filename: &str) -> HashSet<(i32, i32)> {
    let mut ret = HashSet::new();
    let data = std::fs::read_to_string(filename).unwrap();
    let (mut i, mut j) = (0, 0);
    for line in data.lines() {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (dist, _) = rest.split_once(' ').unwrap();
        let (di, dj) = parse_dir(dir);
        let dist: i32 = dist.parse().unwrap();
        for _ in 0..dist {
            ret.insert((i, j));
            i += di;
            j += dj;
        }
    }
    ret
}

fn part1(filename: &str) -> usize {
    let trench = parse_trench(filename);
    let mut visited = trench;
    let mut q = vec![(1, 1)];
    while let Some((i, j)) = q.pop() {
        if visited.insert((i, j)) {
            for n in [(i, j - 1), (i - 1, j), (i, j + 1), (i + 1, j)] {
                q.push(n);
            }
        }
    }
    visited.len()
}

fn parse_dir2(dir: u8) -> (i32, i32) {
    match dir {
        b'2' => (0, -1),
        b'3' => (-1, 0),
        b'0' => (0, 1),
        b'1' => (1, 0),
        _ => unreachable!(),
    }
}

fn part2(filename: &str) -> usize {
    let mut ret = HashSet::new();
    let data = std::fs::read_to_string(filename).unwrap();
    let (mut i, mut j) = (0, 0);
    for line in data.lines() {
        let (_, color) = line.rsplit_once('#').unwrap();
        let dist = i32::from_str_radix(&color[..5], 16).unwrap();
        let dir = color.as_bytes()[5];
        let (di, dj) = parse_dir2(dir);
        println!("{dir:?} {dist}");
        for _ in 0..dist {
            ret.insert((i, j));
            i += di;
            j += dj;
        }
    }

    let trench = ret;
    let mut visited = trench;
    let mut q = vec![(1, 1)];
    while let Some((i, j)) = q.pop() {
        if visited.insert((i, j)) {
            for n in [(i, j - 1), (i - 1, j), (i, j + 1), (i + 1, j)] {
                q.push(n);
            }
        }
    }
    visited.len()
}

fn main() {
    assert_eq!(part1("example"), 62);
    assert_eq!(part1("input"), 44436);

    // assert_eq!(part2("example"), 62);
    assert_eq!(part2("input"), 44436);
}
