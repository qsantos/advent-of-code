use std::collections::{HashMap, HashSet};

struct Elves {
    elves: HashSet<(i64, i64)>,
    round: usize,
}

impl Elves {
    fn from(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut elves = HashSet::new();
        for (i, line) in contents.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => {
                        elves.insert((i as i64, j as i64));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Elves { elves, round: 0 }
    }

    #[allow(dead_code)]
    fn display(&self) {
        let elves = &self.elves;
        let min_i = elves.iter().map(|&(i, _)| i).min().unwrap();
        let max_i = elves.iter().map(|&(i, _)| i).max().unwrap();
        let min_j = elves.iter().map(|&(_, j)| j).min().unwrap();
        let max_j = elves.iter().map(|&(_, j)| j).max().unwrap();
        for i in min_i..=max_i {
            println!(
                "{}",
                (min_j..=max_j)
                    .map(|j| if elves.contains(&(i, j)) { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }

    fn count_empty(&self) -> usize {
        let elves = &self.elves;
        let min_i = elves.iter().map(|&(i, _)| i).min().unwrap();
        let max_i = elves.iter().map(|&(i, _)| i).max().unwrap();
        let min_j = elves.iter().map(|&(_, j)| j).min().unwrap();
        let max_j = elves.iter().map(|&(_, j)| j).max().unwrap();
        ((max_i - min_i + 1) * (max_j - min_j + 1)) as usize - elves.len()
    }

    fn choose(&self, (i, j): (i64, i64)) -> (i64, i64) {
        let elves = &self.elves;
        if (i - 1..=i + 1)
            .all(|ci| (j - 1..=j + 1).all(|cj| (ci, cj) == (i, j) || !elves.contains(&(ci, cj))))
        {
            // no other elves around
            return (i, j);
        }

        for choice_base in 0..4 {
            let choice = (choice_base + self.round) % 4;
            let checks = match choice {
                0 => [(i - 1, j), (i - 1, j - 1), (i - 1, j + 1)],
                1 => [(i + 1, j), (i + 1, j - 1), (i + 1, j + 1)],
                2 => [(i, j - 1), (i - 1, j - 1), (i + 1, j - 1)],
                3 => [(i, j + 1), (i - 1, j + 1), (i + 1, j + 1)],
                _ => unreachable!(),
            };
            if checks.iter().all(|check| !elves.contains(check)) {
                return checks[0];
            }
        }

        (i, j)
    }

    fn round(&mut self) -> bool {
        let mut choices = HashMap::new();
        for &elf in self.elves.iter() {
            let choice = self.choose(elf);
            choices.entry(choice).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut at_least_one_elf_moved = false;
        let mut next_elves = HashSet::new();
        for &elf in self.elves.iter() {
            let choice = self.choose(elf);
            let choice = if choices[&choice] == 1 { choice } else { elf };
            if choice != elf {
                at_least_one_elf_moved = true;
            }
            next_elves.insert(choice);
        }
        if !at_least_one_elf_moved {
            return false;
        }
        self.elves = next_elves;
        self.round += 1;
        true
    }
}

fn do_rounds(filename: &str, count: usize) -> usize {
    let mut elves = Elves::from(filename);
    for _ in 0..count {
        elves.round();
    }
    elves.count_empty()
}

fn count_rounds(filename: &str) -> usize {
    let mut elves = Elves::from(filename);
    while elves.round() {}
    elves.round + 1
}

fn main() {
    // puzzle 1
    assert_eq!(do_rounds("example", 10), 110);
    assert_eq!(do_rounds("input", 10), 4056);

    // puzzle 1
    assert_eq!(count_rounds("example"), 20);
    assert_eq!(count_rounds("input"), 999);
}
