use std::collections::HashMap;

const WIDTH: i64 = 7;
const MARGIN_LEFT: i64 = 2;
const MARGIN_BOTTOM: i64 = 3;
const TALLEST_ROCK: i64 = 4;

struct Rock {
    parts: Vec<(i64, i64)>,
}

impl Rock {
    fn from(parts: &[(i64, i64)]) -> Self {
        Rock {
            parts: Vec::from(parts),
        }
    }

    fn shift(&self, dx: i64, dy: i64) -> Self {
        Rock {
            parts: self.parts.iter().map(|(x, y)| (x + dx, y + dy)).collect(),
        }
    }

    fn can_be_there(&self, occupied: &[i64], occupied_bottom: i64) -> bool {
        self.parts.iter().all(|&(x, y)| {
            (0..WIDTH).contains(&x)
                && y >= 0
                && occupied[(y - occupied_bottom) as usize] & (1 << x) == 0
        })
    }
}

fn measure_tower(filename: &str, iterations: i64) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let jets: Vec<i64> = contents
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => unreachable!(),
        })
        .collect();

    let rocks = [
        // ####
        Rock::from(&[(0, 0), (1, 0), (2, 0), (3, 0)]),
        // .#.
        // ###  note that the center is not useful
        // .#.
        Rock::from(&[(0, 1), (1, 0), /* (1, 1), */ (1, 2), (2, 1)]),
        // ..#
        // ..#
        // ###
        Rock::from(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        // #
        // #
        // #
        // #
        Rock::from(&[(0, 0), (0, 1), (0, 2), (0, 3)]),
        // ##
        // ##
        Rock::from(&[(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    // map state to (iteration, tower height)
    let mut last_seen = HashMap::new();

    let mut occupied = Vec::new();
    occupied.resize(50, 0);
    let mut occupied_bottom = MARGIN_BOTTOM + TALLEST_ROCK - (occupied.len() as i64);
    let mut top = 0;
    let mut current_rock = 0;
    let mut current_jet = 0;

    let mut iteration = 0;
    let mut found_cycle = false;
    while iteration < iterations {
        let mut rock = rocks[current_rock].shift(MARGIN_LEFT, top + MARGIN_BOTTOM);
        current_rock = (current_rock + 1) % rocks.len();

        loop {
            let jet = jets[current_jet];
            current_jet = (current_jet + 1) % jets.len();

            let next_rock = rock.shift(jet, 0);
            if next_rock.can_be_there(&occupied, occupied_bottom) {
                rock = next_rock;
            }

            let next_rock = rock.shift(0, -1);
            if !next_rock.can_be_there(&occupied, occupied_bottom) {
                break;
            }
            rock = next_rock;
        }

        for (x, y) in rock.parts {
            if y >= top {
                top = y + 1;
            }
            occupied[(y - occupied_bottom) as usize] |= 1 << x;
        }

        let needed_shift = occupied
            [((occupied.len() as i64 - TALLEST_ROCK - MARGIN_BOTTOM) as usize)..]
            .iter()
            .take_while(|&&x| x != 0)
            .count();
        for _ in 0..needed_shift {
            occupied.remove(0);
            occupied.push(0);
        }
        occupied_bottom += needed_shift as i64;

        iteration += 1;

        if !found_cycle {
            if let Some((previous_iteration, previous_top)) = last_seen.get(&occupied) {
                let cycle_length = iteration - previous_iteration;
                let cycle_height = top - previous_top;
                let remaining_cycles = (iterations - iteration) / cycle_length;
                iteration += remaining_cycles * cycle_length;
                top += remaining_cycles * cycle_height;
                occupied_bottom += remaining_cycles * cycle_height;
                found_cycle = true;
            } else {
                last_seen.insert(occupied.clone(), (iteration, top));
            }
        }
    }
    top
}

fn puzzle1() {
    let count = 2022;
    assert_eq!(measure_tower("example", count), 3068);
    assert_eq!(measure_tower("input", count), 3206);
}

fn puzzle2() {
    let count = 1_000_000_000_000;
    assert_eq!(measure_tower("example", count), 1_514_285_714_288);
    assert_eq!(measure_tower("input", count), 1_602_881_844_347);
}

fn main() {
    puzzle1();
    puzzle2();
}
