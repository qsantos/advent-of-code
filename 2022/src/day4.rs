struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        assert_eq!(parts.len(), 2);
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

fn count_relations<T>(input: &str, relation: T) -> i32
where
    T: Fn(&Range, &Range) -> bool,
{
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            assert_eq!(parts.len(), 2);
            let first = Range::from_str(parts[0]);
            let second = Range::from_str(parts[1]);
            (relation(&first, &second) || relation(&second, &first)) as i32
        })
        .sum()
}

pub fn part1(input: &str) -> i32 {
    count_relations(input, Range::contains)
}

pub fn part2(input: &str) -> i32 {
    count_relations(input, Range::overlaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day4.txt");
    const INPUT: &str = include_str!("../inputs/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
        assert_eq!(part1(INPUT), 540);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4);
        assert_eq!(part2(INPUT), 872);
    }
}
