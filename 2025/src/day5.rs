use std::ops::RangeInclusive;

struct Problem {
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
}

impl Problem {
    fn from(input: &str) -> Problem {
        let (fresh_id_ranges, available) = input.trim().split_once("\n\n").unwrap();
        let fresh_id_ranges = fresh_id_ranges.lines().map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            start..=end
        }).collect();
        let available_ids = available.lines().map(|line| line.parse().unwrap()).collect();
        Problem {
            fresh_id_ranges,
            available_ids,
        }
    }
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from(input);
    let mut count = 0;
    for id in problem.available_ids {
        let is_fresh = problem.fresh_id_ranges.iter().any(|range| range.contains(&id));
        if is_fresh {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> u64 {
    let Problem { fresh_id_ranges, .. } = Problem::from(input);
    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    for range in fresh_id_ranges {
        let (mut start, mut end) = (range.start(), range.end());
        // merged_ranges [ ]  [ ]  [ ]  [ ]
        // range            [       ]
        // we need to find all the ranges that overlap with the current range
        // we replace them with the current range
        // if the first range extends before, we extend the current range left accordingly
        // similarly if the last range extends after
        //
        // find first overlapping range
        let first_index = if let Some((first_index, first_range)) = merged_ranges.iter().enumerate().find(|(_, other)| other.end() >= start) {
            // extend left if needed
            start = first_range.start().min(start);
            first_index
        } else {
            merged_ranges.len()
        };
        // find last overlapping range
        let last_index = if let Some((last_index, last_range)) = merged_ranges.iter().enumerate().rev().find(|(_, other)| other.start() <= end) {
            // extend right if needed
            end = last_range.end().max(end);
            last_index
        } else {
            0
        };
        let range = *start..=*end;
        // replace overlapping ranges
        if first_index == merged_ranges.len() {
            merged_ranges.push(range);
        } else {
            merged_ranges.splice(first_index..=last_index, [range]);
        }
    }
    merged_ranges.into_iter().map(|range| range.end() - range.start() + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day5.txt");
    const INPUT: &str = include_str!("../inputs/day5.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
        assert_eq!(part1(INPUT), 640);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
        assert_eq!(part2(INPUT), 365804144481581);
    }
}
