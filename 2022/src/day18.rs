use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Cube([i32; 3]);

impl Cube {
    fn from(s: &str) -> Self {
        let parts: Vec<i32> = s.split(',').map(|s| s.parse().unwrap()).collect();
        Cube([parts[0], parts[1], parts[2]])
    }

    fn neighbors(&self) -> Vec<Cube> {
        let mut ret = Vec::new();
        for i in 0..3 {
            for d in [-1, 1] {
                let mut other = self.clone();
                other.0[i] += d;
                ret.push(other)
            }
        }
        ret
    }

    fn is_in(&self, lower_bounds: &[i32], upper_bounds: &[i32]) -> bool {
        (0..3).all(|i| (lower_bounds[i]..upper_bounds[i]).contains(&self.0[i]))
    }
}

fn surface(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let cubes: HashSet<Cube> = contents.lines().map(Cube::from).collect();
    cubes
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|other| !cubes.contains(other))
                .count()
        })
        .sum()
}

fn external_surface(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let cubes: HashSet<Cube> = contents.lines().map(Cube::from).collect();

    let lower_bounds: Vec<i32> = (0..3)
        .map(|i| cubes.iter().map(|cube| cube.0[i]).min().unwrap() - 1)
        .collect();
    let upper_bounds: Vec<i32> = (0..3)
        .map(|i| cubes.iter().map(|cube| cube.0[i]).max().unwrap() + 2)
        .collect();

    let mut q = Vec::new();
    q.push(Cube([lower_bounds[0], 0, 0]));
    let mut visited = HashSet::new();
    while let Some(cube) = q.pop() {
        if visited.contains(&cube) {
            continue;
        }
        if cubes.contains(&cube) {
            continue;
        }
        if !cube.is_in(&lower_bounds, &upper_bounds) {
            continue;
        }

        q.append(&mut cube.neighbors());
        visited.insert(cube);
    }

    cubes
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|other| visited.contains(other))
                .count()
        })
        .sum()
}

fn puzzle1() {
    assert_eq!(surface("example"), 64);
    assert_eq!(surface("input"), 4340);
}

fn puzzle2() {
    assert_eq!(external_surface("example"), 58);
    assert_eq!(external_surface("input"), 2468);
}

fn main() {
    puzzle1();
    puzzle2();
}
