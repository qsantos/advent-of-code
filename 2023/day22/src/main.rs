use std::collections::HashSet;

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

fn main() {
    assert_eq!(part1("example"), 5);
    assert_eq!(part1("input"), 477);
}
