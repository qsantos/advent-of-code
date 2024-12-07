// some optimization ideas from https://old.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions/j1xy1ye/
use std::collections::{HashSet, VecDeque};

use regex::Regex;

fn index_of_material(s: &str) -> usize {
    match s {
        "ore" => 0,
        "clay" => 1,
        "obsidian" => 2,
        "geode" => 3,
        _ => unreachable!(),
    }
}

struct Blueprint {
    costs: [[u32; 4]; 4],
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let mut costs = [[0; 4]; 4];
        let regex = Regex::new(r"Each (.*?) robot costs (.*?)\.").unwrap();
        // iterate over robot types
        for m in regex.captures_iter(s) {
            let robot = index_of_material(&m[1]);
            // iterate over ingredients
            for x in m[2].split(" and ") {
                let (amount, material) = x.split_once(' ').unwrap();
                let amount: u32 = amount.parse().unwrap();
                let material = index_of_material(material);
                costs[robot][material] = amount;
            }
        }
        Blueprint { costs }
    }

    fn max_geodes(&self, timeout: u32) -> u32 {
        // no point is making more than X MATERIAL robots if no robot needs more than X MATERIAL
        let mut max_robots = [0; 4];
        for material in 0..3 {
            max_robots[material] = self
                .costs
                .map(|costs| costs[material])
                .iter()
                .max()
                .copied()
                .unwrap();
        }
        // of course, except for obsidian
        max_robots[3] = std::u32::MAX;

        let mut best = 0;
        let mut q = VecDeque::new();
        q.push_back((0, [0; 4], [1, 0, 0, 0], [false; 4]));
        let mut visited = HashSet::new();
        while let Some(state) = q.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            let (elapsed, mut amounts, counts, was_buildable) = state;

            // can we beat the best if we build a geode robot at every minute?
            let remaining_time = timeout - elapsed;
            let geode_upper_bound = amounts[3] + remaining_time * (remaining_time + 1) / 2;
            if geode_upper_bound < best {
                continue;
            }

            best = best.max(amounts[3]);
            if elapsed >= timeout {
                continue;
            }
            let mut is_buildable = [false; 4];
            #[allow(clippy::needless_range_loop)]
            for robot in 0..4 {
                if self.costs[robot]
                    .iter()
                    .zip(amounts)
                    .all(|(&cost, amount)| amount >= cost)
                {
                    is_buildable[robot] = true;
                }
            }
            for material in 0..4 {
                amounts[material] += counts[material];
            }
            if is_buildable[3] {
                let mut new_counts = counts;
                new_counts[3] += 1;
                let mut new_amounts = amounts;
                #[allow(clippy::needless_range_loop)]
                for material in 0..4 {
                    new_amounts[material] -= self.costs[3][material];
                }
                q.push_back((elapsed + 1, new_amounts, new_counts, [false; 4]));
                continue;
            }
            q.push_back((elapsed + 1, amounts, counts, is_buildable));
            for robot in 0..3 {
                if is_buildable[robot] && !was_buildable[robot] && counts[robot] < max_robots[robot]
                {
                    let mut new_counts = counts;
                    new_counts[robot] += 1;
                    let mut new_amounts = amounts;
                    #[allow(clippy::needless_range_loop)]
                    for material in 0..4 {
                        new_amounts[material] -= self.costs[robot][material];
                    }
                    q.push_back((elapsed + 1, new_amounts, new_counts, [false; 4]));
                }
            }
        }
        best
    }
}

fn total_quality_level(filename: &str) -> u32 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let blueprints = contents.lines().map(Blueprint::parse);
    blueprints
        .enumerate()
        .map(|(i, blueprint)| blueprint.max_geodes(24) * (i as u32 + 1))
        .sum()
}

fn first_three(filename: &str) -> u32 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let blueprints = contents.lines().map(Blueprint::parse);
    blueprints
        .take(3)
        .map(|blueprint| blueprint.max_geodes(32))
        .product()
}

fn puzzle1() {
    assert_eq!(total_quality_level("example"), 33);
    assert_eq!(total_quality_level("input"), 1009);
}

fn puzzle2() {
    assert_eq!(first_three("example"), 3472);
    assert_eq!(first_three("input"), 18816);
}

fn main() {
    puzzle1();
    puzzle2();
}
