use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn read(s: &str) -> Self {
        let mut parts = s.split(',');
        Coord {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
        }
    }
}

struct Wall {
    corners: Vec<Coord>,
}

impl Wall {
    fn read(s: &str) -> Self {
        Wall {
            corners: s.split(" -> ").map(Coord::read).collect(),
        }
    }
}

struct World {
    occupied: HashSet<Coord>,
    lowest_wall: usize,
}

impl World {
    fn read(s: &str) -> Self {
        let mut occupied = HashSet::new();
        let walls: Vec<Wall> = s.lines().map(Wall::read).collect();
        for wall in &walls {
            let mut previous_corner = &wall.corners[0];
            for corner in &wall.corners[1..] {
                if corner.x == previous_corner.x {
                    // vertical
                    let a = previous_corner.y.min(corner.y);
                    let b = previous_corner.y.max(corner.y);
                    for y in a..=b {
                        occupied.insert(Coord { x: corner.x, y });
                    }
                } else if corner.y == previous_corner.y {
                    // horizontal
                    let a = previous_corner.x.min(corner.x);
                    let b = previous_corner.x.max(corner.x);
                    for x in a..=b {
                        occupied.insert(Coord { x, y: corner.y });
                    }
                } else {
                    unreachable!();
                }
                previous_corner = corner;
            }
        }
        let lowest_wall = walls
            .iter()
            .map(|wall| wall.corners.iter().map(|corner| corner.y).max().unwrap())
            .max()
            .unwrap();
        World {
            occupied,
            lowest_wall,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        let min_x = self.occupied.iter().map(|c| c.x).min().unwrap();
        let max_x = self.occupied.iter().map(|c| c.x).max().unwrap();
        let min_y = self.occupied.iter().map(|c| c.y).min().unwrap();
        let max_y = self.occupied.iter().map(|c| c.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = Coord { x, y };
                if self.occupied.contains(&c) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn read_file(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename).unwrap();
        World::read(&contents)
    }

    fn fill_with_sand_without_bottom(&mut self) -> u64 {
        fn aux(world: &mut World, c: Coord) -> (u64, bool) {
            if world.occupied.contains(&c) {
                return (0, true);
            }
            if c.y >= world.lowest_wall {
                return (0, false);
            }
            let mut ret = 0;
            let (below, stable) = aux(world, Coord { x: c.x, y: c.y + 1 });
            ret += below;
            if !stable {
                return (ret, false);
            }
            let (left, stable) = aux(
                world,
                Coord {
                    x: c.x - 1,
                    y: c.y + 1,
                },
            );
            ret += left;
            if !stable {
                return (ret, false);
            }
            let (right, stable) = aux(
                world,
                Coord {
                    x: c.x + 1,
                    y: c.y + 1,
                },
            );
            ret += right;
            if !stable {
                return (ret, false);
            }
            world.occupied.insert(c);
            ret += 1;
            (ret, true)
        }
        let (count, stable) = aux(self, Coord { x: 500, y: 0 });
        assert!(!stable);
        count
    }

    fn fill_with_sand_with_bottom(&mut self) -> u64 {
        fn fill_from(world: &mut World, c: Coord) -> u64 {
            if world.occupied.contains(&c) || c.y == world.lowest_wall + 2 {
                return 0;
            }
            let mut ret = 0;
            let below = Coord { x: c.x, y: c.y + 1 };
            ret += fill_from(world, below);
            let left = Coord {
                x: c.x - 1,
                y: c.y + 1,
            };
            ret += fill_from(world, left);
            let right = Coord {
                x: c.x + 1,
                y: c.y + 1,
            };
            ret += fill_from(world, right);
            world.occupied.insert(c);
            ret += 1;
            ret
        }
        fill_from(self, Coord { x: 500, y: 0 })
    }
}

fn count_without_bottom(filename: &str) -> u64 {
    World::read_file(filename).fill_with_sand_without_bottom()
}

fn count_with_bottom(filename: &str) -> u64 {
    World::read_file(filename).fill_with_sand_with_bottom()
}

fn puzzle1() {
    assert_eq!(count_without_bottom("example"), 24);
    assert_eq!(count_without_bottom("input"), 655);
}

fn puzzle2() {
    assert_eq!(count_with_bottom("example"), 93);
    assert_eq!(count_with_bottom("input"), 26484);
}

fn main() {
    puzzle1();
    puzzle2();
}
