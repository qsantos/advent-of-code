use std::fmt::Display;

pub fn quadrans_after_steps(input: &str, steps: usize, width: i64, height: i64) -> usize {
    assert!(width > 0);
    assert!(height > 0);
    let steps = steps as i64;
    let mut quadrants = vec![0, 0, 0, 0];
    for line in input.lines() {
        let (pos, vel) = line.split_once(' ').unwrap();
        let pos = pos.strip_prefix("p=").unwrap();
        let vel = vel.strip_prefix("v=").unwrap();
        let (px, py) = pos.split_once(',').unwrap();
        let (vx, vy) = vel.split_once(',').unwrap();
        let px: i64 = px.parse().unwrap();
        let py: i64 = py.parse().unwrap();
        let vx: i64 = vx.parse().unwrap();
        let vy: i64 = vy.parse().unwrap();
        let x = (px + steps * vx).rem_euclid(width);
        let y = (py + steps * vy).rem_euclid(height);
        let mid_x = width / 2;
        let mid_y = height / 2;
        match (x.cmp(&mid_x), y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[1] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[2] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
            _ => (),
        }
    }
    quadrants.into_iter().product()
}

pub fn part1_example(input: &str) -> impl Display {
    quadrans_after_steps(input, 100, 11, 7)
}

pub fn part1(input: &str) -> impl Display {
    quadrans_after_steps(input, 100, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day14.txt");
    const INPUT: &str = include_str!("../inputs/day14.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1_example(EXAMPLE).to_string(), "12");
        assert_eq!(part1(INPUT).to_string(), "231782040");
    }
}
