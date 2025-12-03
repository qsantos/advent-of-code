fn solve(input: &str, digits: usize) -> u64 {
    let mut ret = 0u64;
    for line in input.lines() {
        let line = line.as_bytes();
        let mut pos = 0usize;
        let mut value = 0;
        for digit in 0..digits {
            let mut max_pos = pos;
            let mut max_val = line[pos];
            #[allow(clippy::needless_range_loop)]
            for i in pos..line.len() + digit + 1 - digits {
                let c = line[i];
                if c > max_val {
                    max_pos = i;
                    max_val = c;
                }
            }
            pos = max_pos + 1;
            value = value * 10 + (max_val - b'0') as u64;
        }
        ret += value;
    }
    ret
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 12)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day3.txt");
    const INPUT: &str = include_str!("../inputs/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
        assert_eq!(part1(INPUT), 17346);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
        assert_eq!(part2(INPUT), 172981362045136);
    }
}
