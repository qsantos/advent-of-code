#[derive(Clone, Debug, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn from(s: &str) -> Self {
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        let x = x.trim().parse().unwrap();
        let y = y.trim().parse().unwrap();
        let z = z.trim().parse().unwrap();
        Vec3 { x, y, z }
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
}

fn crossing_time(a: &Hailstone, b: &Hailstone) -> f32 {
    let ta1 = b.vel.y * (b.pos.x - a.pos.x) - b.vel.x * (b.pos.y - a.pos.y);
    let ta2 = b.vel.y * a.vel.x - b.vel.x * a.vel.y;
    ta1 / ta2
}

fn intersects(a: &Hailstone, b: &Hailstone, min: f32, max: f32) -> bool {
    let ta = crossing_time(a, b);
    let tb = crossing_time(b, a);
    if ta < 0. || tb < 0. {
        return false;
    }
    let x = a.pos.x + ta * a.vel.x;
    let y = a.pos.y + ta * a.vel.y;
    min <= x && x <= max && min <= y && y <= max
}

fn part1(filename: &str, min: f32, max: f32) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let hail: Vec<_> = data.lines().map(Hailstone::from).collect();
    let mut count = 0;
    for (i, a) in hail.iter().enumerate() {
        for b in &hail[i + 1..] {
            if intersects(a, b, min, max) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    assert_eq!(part1("example", 10., 20.), 2);
    assert_eq!(part1("input", 200000000000000., 400000000000000.), 15107);
}
