use std::fmt::Display;

fn next_secret(mut secret: i64) -> i64 {
    secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}

fn nth_secret(mut secret: i64, n: usize) -> i64 {
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

#[test]
fn test_nth_secret() {
    assert_eq!(nth_secret(123, 1), 15887950);
    assert_eq!(nth_secret(123, 2), 16495136);
    assert_eq!(nth_secret(123, 3), 527345);
    assert_eq!(nth_secret(123, 4), 704524);
    assert_eq!(nth_secret(123, 5), 1553684);
    assert_eq!(nth_secret(123, 6), 12683156);
    assert_eq!(nth_secret(123, 7), 11100544);
    assert_eq!(nth_secret(123, 8), 12249484);
    assert_eq!(nth_secret(123, 9), 7753432);
    assert_eq!(nth_secret(123, 10), 5908254);
    assert_eq!(nth_secret(1, 2000), 8685429);
    assert_eq!(nth_secret(10, 2000), 4700978);
    assert_eq!(nth_secret(100, 2000), 15273692);
    assert_eq!(nth_secret(2024, 2000), 8667524);
}

pub fn part1(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| nth_secret(line.parse().unwrap(), 2000))
        .sum::<i64>()
}

pub fn part2(input: &str) -> impl Display {
    let mut bananas_per_changes = vec![0i64; 20 * 20 * 20 * 20];
    for line in input.lines() {
        let mut secret: i64 = line.parse().unwrap();
        let new_secret = next_secret(secret);
        let mut d1 = new_secret % 10 - secret % 10;
        secret = new_secret;
        let new_secret = next_secret(secret);
        let mut d2 = new_secret % 10 - secret % 10;
        secret = new_secret;
        let new_secret = next_secret(secret);
        let mut d3 = new_secret % 10 - secret % 10;
        secret = new_secret;
        let mut seen = vec![false; 20 * 20 * 20 * 20];
        for _ in 0..1997 {
            let new_secret = next_secret(secret);
            let d4 = new_secret % 10 - secret % 10;
            secret = new_secret;
            let i = (((d1 + 10) * 20 + (d2 + 10)) * 20 + (d3 + 10)) * 20 + (d4 + 10);
            let i = i as usize;
            if !seen[i] {
                seen[i] = true;
                bananas_per_changes[i] += secret % 10;
            }
            (d1, d2, d3) = (d2, d3, d4);
        }
    }
    bananas_per_changes.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day22-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day22-2.txt");
    const INPUT: &str = include_str!("../inputs/day22.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "37327623");
        assert_eq!(part1(INPUT).to_string(), "13004408787");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2).to_string(), "23");
        assert_eq!(part2(INPUT).to_string(), "1455");
    }
}
