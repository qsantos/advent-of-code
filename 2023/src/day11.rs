use std::collections::HashSet;

fn complement(set: HashSet<usize>) -> HashSet<usize> {
    let min = *set.iter().min().unwrap();
    let max = *set.iter().max().unwrap();
    let mut ret = HashSet::new();
    for i in min + 1..max {
        if !set.contains(&i) {
            ret.insert(i);
        }
    }
    ret
}

fn distance(
    (ai, aj): &(usize, usize),
    (bi, bj): &(usize, usize),
    expanding_rows: &HashSet<usize>,
    expanding_cols: &HashSet<usize>,
    expansion_factor: usize,
) -> usize {
    let (ai, bi) = (ai.min(bi), ai.max(bi));
    let (aj, bj) = (aj.min(bj), aj.max(bj));
    let mut expansion = 0;
    for expanding_row in expanding_rows {
        if (ai..=bi).contains(&expanding_row) {
            expansion += expansion_factor - 1;
        }
    }
    for expanding_col in expanding_cols {
        if (aj..=bj).contains(&expanding_col) {
            expansion += expansion_factor - 1;
        }
    }
    (bi - ai) + (bj - aj) + expansion
}

fn solve(input: &str, expansion_factor: usize) -> usize {
    let mut galaxies = HashSet::new();
    let mut occupied_rows = HashSet::new();
    let mut occupied_cols = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                galaxies.insert((i, j));
                occupied_rows.insert(i);
                occupied_cols.insert(j);
            }
        }
    }
    let expanding_rows = complement(occupied_rows);
    let expanding_cols = complement(occupied_cols);
    let mut sum = 0;
    for a in galaxies.iter() {
        for b in galaxies.iter() {
            if a == b {
                continue;
            }
            sum += distance(a, b, &expanding_rows, &expanding_cols, expansion_factor);
        }
    }
    sum / 2
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day11.txt");
    const INPUT: &str = include_str!("../inputs/day11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 374);
        assert_eq!(part1(INPUT), 9734203);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(EXAMPLE, 10), 1030);
        assert_eq!(solve(EXAMPLE, 100), 8410);
        assert_eq!(part2(INPUT), 568914596391);
    }
}
