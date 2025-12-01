fn part1(input: &str) -> i32 {
    let mut pos = 50;
    let mut count = 0;
    for line in input.lines() {
        let number: i32 = line[1..].parse().unwrap();
        let dir = line.chars().next().unwrap();
        pos += match dir {
            'L' => -number,
            'R' => number,
            _ => unreachable!(),
        };
        pos = pos.rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut pos = 50;
    let mut count = 0;
    for line in input.lines() {
        let number: i32 = line[1..].parse().unwrap();
        let dir = line.chars().next().unwrap();
        let number = match dir {
            'L' => -number,
            'R' => number,
            _ => unreachable!(),
        };
        // Left
        // == 0, > 0  pos / 100
        // != 0, > 0  pos / 100
        // != 0, == 0 1 + pos / 100
        // == 0, < 0  -pos / 100
        // != 0, < 0  1 - pos / 100
        // Right
        // == 0, > 0  pos / 100
        // != 0, > 0  pos / 100
        if pos != 0 && pos + number <= 0 {
            count += 1;
        }
        pos += number;
        count += (pos / 100).abs();
        pos = pos.rem_euclid(100).abs();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day1.txt");
    const INPUT: &str = include_str!("../inputs/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
        assert_eq!(part1(INPUT), 1026);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
        assert_eq!(part2(INPUT), 5923);
    }
}
