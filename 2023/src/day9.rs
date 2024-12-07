fn next_of(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.iter().skip(1).zip(numbers.iter()).map(|(a, b)| a - b).collect();
        numbers.last().unwrap() + next_of(&diffs)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        sum += next_of(&numbers);
    }
    sum
}

fn prev_of(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.iter().skip(1).zip(numbers.iter()).map(|(a, b)| a - b).collect();
        numbers[0] - prev_of(&diffs)
    }
}

pub fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        sum += prev_of(&numbers);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day9.txt");
    const INPUT: &str = include_str!("../inputs/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 114);
        assert_eq!(part1(INPUT), 1995001648);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 2);
        assert_eq!(part2(INPUT), 988);
    }
}
