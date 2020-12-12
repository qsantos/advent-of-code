extern crate bytecount;
extern crate regex;

use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_input() -> Vec::<String> {
    let f = File::open("input").expect("Could not open file");
    return BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();
}

fn puzzle1(passwords: &Vec::<String>) {
    let mut n_valid_passwords = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    for password in passwords {
        let m = re.captures(password).unwrap();
        let lowest = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let highest = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = m.get(3).unwrap().as_str().as_bytes()[0];
        let password = m.get(4).unwrap().as_str();
        let count = bytecount::count(password.as_bytes(), letter);
        if lowest <= count && count <= highest {
            n_valid_passwords += 1;
        }
    }
    println!("{}", n_valid_passwords);
}

fn puzzle2(passwords: &Vec::<String>) {
    let mut n_valid_passwords = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    for password in passwords {
        let m = re.captures(password).unwrap();
        let first = m.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let second = m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let letter = m.get(3).unwrap().as_str().as_bytes()[0];
        let password = m.get(4).unwrap().as_str();
        let first_ok = password.as_bytes()[first] == letter;
        let second_ok = password.as_bytes()[second] == letter;
        if first_ok != second_ok {
            n_valid_passwords += 1;
        }
    }
    println!("{}", n_valid_passwords);
}

fn main() {
    let passwords = read_input();
    puzzle1(&passwords);
    puzzle2(&passwords);
}
