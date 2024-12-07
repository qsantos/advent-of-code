use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
];

struct Grid<'a> {
    rows: usize,
    cols: usize,
    cells: Vec<&'a [u8]>,
}

impl<'a> Grid<'a> {
    fn from(data: &'a [u8]) -> Self {
        let cells: Vec<&[u8]> = data.split(|b| *b == b'\n').collect();
        let rows = cells.len();
        let cols = cells[0].len();
        assert!(cells.iter().all(|row| row.len() == cols));
        Grid { cells, rows, cols }
    }

    fn at(&self, (i, j): Position) -> u8 {
        self.cells[i][j]
    }

    fn find_start(&self) -> Position {
        for (i, row) in self.cells.iter().enumerate() {
            if let Some(j) = row.iter().position(|b| *b == b'S') {
                return (i, j);
            }
        }
        unreachable!();
    }

    fn can_connect_to(&self, pos: Position, dir: Direction) -> bool {
        if let Some(n) = self.peek(pos, dir) {
            match dir {
                Direction::Left => n == b'-' || n == b'F' || n == b'L',
                Direction::Up => n == b'|' || n == b'F' || n == b'7',
                Direction::Right => n == b'-' || n == b'7' || n == b'J',
                Direction::Down => n == b'|' || n == b'L' || n == b'J',
            }
        } else {
            false
        }
    }

    fn start_dir(&self, start: Position) -> Direction {
        for dir in DIRECTIONS {
            if self.can_connect_to(start, dir) {
                return dir;
            }
        }
        panic!("start is not connected to any pipes");
    }

    fn start_equivalent(&self, start: Position) -> u8 {
        match DIRECTIONS.map(|dir| self.can_connect_to(start, dir)) {
            [false, false, true, true] => b'F',
            [true, false, false, true] => b'7',
            [true, true, false, false] => b'J',
            [false, true, true, false] => b'L',
            _ => panic!("impossible situation for start"),
        }
    }

    fn neighbor(&self, (i, j): Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Left => (j > 0).then(|| (i, j - 1)),
            Direction::Up => (i > 0).then(|| (i - 1, j)),
            Direction::Right => (j < self.cols - 1).then(|| (i, j + 1)),
            Direction::Down => (i < self.rows - 1).then(|| (i + 1, j)),
        }
    }

    fn peek(&self, pos: Position, dir: Direction) -> Option<u8> {
        self.neighbor(pos, dir).map(|n| self.at(n))
    }

    fn next_dir(&self, dir: Direction, pos: Position) -> Direction {
        let c = self.at(pos);
        match (dir, c) {
            (Direction::Left, b'-') => Direction::Left,
            (Direction::Left, b'F') => Direction::Down,
            (Direction::Left, b'L') => Direction::Up,
            (Direction::Up, b'|') => Direction::Up,
            (Direction::Up, b'F') => Direction::Right,
            (Direction::Up, b'7') => Direction::Left,
            (Direction::Right, b'-') => Direction::Right,
            (Direction::Right, b'7') => Direction::Down,
            (Direction::Right, b'J') => Direction::Up,
            (Direction::Down, b'|') => Direction::Down,
            (Direction::Down, b'L') => Direction::Right,
            (Direction::Down, b'J') => Direction::Left,
            (_, b'S') => dir,
            _ => panic!(
                "unexpected incoming direction {dir:?} on {:?}",
                c.escape_ascii().to_string()
            ),
        }
    }

    fn follow_pipe(&self, pos: Position, dir: Direction) -> (Position, Direction) {
        let pos = self.neighbor(pos, dir).unwrap();
        let dir = self.next_dir(dir, pos);
        (pos, dir)
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from(input.trim().as_bytes());
    let start = grid.find_start();
    let mut pos = start;
    let mut dir = grid.start_dir(pos);
    let mut steps = 0;
    loop {
        (pos, dir) = grid.follow_pipe(pos, dir);
        steps += 1;
        if pos == start {
            break;
        }
    }
    (steps + 1) / 2
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from(input.trim().as_bytes());
    let start = grid.find_start();
    let mut pos = start;
    let mut dir = grid.start_dir(pos);
    let mut r#loop = HashSet::new();
    loop {
        (pos, dir) = grid.follow_pipe(pos, dir);
        r#loop.insert(pos);
        if pos == start {
            break;
        }
    }

    let mut count = 0;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if r#loop.contains(&(i, j)) {
                continue;
            }
            let mut n = 0;
            for oi in 0..i {
                if !r#loop.contains(&(oi, j)) {
                    continue;
                }
                let mut c = grid.at((oi, j));
                if c == b'S' {
                    c = grid.start_equivalent((oi, j));
                }
                if c == b'-' {
                    n += 2;
                } else if c == b'F' || c == b'J' {
                    n += 1;
                } else if c == b'7' || c == b'L' {
                    n -= 1;
                }
            }
            assert_eq!(n % 2, 0);
            if (n / 2) % 2 != 0 {
                // inside
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day10-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day10-2.txt");
    const EXAMPLE3: &str = include_str!("../examples/day10-3.txt");
    const EXAMPLE4: &str = include_str!("../examples/day10-4.txt");
    const EXAMPLE5: &str = include_str!("../examples/day10-5.txt");
    const EXAMPLE6: &str = include_str!("../examples/day10-6.txt");
    const EXAMPLE7: &str = include_str!("../examples/day10-7.txt");
    const INPUT: &str = include_str!("../inputs/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 4);
        assert_eq!(part1(EXAMPLE2), 4);
        assert_eq!(part1(EXAMPLE3), 8);
        assert_eq!(part1(EXAMPLE4), 8);
        assert_eq!(part1(INPUT), 7102);

        assert_eq!(part2(EXAMPLE1), 1);
        assert_eq!(part2(EXAMPLE2), 1);
        assert_eq!(part2(EXAMPLE3), 1);
        assert_eq!(part2(EXAMPLE4), 1);
        assert_eq!(part2(EXAMPLE5), 4);
        assert_eq!(part2(EXAMPLE6), 8);
        assert_eq!(part2(EXAMPLE7), 10);
        assert_eq!(part2(INPUT), 363);
    }
}
