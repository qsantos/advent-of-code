use std::collections::{BinaryHeap, HashSet};

struct Blizzard {
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
}

struct Map {
    rows: i32,
    cols: i32,
    blizzards: Vec<Blizzard>,
}

impl Map {
    fn from(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut rows = 0;
        let mut cols = 0;
        let mut blizzards = Vec::new();
        for (i, line) in contents.lines().skip(1).enumerate() {
            let i = i as i32;
            for (j, c) in line.chars().skip(1).enumerate() {
                let j = j as i32;
                match c {
                    '^' => blizzards.push(Blizzard {
                        i,
                        j,
                        di: -1,
                        dj: 0,
                    }),
                    'v' => blizzards.push(Blizzard { i, j, di: 1, dj: 0 }),
                    '<' => blizzards.push(Blizzard {
                        i,
                        j,
                        di: 0,
                        dj: -1,
                    }),
                    '>' => blizzards.push(Blizzard { i, j, di: 0, dj: 1 }),
                    _ => (),
                }
                cols = cols.max(j);
            }
            rows = rows.max(i);
        }
        Map {
            rows,
            cols,
            blizzards,
        }
    }

    fn blizzard_at(&self, time: i32, i: i32, j: i32) -> bool {
        self.blizzards.iter().any(|blizzard| {
            let bi = (blizzard.i + blizzard.di * time).rem_euclid(self.rows);
            let bj = (blizzard.j + blizzard.dj * time).rem_euclid(self.cols);
            (bi, bj) == (i, j)
        })
    }

    fn find_path(&self, start_time: i32, start: (i32, i32), goal: (i32, i32)) -> Option<i32> {
        let (si, sj) = start;
        let (gi, gj) = goal;
        let mut q = BinaryHeap::new();
        q.push((0, (start_time, si, sj)));
        let mut visited = HashSet::new();
        while let Some((_, state)) = q.pop() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);
            let (time, i, j) = state;
            // println!("{time}: {i} {j}");
            if i == gi && j == gj {
                return Some(time);
            }
            if !self.blizzard_at(time + 1, i, j) {
                q.push((
                    -(i.abs_diff(gi) as i32 + j.abs_diff(gj) as i32 + time),
                    (time + 1, i, j),
                ));
            }
            for (ni, nj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if !((0..self.rows).contains(&ni) && (0..self.cols).contains(&nj)) {
                    continue;
                }
                if self.blizzard_at(time + 1, ni, nj) {
                    continue;
                }
                q.push((
                    -(ni.abs_diff(gi) as i32 + nj.abs_diff(gj) as i32 + time),
                    (time + 1, ni, nj),
                ));
            }
        }
        None
    }
}

fn direct_path(filename: &str) -> i32 {
    let map = Map::from(filename);
    map.find_path(0, (-1, 0), (map.rows - 1, map.cols - 1))
        .unwrap()
        + 1
}

fn there_and_back_and_there_again(filename: &str) -> i32 {
    let map = Map::from(filename);
    let a = map
        .find_path(0, (-1, 0), (map.rows - 1, map.cols - 1))
        .unwrap()
        + 1;
    let b = map.find_path(a, (map.rows, map.cols - 1), (0, 0)).unwrap() + 1;
    let c = map
        .find_path(b, (-1, 0), (map.rows - 1, map.cols - 1))
        .unwrap()
        + 1;
    println!("{a} {b} {c}");
    c
}

fn main() {
    // part 1
    assert_eq!(direct_path("example"), 18);
    assert_eq!(direct_path("input"), 326);

    // part 2
    assert_eq!(there_and_back_and_there_again("example"), 54);
    assert_eq!(there_and_back_and_there_again("input"), 976);
}
