use std::fmt::Display;

fn nth_secret(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
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
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day22.txt");
    const INPUT: &str = include_str!("../inputs/day22.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "37327623");
        assert_eq!(part1(INPUT).to_string(), "13004408787");
    }
}
