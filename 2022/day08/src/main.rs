use std::collections::HashSet;

fn read_grid(filename: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn count_visible_trees(filename: &str) -> usize {
    let grid = read_grid(filename);
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

fn puzzle1() {
    assert_eq!(count_visible_trees("example"), 21);
    assert_eq!(count_visible_trees("input"), 1812);
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

fn highest_scenic_score(filename: &str) -> usize {
    let grid = read_grid(filename);
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

fn puzzle2() {
    assert_eq!(highest_scenic_score("example"), 8);
    assert_eq!(highest_scenic_score("input"), 315495);
}

fn main() {
    puzzle1();
    puzzle2();
}
