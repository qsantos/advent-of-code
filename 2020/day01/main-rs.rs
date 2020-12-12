use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

fn read_input() -> HashSet::<u64> {
    let f = File::open("input")
        .expect("Could not read file");
    return BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse().expect("Not an uint"))
        .collect();
}

fn puzzle1(values: &HashSet::<u64>) {
    let target = 2020;
    for a in values {
        let b = target - a;
        if values.contains(&b) {
            println!("{}", a * b);
            break;
        }
    }
}

fn puzzle2(values: &HashSet::<u64>) {
    let target = 2020;
    let seen = HashSet::<u64>::new();
    for a in values {
        for b in &seen {
            let c = target - a - b;
            if seen.contains(&c) {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}

fn main() {
    let values = read_input();

    puzzle1(&values);
    puzzle2(&values);
}
