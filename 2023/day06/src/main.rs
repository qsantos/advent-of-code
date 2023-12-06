fn parse_line<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Vec<u64> {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(filename: &str) -> u64 {
    let text = std::fs::read_to_string(filename).unwrap();
    let mut lines = text.lines();
    let times = parse_line(&mut lines);
    let distances = parse_line(&mut lines);
    let mut ret = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
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

        ret *= t2 - t1 + 1;
    }
    ret
}

fn main() {
    assert_eq!(part1("example"), 288);
    assert_eq!(part1("input"), 3316275);
}
