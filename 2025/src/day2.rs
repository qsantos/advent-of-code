fn part1(input: &str) -> u64 {
    let mut invalid = 0;
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for number in start..=end {
            let x = format!("{number}");
            let l = x.len();
            if l % 2 == 0 && x[..l/2] == x[l/2..] {
                invalid += number;
            }
        }
    }
    invalid
}

fn part2(input: &str) -> u64 {
    let mut invalid = 0;
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for number in start..=end {
            let x = format!("{number}");
            let l = x.len();
            'outer:
            for stride in 1..l {
                if l % stride != 0 {
                    continue;
                }
                for offset in (stride..=l - stride).step_by(stride) {
                    if x[offset..offset + stride] != x[..stride] {
                        continue 'outer;
                    }
                }
                invalid += number;
                break 'outer;
            }
        }
    }
    invalid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day2.txt");
    const INPUT: &str = include_str!("../inputs/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
        assert_eq!(part1(INPUT), 35367539282);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
        assert_eq!(part2(INPUT), 45814076230);
    }
}
