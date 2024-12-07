use std::cmp::Ord;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn read(s: &str) -> Self {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse().unwrap();
        Coord { x, y }
    }

    fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn tuning_frequency(&self) -> i64 {
        self.x * 4_000_000 + self.y
    }
}

struct Sensor {
    position: Coord,
    cell_radius: u64,
}

impl Sensor {
    fn read(s: &str) -> Self {
        let (position, beacon) = s.split_once(": ").unwrap();
        let position = Coord::read(position.strip_prefix("Sensor at ").unwrap());
        let beacon = Coord::read(beacon.strip_prefix("closest beacon is at ").unwrap());
        let radius = position.distance(&beacon);
        Sensor {
            position,
            cell_radius: radius,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum BoundType {
    Open,
    Close,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct IntervalBound {
    position: i64,
    kind: BoundType,
}

impl IntervalBound {
    fn open(position: i64) -> Self {
        IntervalBound {
            position,
            kind: BoundType::Open,
        }
    }
    fn close(position: i64) -> Self {
        IntervalBound {
            position,
            kind: BoundType::Close,
        }
    }
}

fn count_not_beacons_at(filename: &str, y: i64) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let sensors: Vec<Sensor> = contents.lines().map(Sensor::read).collect();
    let mut bounds = Vec::new();
    for sensor in sensors {
        let dy = sensor.position.y.abs_diff(y);
        if sensor.cell_radius < dy {
            continue;
        }
        let rem = (sensor.cell_radius - dy) as i64;
        bounds.push(IntervalBound::open(sensor.position.x - rem));
        bounds.push(IntervalBound::close(sensor.position.x + rem));
    }
    bounds.sort();

    let mut count = 0;
    let mut depth = 0u64;
    let mut first_open = None;
    for bound in bounds {
        match bound.kind {
            BoundType::Open => {
                if depth == 0 {
                    first_open = Some(bound.position);
                }
                depth += 1;
            }
            BoundType::Close => {
                depth -= 1;
                if depth == 0 {
                    count += bound.position - first_open.unwrap();
                    first_open = None;
                }
            }
        };
    }
    count
}

fn find_distress_beacon(filename: &str, size: i64) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let sensors: Vec<Sensor> = contents.lines().map(Sensor::read).collect();

    for y in 0..size {
        let mut bounds = Vec::new();
        for sensor in &sensors {
            let dy = sensor.position.y.abs_diff(y);
            if sensor.cell_radius < dy {
                continue;
            }
            let rem = (sensor.cell_radius - dy) as i64;
            bounds.push((sensor.position.x - rem, sensor.position.x + rem));
        }
        bounds.sort();

        let mut cover_stop = 0;
        for (start, end) in bounds {
            if start > cover_stop {
                let c = Coord { x: cover_stop, y };
                return c.tuning_frequency();
            }
            if end > cover_stop {
                cover_stop = end + 1;
            }
            if cover_stop >= size {
                break;
            }
        }
    }
    unreachable!();
}

fn puzzle1() {
    assert_eq!(count_not_beacons_at("example", 10), 26);
    assert_eq!(count_not_beacons_at("input", 2_000_000), 4_582_667);
}

fn puzzle2() {
    assert_eq!(find_distress_beacon("example", 20), 56000011);
    assert_eq!(find_distress_beacon("input", 4_000_000), 10961118625406);
}

fn main() {
    puzzle1();
    puzzle2();
}
