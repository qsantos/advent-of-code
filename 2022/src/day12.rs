use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord(i64, i64);

impl Coord {
    fn neighbors(&self) -> Vec<Self> {
        let &Coord(i, j) = self;
        vec![
            Coord(i - 1, j),
            Coord(i, j - 1),
            Coord(i, j + 1),
            Coord(i + 1, j),
        ]
    }
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn read(input: &str) -> Self {
        Grid(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn contains(&self, coord: &Coord) -> bool {
        let grid = &self.0;
        let rows = grid.len() as i64;
        let cols = grid[0].len() as i64;
        0 <= coord.0 && coord.0 < rows && 0 <= coord.1 && coord.1 < cols
    }

    fn find_char(&self, needle: char) -> Option<Coord> {
        for (i, row) in self.0.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == needle {
                    return Some(Coord(i as i64, j as i64));
                }
            }
        }
        None
    }

    fn char_at(&self, coord: &Coord) -> char {
        let &Coord(i, j) = coord;
        self.0[i as usize][j as usize]
    }

    fn height_at(&self, coord: &Coord) -> u32 {
        match self.char_at(coord) {
            'S' => 1,
            'E' => 26,
            c => 1 + (c as u32) - ('a' as u32),
        }
    }
}

fn count_steps(input: &str, target: char) -> Option<u32> {
    let grid = Grid::read(input);
    let start = grid.find_char('E').unwrap();

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, start));
    while let Some((steps, coord)) = q.pop_front() {
        if seen.contains(&coord) {
            continue;
        }
        seen.insert(coord.clone());

        if grid.char_at(&coord) == target {
            return Some(steps);
        }

        let min_next_elevation = grid.height_at(&coord) - 1;

        for next in coord.neighbors() {
            if !grid.contains(&next) || grid.height_at(&next) < min_next_elevation {
                continue;
            }
            q.push_back((steps + 1, next));
        }
    }
    None
}

pub fn part1(input: &str) -> u32 {
    count_steps(input, 'S').unwrap()
}

pub fn part2(input: &str) -> u32 {
    count_steps(input, 'a').unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day12.txt");
    const INPUT: &str = include_str!("../inputs/day12.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 31);
        assert_eq!(part1(INPUT), 408);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 29);
        assert_eq!(part2(INPUT), 399);
    }
}
