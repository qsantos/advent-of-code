use std::fmt::Display;

fn find_unsafe_pair_decreasing(levels: &[u32]) -> Option<usize> {
    (1..levels.len()).find(|&i| levels[i] >= levels[i - 1] || levels[i - 1] - levels[i] > 3)
}

fn is_safe_decreasing(levels: &[u32]) -> bool {
    find_unsafe_pair_decreasing(levels).is_none()
}

fn is_safe_decreasing_tolerance(levels: &mut Vec<u32>) -> bool {
    let Some(i) = find_unsafe_pair_decreasing(levels) else {
        return true;
    };
    let mut original = levels.remove(i - 1);
    let safe = is_safe_decreasing(levels);
    if safe {
        levels.insert(i - 1, original);
        return true;
    }
    // combine the two operations below in one to avoid moving all elements
    //levels.insert(i - 1, original);
    //let original = levels.remove(i);
    (levels[i - 1], original) = (original, levels[i - 1]);
    let safe = is_safe_decreasing(levels);
    levels.insert(i, original);
    safe
}

fn find_unsafe_pair_increasing(levels: &[u32]) -> Option<usize> {
    (1..levels.len()).find(|&i| levels[i] <= levels[i - 1] || levels[i] - levels[i - 1] > 3)
}

fn is_safe_increasing(levels: &[u32]) -> bool {
    find_unsafe_pair_increasing(levels).is_none()
}

fn is_safe_increasing_tolerance(levels: &mut Vec<u32>) -> bool {
    let Some(i) = find_unsafe_pair_increasing(levels) else {
        return true;
    };
    let mut original = levels.remove(i - 1);
    let safe = is_safe_increasing(levels);
    if safe {
        levels.insert(i - 1, original);
        return true;
    }
    // combine the two operations below in one to avoid moving all elements
    (levels[i - 1], original) = (original, levels[i - 1]);
    //levels.insert(i - 1, original);
    //let original = levels.remove(i);
    let safe = is_safe_increasing(levels);
    levels.insert(i, original);
    safe
}

fn is_safe(levels: &[u32]) -> bool {
    is_safe_decreasing(levels) || is_safe_increasing(levels)
}

fn is_safe_tolerance(levels: &mut Vec<u32>) -> bool {
    is_safe_decreasing_tolerance(levels) || is_safe_increasing_tolerance(levels)
}

pub fn part1(input: &str) -> impl Display {
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<u32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        if is_safe(&levels) {
            safe_reports += 1;
        }
    }
    safe_reports
}

pub fn part2(input: &str) -> impl Display {
    let mut safe_reports = 0;
    for line in input.lines() {
        let mut levels: Vec<u32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        if is_safe_tolerance(&mut levels) {
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "4");
        assert_eq!(part2(INPUT).to_string(), "540");
    }
}
