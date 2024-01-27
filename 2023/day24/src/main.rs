use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn from(s: &str) -> Self {
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        let x = x.trim().parse().unwrap();
        let y = y.trim().parse().unwrap();
        let z = z.trim().parse().unwrap();
        Vec3 { x, y, z }
    }

    fn dist2(&self, other: &Vec3) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn from(s: &str) -> Self {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let pos = Vec3::from(pos);
        let vel = Vec3::from(vel);
        Hailstone { pos, vel }
    }

    fn pos_at(&self, t: f64) -> Vec3 {
        &self.pos + &self.vel * t
    }

    fn rel_vel(&self, vel: &Vec3) -> Hailstone {
        Hailstone {
            pos: self.pos.clone(),
            vel: &self.vel - vel,
        }
    }
}

fn crossing_time_xy(a: &Hailstone, b: &Hailstone) -> f64 {
    let ta1 = b.vel.y * (b.pos.x - a.pos.x) - b.vel.x * (b.pos.y - a.pos.y);
    let ta2 = b.vel.y * a.vel.x - b.vel.x * a.vel.y;
    ta1 / ta2
}

fn intersects_xy(a: &Hailstone, b: &Hailstone, min: f64, max: f64) -> bool {
    let ta = crossing_time_xy(a, b);
    let tb = crossing_time_xy(b, a);
    if ta < 0. || tb < 0. {
        return false;
    }
    let x = a.pos.x + ta * a.vel.x;
    let y = a.pos.y + ta * a.vel.y;
    min <= x && x <= max && min <= y && y <= max
}

fn part1(filename: &str, min: f64, max: f64) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let hail: Vec<_> = data.lines().map(Hailstone::from).collect();
    let mut count = 0;
    for (i, a) in hail.iter().enumerate() {
        for b in &hail[i + 1..] {
            if intersects_xy(a, b, min, max) {
                count += 1;
            }
        }
    }
    count
}

fn evaluate_candidate_vel_xyz(
    hail: &[Hailstone],
    a: &Hailstone,
    ta: f64,
    svx: f64,
    svy: f64,
    svz: f64,
) -> Option<Hailstone> {
    let vel = Vec3::new(svx, svy, svz);
    // find out where a and b should cross
    let a = a.rel_vel(&vel);
    let pos = a.pos_at(ta);
    // check whether that works with other hailstones
    let rock = Hailstone { pos, vel };
    for c in &hail[1..] {
        let tc = crossing_time_xy(c, &rock);
        if c.pos_at(tc).dist2(&rock.pos_at(tc)) > 0.1 {
            return None;
        }
    }
    Some(rock)
}

fn evaluate_candidate_vel_xy(
    hail: &[Hailstone],
    svx: f64,
    svy: f64,
    range: i64,
) -> Option<Hailstone> {
    // find two non-aligned hailstones to find their intersection
    let a = &hail[0];
    let b = &hail[1];

    // find out when the rock will hit a
    let vel_xy = Vec3::new(svx, svy, 0.);
    let ta = crossing_time_xy(&a.rel_vel(&vel_xy), &b.rel_vel(&vel_xy));
    if !ta.is_finite() {
        // a and b are not meeting at all
        return None;
    }
    if (ta.round() - ta).abs() > 0.1 {
        // not an integral time
        return None;
    }

    // try all possible svz until one matches
    (-range..=range)
        .flat_map(|svz| evaluate_candidate_vel_xyz(hail, a, ta, svx, svy, svz as f64))
        .next()
}

fn part2(filename: &str) -> f64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let hail: Vec<_> = data.lines().map(Hailstone::from).collect();
    let mut range = 1;
    loop {
        for svx in -range..=range {
            for svy in -range..=range {
                if let Some(rock) = evaluate_candidate_vel_xy(&hail, svx as f64, svy as f64, range)
                {
                    return rock.pos.x + rock.pos.y + rock.pos.z;
                }
            }
        }
        range *= 2;
    }
}

fn main() {
    assert_eq!(part1("example", 10., 20.), 2);
    assert_eq!(part1("input", 200000000000000., 400000000000000.), 15107);

    assert_eq!(part2("example"), 47.);
    assert_eq!(part2("input"), 856642398547748.);
}
