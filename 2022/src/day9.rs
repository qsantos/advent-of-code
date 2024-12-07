use std::collections::HashSet;

fn count_tail_visits(input: &str, length: usize) -> usize {
    let mut visited_positions = HashSet::new();
    let mut knots = [(0i32, 0i32)].repeat(length + 1);
    visited_positions.insert(*knots.last().unwrap());
    for line in input.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        assert_eq!(command.len(), 2);
        let direction = command[0];
        let (di, dj) = match direction {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => unreachable!(),
        };
        let n_steps: u32 = command[1].parse().unwrap();
        for _ in 0..n_steps {
            let (mut hi, mut hj) = knots[0];
            hi += di;
            hj += dj;
            knots[0] = (hi, hj);

            for knot in &mut knots[1..] {
                let (mut ti, mut tj) = knot;
                if hi.abs_diff(ti) > 1 || hj.abs_diff(tj) > 1 {
                    ti += (hi - ti).signum();
                    tj += (hj - tj).signum();
                    *knot = (ti, tj);
                }
                hi = ti;
                hj = tj;
            }
            visited_positions.insert(*knots.last().unwrap());
            // println!("head:{hi},{hj} tail:{ti},{tj}");
        }
    }
    visited_positions.len()
}

pub fn part1(input: &str) -> usize {
    count_tail_visits(input, 1)
}

pub fn part2(input: &str) -> usize {
    count_tail_visits(input, 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day9-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day9-2.txt");
    const INPUT: &str = include_str!("../inputs/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 13);
        assert_eq!(part1(EXAMPLE2), 88);
        assert_eq!(part1(INPUT), 5902);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 1);
        assert_eq!(part2(EXAMPLE2), 36);
        assert_eq!(part2(INPUT), 2445);
    }
}
