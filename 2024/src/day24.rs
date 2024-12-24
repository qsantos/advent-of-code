use std::collections::HashMap;
use std::fmt::Display;

fn eval<'a>(
    values: &'_ mut HashMap<&'a str, bool>,
    gates: &'a HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    output: &'a str,
) -> bool {
    if let Some(&value) = values.get(output) {
        return value;
    }
    let &(lhs, op, rhs) = gates.get(output).unwrap();
    let lhs = eval(values, gates, lhs);
    let rhs = eval(values, gates, rhs);
    let value = match op {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => panic!("Unknown operator: {}", op),
    };
    values.insert(output, value);
    value
}

pub fn part1(input: &str) -> impl Display {
    let (initial_values, gates) = input.trim().split_once("\n\n").unwrap();
    let mut values: HashMap<&str, bool> = initial_values
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let value = value == "1";
            (name, value)
        })
        .collect();
    let gates: HashMap<&str, (&str, &str, &str)> = gates
        .lines()
        .map(|line| {
            let (gate, res) = line.split_once(" -> ").unwrap();
            let (lhs, rest) = gate.split_once(" ").unwrap();
            let (op, rhs) = rest.split_once(" ").unwrap();
            (res, (lhs, op, rhs))
        })
        .collect();
    let mut outputs: Vec<&str> = gates
        .keys()
        .copied()
        .filter(|&key| key.starts_with('z'))
        .collect();
    outputs.sort();
    outputs.reverse();
    let mut res = 0u64;
    for output in outputs {
        res <<= 1;
        if eval(&mut values, &gates, output) {
            res |= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day24-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day24-2.txt");
    const INPUT: &str = include_str!("../inputs/day24.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "4");
        assert_eq!(part1(EXAMPLE2).to_string(), "2024");
        assert_eq!(part1(INPUT).to_string(), "58740594706150");
    }
}
