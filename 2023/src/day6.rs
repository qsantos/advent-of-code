fn parse_line1<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Vec<u64> {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
}

fn race_winning_possibilities(time: u64, distance: u64) -> u64 {
    // d = t * (time - t)
    // t * t - t * time + d = 0
    // Δ = time * time - 4 * d
    // t = (time ± √(time * time - 4 * d)) / 2
    let delta = time * time - 4 * distance;
    let sq_delta = (delta as f64).sqrt();
    let t1 = ((time as f64 - sq_delta) / 2.).floor() as u64 + 1;
    let t2 = ((time as f64 + sq_delta) / 2.).ceil() as u64 - 1;
    t2 - t1 + 1
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_line1(&mut lines);
    let distances = parse_line1(&mut lines);
    let mut ret = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        ret *= race_winning_possibilities(time, distance);
    }
    ret
}

fn parse_line2<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> u64 {
    lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_line2(&mut lines);
    let distance = parse_line2(&mut lines);
    race_winning_possibilities(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day6.txt");
    const INPUT: &str = include_str!("../inputs/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 288);
        assert_eq!(part1(INPUT), 3316275);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 71503);
        assert_eq!(part2(INPUT), 27102791);
    }
}
