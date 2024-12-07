use std::collections::HashSet;

fn read_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let grid = read_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visible_trees = HashSet::new();

    // horizontal
    #[allow(clippy::needless_range_loop)]
    for i in 0..rows {
        // left-to-right
        let mut current_max_height = -1;
        for j in 0..cols {
            let tree = grid[i][j];
            if tree > current_max_height {
                visible_trees.insert((i, j));
                current_max_height = tree;
            }
        }
        // right-to-left
        current_max_height = -1;
        for j in (0..cols).rev() {
            let tree = grid[i][j];
            if tree > current_max_height {
                visible_trees.insert((i, j));
                current_max_height = tree;
            }
        }
    }

    // vertical
    for j in 0..cols {
        // top-down
        let mut current_max_height = -1;
        #[allow(clippy::needless_range_loop)]
        for i in 0..rows {
            let tree = grid[i][j];
            if tree > current_max_height {
                visible_trees.insert((i, j));
                current_max_height = tree;
            }
        }
        // bottom-up
        current_max_height = -1;
        for i in (0..rows).rev() {
            let tree = grid[i][j];
            if tree > current_max_height {
                visible_trees.insert((i, j));
                current_max_height = tree;
            }
        }
    }

    visible_trees.len()
}

struct TakeWhileInclusive<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> TakeWhileInclusive<I, P> {
    fn new(iter: I, predicate: P) -> Self {
        TakeWhileInclusive {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I, P> Iterator for TakeWhileInclusive<I, P>
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            let item = self.iter.next()?;
            if !(self.predicate)(&item) {
                self.flag = true;
            }
            Some(item)
        }
    }
}

fn take_while_inclusive<I, F>(iter: I, predicate: F) -> TakeWhileInclusive<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    TakeWhileInclusive::new(iter, predicate)
}

pub fn part2(input: &str) -> usize {
    let grid = read_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| {
                    let tree = grid[i][j];
                    let up = take_while_inclusive((0..i).rev(), |a| grid[*a][j] < tree).count();
                    let down = take_while_inclusive(i + 1..rows, |a| grid[*a][j] < tree).count();
                    let left = take_while_inclusive((0..j).rev(), |b| grid[i][*b] < tree).count();
                    let right = take_while_inclusive(j + 1..cols, |b| grid[i][*b] < tree).count();
                    up * down * left * right
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day8.txt");
    const INPUT: &str = include_str!("../inputs/day8.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
        assert_eq!(part1(INPUT), 1812);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 8);
        assert_eq!(part2(INPUT), 315495);
    }
}
