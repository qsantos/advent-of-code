fn arrangements(springs: &[u8], groups: &[i32]) -> usize {
    // c[i][j] is the number of arrangements when looking at the first i springs and j groups
    let mut c = Vec::new();

    {
        let mut first_row = Vec::new();
        first_row.push(1);
        first_row.resize(groups.len() + 1, 0);
        c.push(first_row);
    }

    for i in 1..=springs.len() {
        let spring = springs[i - 1];
        let mut row = Vec::new();
        for j in 0..=groups.len() {
            let mut count = 0;
            // consume no group
            if spring == b'.' || spring == b'?' {
                count += c[i - 1][j];
            }
            // consume a single group
            if j > 0 {
                let group_size = groups[j - 1] as usize;
                // group elements
                if i >= group_size
                    && springs[(i - 1) - (group_size - 1)..=(i - 1)]
                        .iter()
                        .all(|s| *s == b'#' || *s == b'?')
                {
                    if i == group_size {
                        count += c[0][j - 1];
                    } else {
                        // group separator
                        let s = springs[(i - 1) - (group_size - 1) - 1];
                        if s == b'.' || s == b'?' {
                            count += c[i - group_size - 1][j - 1];
                        }
                    }
                }
            }
            row.push(count);
        }
        c.push(row);
    }
    c[springs.len()][groups.len()]
}

pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.as_bytes();
        let groups: Vec<i32> = groups.split(',').map(|g| g.parse().unwrap()).collect();
        let c = arrangements(springs, &groups);
        sum += c;
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.as_bytes();
        let springs = [springs, springs, springs, springs, springs].join(&b'?');
        let groups: Vec<i32> = groups.split(',').map(|g| g.parse().unwrap()).collect();
        let groups = groups.repeat(5);
        let c = arrangements(&springs, &groups);
        dbg!(line, c);
        sum += c;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day12.txt");
    const INPUT: &str = include_str!("../inputs/day12.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
        assert_eq!(part1(INPUT), 6852);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 525152);
        assert_eq!(part2(INPUT), 8475948826693);
    }
}
