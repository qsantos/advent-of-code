use std::collections::HashSet;
use std::fmt::Display;

pub fn regions(input: &str) -> Vec<HashSet<(i64, i64)>> {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|&b| b == b'\n').collect();
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut regions = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            if visited.contains(&(i, j)) {
                continue;
            }
            let c = grid[i as usize][j as usize];
            let mut q = vec![(i, j)];
            let mut region = HashSet::new();
            while let Some((i, j)) = q.pop() {
                if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                    continue;
                }
                if grid[i as usize][j as usize] != c {
                    continue;
                }
                if !region.insert((i, j)) {
                    continue;
                }
                q.push((i - 1, j));
                q.push((i + 1, j));
                q.push((i, j - 1));
                q.push((i, j + 1));
            }
            visited.extend(&region);
            regions.push(region);
        }
    }
    regions
}

pub fn part1(input: &str) -> impl Display {
    let regions = regions(input);
    let mut total = 0;
    for region in regions {
        let mut perimeter = 0;
        for &(i, j) in &region {
            // above
            if !region.contains(&(i - 1, j)) {
                perimeter += 1;
            }
            // below
            if !region.contains(&(i + 1, j)) {
                perimeter += 1;
            }
            // left
            if !region.contains(&(i, j - 1)) {
                perimeter += 1;
            }
            // right
            if !region.contains(&(i, j + 1)) {
                perimeter += 1;
            }
        }
        let area = region.len();
        total += perimeter * area;
    }
    total
}

pub fn part2(input: &str) -> impl Display {
    let regions = regions(input);
    let mut total = 0;
    for region in regions {
        let mut perimeter = 0;
        // Clippy "improvements" change nothing here
        #[allow(clippy::nonminimal_bool)]
        for &(i, j) in &region {
            // above
            if !(region.contains(&(i, j - 1)) && !region.contains(&(i - 1, j - 1)))
                && !region.contains(&(i - 1, j))
            {
                perimeter += 1;
            }
            // below
            if !(region.contains(&(i, j - 1)) && !region.contains(&(i + 1, j - 1)))
                && !region.contains(&(i + 1, j))
            {
                perimeter += 1;
            }
            // left
            if !(region.contains(&(i - 1, j)) && !region.contains(&(i - 1, j - 1)))
                && !region.contains(&(i, j - 1))
            {
                perimeter += 1;
            }
            // right
            if !(region.contains(&(i - 1, j)) && !region.contains(&(i - 1, j + 1)))
                && !region.contains(&(i, j + 1))
            {
                perimeter += 1;
            }
        }
        let area = region.len();
        total += perimeter * area;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day12-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day12-2.txt");
    const EXAMPLE3: &str = include_str!("../examples/day12-3.txt");
    const EXAMPLE4: &str = include_str!("../examples/day12-4.txt");
    const EXAMPLE5: &str = include_str!("../examples/day12-5.txt");
    const INPUT: &str = include_str!("../inputs/day12.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "140");
        assert_eq!(part1(EXAMPLE2).to_string(), "772");
        assert_eq!(part1(EXAMPLE3).to_string(), "1930");
        assert_eq!(part1(INPUT).to_string(), "1424472");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1).to_string(), "80");
        assert_eq!(part2(EXAMPLE2).to_string(), "436");
        assert_eq!(part2(EXAMPLE3).to_string(), "1206");
        assert_eq!(part2(EXAMPLE4).to_string(), "236");
        assert_eq!(part2(EXAMPLE5).to_string(), "368");
        assert_eq!(part2(INPUT).to_string(), "870202");
    }
}
