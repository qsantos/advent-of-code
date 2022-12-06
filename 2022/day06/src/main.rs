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

fn marker_position(filename: &str, marker_length: usize) -> usize {
    let chars: Vec<char> = std::fs::read_to_string(filename).unwrap().chars().collect();
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

fn puzzle1() {
    assert_eq!(marker_position("example1", 4), 7);
    assert_eq!(marker_position("example2", 4), 5);
    assert_eq!(marker_position("example3", 4), 6);
    assert_eq!(marker_position("example4", 4), 10);
    assert_eq!(marker_position("example5", 4), 11);
    assert_eq!(marker_position("input", 4), 1757);
}

fn puzzle2() {
    assert_eq!(marker_position("example1", 14), 19);
    assert_eq!(marker_position("example2", 14), 23);
    assert_eq!(marker_position("example3", 14), 23);
    assert_eq!(marker_position("example4", 14), 29);
    assert_eq!(marker_position("example5", 14), 26);
    assert_eq!(marker_position("input", 14), 2950);
}

fn main() {
    puzzle1();
    puzzle2();
}
