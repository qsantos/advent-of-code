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

    let t1 = ((time as f64 - sq_delta) / 2.).ceil() as u64;
    let t1 = if t1 * (time - t1) == distance { t1 + 1 } else { t1 };
    assert!((t1 - 1) * (time - (t1 - 1)) <= distance);
    assert!(t1 * (time - t1) > distance);

    let t2 = ((time as f64 + sq_delta) / 2.).floor() as u64;
    let t2 = if t2 * (time - t2) == distance { t2 - 1 } else { t2 };
    assert!(t2 * (time - t2) > distance);
    assert!((t2 + 1) * (time - (t2 + 1)) <= distance);

    t2 - t1 + 1
}

fn part1(filename: &str) -> u64 {
    let text = std::fs::read_to_string(filename).unwrap();
    let mut lines = text.lines();
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

fn part2(filename: &str) -> u64 {
    let text = std::fs::read_to_string(filename).unwrap();
    let mut lines = text.lines();
    let time = parse_line2(&mut lines);
    let distance = parse_line2(&mut lines);
    race_winning_possibilities(time, distance)
}

fn main() {
    assert_eq!(part1("example"), 288);
    assert_eq!(part1("input"), 3316275);

    assert_eq!(part2("example"), 71503);
    assert_eq!(part2("input"), 27102791);
}
