use std::collections::HashSet;

type Roll = (isize, isize);
type RollSet = HashSet<Roll>;

fn parse_roll_set(input: &str) -> RollSet {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut ret = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == b'@' {
                ret.insert((i as isize, j as isize));
            }
        }
    }
    ret
}

fn removable(roll_set: &RollSet) -> Vec<Roll> {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ret = Vec::new();
    for &(i, j) in roll_set {
        let n = dirs.iter().filter(|&(di, dj)| roll_set.contains(&(i + di, j + dj))).count();
        if n < 4 {
            ret.push((i, j));
        }
    }
    ret
}

fn part1(input: &str) -> usize {
    let roll_set = parse_roll_set(input);
    removable(&roll_set).len()
}

fn part2(input: &str) -> usize {
    let mut roll_set = parse_roll_set(input);
    let mut ret = 0;
    loop {
        let rolls = removable(&roll_set);
        if rolls.is_empty() {
            break;
        }
        ret += rolls.len();
        for roll in rolls {
            roll_set.remove(&roll);
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day4.txt");
    const INPUT: &str = include_str!("../inputs/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
        assert_eq!(part1(INPUT), 1527);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
        assert_eq!(part2(INPUT), 8690);
    }
}
