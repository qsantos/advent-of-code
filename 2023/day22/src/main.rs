use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

impl Coord {
    fn from(s: &str) -> Coord {
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let z = z.parse().unwrap();
        assert!(x < 10);
        assert!(y < 10);
        Coord { x, y, z }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn from(line: &str) -> Self {
        let (start, end) = line.split_once('~').unwrap();
        let start = Coord::from(start);
        let end = Coord::from(end);
        assert!(start.z <= end.z);
        Brick { start, end }
    }
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut bricks: Vec<_> = data.lines().map(Brick::from).collect();
    bricks.sort_by_key(|brick| brick.start.z);

    let mut is_unique_support = HashSet::new();
    let mut summit = [[(0, None); 10]; 10];
    let mut supports = HashSet::new();
    for (i, brick) in bricks.iter().enumerate() {
        let Brick { start, end } = brick;
        // find where, and on which bricks this bricks will fall
        supports.clear();
        let mut max_height = 0;
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let (height, support) = summit[x as usize][y as usize];
                if height > max_height {
                    supports.clear();
                    max_height = height;
                }
                if height == max_height {
                    if let Some(support) = support {
                        supports.insert(support);
                    }
                }
            }
        }
        // update the information about the highest bricks
        let brick_h = brick.end.z - brick.start.z + 1;
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                summit[x as usize][y as usize] = (max_height + brick_h, Some(i));
            }
        }
        // update information about which bricks are unique supports
        if supports.len() == 1 {
            let support = supports.iter().copied().next().unwrap();
            is_unique_support.insert(support);
        }
    }
    bricks.len() - is_unique_support.len()
}

fn part2(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut bricks: Vec<_> = data.lines().map(Brick::from).collect();
    bricks.sort_by_key(|brick| brick.start.z);

    let mut is_supported_by = HashMap::new();
    let mut is_support_for = HashMap::new();

    let mut summit = [[(0, None); 10]; 10];
    let mut supports = HashSet::new();
    for (i, brick) in bricks.iter().enumerate() {
        let Brick { start, end } = brick;
        // find where, and on which bricks this bricks will fall
        supports.clear();
        let mut max_height = 0;
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let (height, support) = summit[x as usize][y as usize];
                if height > max_height {
                    supports.clear();
                    max_height = height;
                }
                if height == max_height {
                    if let Some(support) = support {
                        supports.insert(support);
                    }
                }
            }
        }
        // update the information about the highest bricks
        let brick_h = brick.end.z - brick.start.z + 1;
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                summit[x as usize][y as usize] = (max_height + brick_h, Some(i));
            }
        }
        // update information about which bricks support which
        is_supported_by.insert(i, supports.iter().copied().collect::<Vec<_>>());
        for support in supports.iter().copied() {
            let supported = is_support_for.entry(support).or_insert_with(HashSet::new);
            supported.insert(i);
        }
    }

    let mut total = 0;
    let mut q = Vec::new();
    let mut falls = HashSet::new();
    for i in 0..bricks.len() {
        let mut sub_total = 0;
        falls.clear();
        falls.insert(i);
        q.clear();
        let Some(supported) = is_support_for.get(&i) else {
            continue;
        };
        for j in supported {
            q.push(*j);
        }
        while let Some(i) = q.pop() {
            // check whether there are remaining supports for i
            let Some(supports) = is_supported_by.get(&i) else {
                continue;
            };
            if !supports.iter().all(|support| falls.contains(support)) {
                continue;
            }
            // i falls
            falls.insert(i);
            sub_total += 1;
            // propagate the fall
            let Some(supported) = is_support_for.get(&i) else {
                continue;
            };
            for j in supported {
                q.push(*j);
            }
        }
        total += sub_total;
    }

    total
}

fn main() {
    assert_eq!(part1("example"), 5);
    assert_eq!(part1("input"), 477);

    assert_eq!(part2("example"), 7);
    assert_eq!(part2("input"), 61555);
}
