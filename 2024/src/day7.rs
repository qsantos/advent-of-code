use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let left: u64 = left.parse().unwrap();
        let right: Vec<u64> = right.split(' ').map(|x| x.parse().unwrap()).collect();
        fn aux(left: u64, right: &[u64], i: usize, acc: u64) -> bool {
            if i == right.len() {
                return acc == left;
            }
            if acc > left {
                return false;
            }
            aux(left, right, i + 1, acc + right[i]) || aux(left, right, i + 1, acc * right[i])
        }
        if aux(left, &right, 1, right[0]) {
            total += left;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day7.txt");
    const INPUT: &str = include_str!("../inputs/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "3749");
        assert_eq!(part1(INPUT).to_string(), "882304362421");
    }
}
