use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;

fn parse(
    input: &str,
) -> (
    HashSet<(u64, u64)>,
    impl Iterator<Item = Vec<u64>> + use<'_>,
) {
    let (orders, updates) = input.split_once("\n\n").unwrap();
    let orders = orders
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("|").unwrap();
            let left: u64 = left.parse().unwrap();
            let right: u64 = right.parse().unwrap();
            (left, right)
        })
        .collect();

    let updates = updates.lines().map(|update| {
        update
            .split(",")
            .map(|page| page.parse().unwrap())
            .collect()
    });

    (orders, updates)
}

fn is_ordered(orders: &HashSet<(u64, u64)>, pages: &[u64]) -> bool {
    for i in 0..pages.len() {
        for j in i + 1..pages.len() {
            if orders.contains(&(pages[j], pages[i])) {
                return false;
            }
        }
    }
    true
}

pub fn part1(input: &str) -> impl Display {
    let (orders, updates) = parse(input);
    let mut count = 0;
    for pages in updates {
        if is_ordered(&orders, &pages) {
            count += pages[pages.len() / 2];
        }
    }
    count
}

fn quickselect(orders: &HashSet<(u64, u64)>, pages: &mut [u64], k: usize) -> u64 {
    let mut left = 0;
    let mut right = pages.len() - 1;
    while left < right {
        let pivot = pages[right];
        let mut i = left;
        for j in left..right {
            if orders.contains(&(pages[j], pivot)) {
                pages.swap(i, j);
                i += 1;
            }
        }
        pages.swap(i, right);
        match i.cmp(&k) {
            Ordering::Equal => return pivot,
            Ordering::Less => left = i + 1,
            Ordering::Greater => right = i - 1,
        }
    }
    pages[left]
}

pub fn part2(input: &str) -> impl Display {
    let (orders, updates) = parse(input);
    let mut count = 0;
    for mut pages in updates {
        if !is_ordered(&orders, &pages) {
            let n = pages.len();
            count += quickselect(&orders, &mut pages, n / 2);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day5.txt");
    const INPUT: &str = include_str!("../inputs/day5.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "143");
        assert_eq!(part1(INPUT).to_string(), "5374");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "123");
        assert_eq!(part2(INPUT).to_string(), "4260");
    }
}
