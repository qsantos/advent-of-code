use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<u32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let decreasing = (1..levels.len()).all(|i| levels[i] > levels[i - 1]);
        let increasing = (1..levels.len()).all(|i| levels[i] < levels[i - 1]);
        let safe = (1..levels.len()).all(|i| levels[i].abs_diff(levels[i - 1]) <= 3);
        if (decreasing || increasing) && safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day2.txt");
    const INPUT: &str = include_str!("../inputs/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "2");
        assert_eq!(part1(INPUT).to_string(), "486");
    }
}
