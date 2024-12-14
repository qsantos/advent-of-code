use std::collections::HashSet;
use std::fmt::Display;

pub fn parse(input: &str) -> Vec<(i64, i64, i64, i64)> {
    let mut posvel = Vec::new();
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
        posvel.push((px, py, vx, vy));
    }
    posvel
}

pub fn step(
    posvel: &Vec<(i64, i64, i64, i64)>,
    steps: usize,
    width: i64,
    height: i64,
) -> Vec<(i64, i64)> {
    assert!(width > 0);
    assert!(height > 0);
    let steps = steps as i64;
    let mut ret = Vec::new();
    for (px, py, vx, vy) in posvel {
        let x = (px + steps * vx).rem_euclid(width);
        let y = (py + steps * vy).rem_euclid(height);
        ret.push((x, y));
    }
    ret
}

pub fn quadrants(pos: &Vec<(i64, i64)>, width: i64, height: i64) -> Vec<usize> {
    assert!(width > 0);
    assert!(height > 0);
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quadrants = vec![0, 0, 0, 0];
    for (x, y) in pos {
        match (x.cmp(&mid_x), y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[1] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[2] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
            _ => (),
        }
    }
    quadrants
}

pub fn quadrants_after_steps(input: &str, steps: usize, width: i64, height: i64) -> usize {
    let posvel = parse(input);
    let pos = step(&posvel, steps, width, height);
    quadrants(&pos, width, height).into_iter().product()
}

fn has_bar(robots: &mut Vec<(i64, i64)>, threshold: usize) -> bool {
    robots.sort();
    let mut px = i64::MIN;
    let mut py = i64::MIN;
    let mut length = 0;
    for &mut (x, y) in robots {
        if (x, y) == (px, py + 1) {
            length += 1;
            if length >= threshold {
                return true;
            }
        } else {
            length = 0;
        }
        px = x;
        py = y;
    }
    false
}

fn print(width: i64, height: i64, robots: &HashSet<(i64, i64)>) {
    for y in 0..height {
        for x in 0..width {
            if robots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part1_example(input: &str) -> impl Display {
    quadrants_after_steps(input, 100, 11, 7)
}

pub fn part1(input: &str) -> impl Display {
    quadrants_after_steps(input, 100, 101, 103)
}

pub fn part2(input: &str) -> impl Display {
    let posvel = parse(input);
    let width = 101;
    let height = 103;
    let mut steps = 0;
    loop {
        let mut pos = step(&posvel, steps, width, height);
        if has_bar(&mut pos, 10) {
            let robots: HashSet<_> = pos.into_iter().collect();
            print(width, height, &robots);
            return steps;
        }
        steps += 1;
    }
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).to_string(), "6475");
    }
}
