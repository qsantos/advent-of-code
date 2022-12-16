use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn read(s: &str) -> Self {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse().unwrap();
        Coord { x, y }
    }

    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Sensor {
    position: Coord,
    closest_beacon: Coord,
    cell_radius: u32,
}

impl Sensor {
    fn read(s: &str) -> Self {
        let (position, beacon) = s.split_once(": ").unwrap();
        let position = Coord::read(position.strip_prefix("Sensor at ").unwrap());
        let beacon = Coord::read(beacon.strip_prefix("closest beacon is at ").unwrap());
        let radius = position.distance(&beacon);
        Sensor {
            position,
            closest_beacon: beacon,
            cell_radius: radius,
        }
    }
}

fn count_not_beacons_at(filename: &str, y: i32) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let sensors: Vec<Sensor> = contents.lines().map(Sensor::read).collect();
    let beacons: HashSet<&Coord> = sensors
        .iter()
        .map(|sensor| &sensor.closest_beacon)
        .collect();

    let min_x = sensors
        .iter()
        .map(|sensor| {
            sensor.position.x - (sensor.cell_radius as i32 - sensor.position.y.abs_diff(y) as i32)
        })
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|sensor| {
            sensor.position.x + (sensor.cell_radius as i32 - sensor.position.y.abs_diff(y) as i32)
        })
        .max()
        .unwrap();

    (min_x..=max_x)
        .filter(|&x| {
            let c = Coord { x, y };
            !beacons.contains(&c)
                && sensors
                    .iter()
                    .any(|sensor| c.distance(&sensor.position) <= sensor.cell_radius)
        })
        .count()
}

fn puzzle1() {
    assert_eq!(count_not_beacons_at("example", 10), 26);
    assert_eq!(count_not_beacons_at("input", 2_000_000), 4_582_667);
}

fn puzzle2() {
    // println!("{}", count_not_beacons_at("example", 10));
    // println!("{}", count_not_beacons_at("input", 2_000_000));
}

fn main() {
    puzzle1();
    puzzle2();
}
