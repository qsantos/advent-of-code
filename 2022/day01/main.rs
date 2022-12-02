use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn read_elves(filename: &str) -> Vec<i32> {
    let mut ret = vec![];
    let file = File::open(filename).expect("Could not open the file");
    let reader = BufReader::new(file);
    let mut current_elf = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.len() == 0 {
            ret.push(current_elf);
            current_elf = 0;
        } else {
            let number: i32 = line.parse().expect("Could not parse line");
            current_elf += number;
        }
    }
    ret.push(current_elf);
    ret
}

fn most_caloric_elf(filename: &str) -> i32 {
    let elves = read_elves(filename);
    *elves.iter().max().expect("No elves!")
}

fn puzzle1() {
    assert_eq!(most_caloric_elf("example"), 24000);
    assert_eq!(most_caloric_elf("input"), 69310);
}

fn most_caloric_elves(filename: &str, count: usize) -> i32 {
    let mut elves = read_elves(filename);
    elves.sort();
    elves.iter().rev().take(count).sum()
}

fn puzzle2() {
    assert_eq!(most_caloric_elves("example", 3), 45000);
    println!("{}", most_caloric_elves("input", 3));
}

fn main() {
    puzzle1();
    puzzle2();
}
