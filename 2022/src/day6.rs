use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

struct MultiSet<T> {
    item_counts: HashMap<T, usize>,
}

impl<T: Eq + Hash> MultiSet<T> {
    fn new() -> Self {
        let bla = HashMap::new();
        MultiSet { item_counts: bla }
    }

    fn add(&mut self, item: T) {
        self.item_counts
            .entry(item)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn remove(&mut self, item: T) {
        match self.item_counts.get_mut(&item) {
            Some(r) => {
                if *r == 1 {
                    self.item_counts.remove(&item);
                } else {
                    *r -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn marker_position(input: &str, marker_length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut marker_characters = MultiSet::new();
    for i in 0..chars.len() {
        if marker_characters.item_counts.len() == marker_length {
            return i;
        }
        if i >= marker_length {
            marker_characters.remove(&chars[i - marker_length]);
        }
        marker_characters.add(&chars[i]);
    }
    unreachable!();
}

pub fn part1(input: &str) -> usize {
    marker_position(input, 4)
}

pub fn part2(input: &str) -> usize {
    marker_position(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day6-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day6-2.txt");
    const EXAMPLE3: &str = include_str!("../examples/day6-3.txt");
    const EXAMPLE4: &str = include_str!("../examples/day6-4.txt");
    const EXAMPLE5: &str = include_str!("../examples/day6-5.txt");
    const INPUT: &str = include_str!("../inputs/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 7);
        assert_eq!(part1(EXAMPLE2), 5);
        assert_eq!(part1(EXAMPLE3), 6);
        assert_eq!(part1(EXAMPLE4), 10);
        assert_eq!(part1(EXAMPLE5), 11);
        assert_eq!(part1(INPUT), 1757);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 19);
        assert_eq!(part2(EXAMPLE2), 23);
        assert_eq!(part2(EXAMPLE3), 23);
        assert_eq!(part2(EXAMPLE4), 29);
        assert_eq!(part2(EXAMPLE5), 26);
        assert_eq!(part2(INPUT), 2950);
    }
}
