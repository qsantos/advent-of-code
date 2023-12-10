type Position = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

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

    fn start_dir(&self, start: Position) -> Direction {
        if let Some(n) = self.peek(start, Direction::Left) {
            if n == b'-' || n == b'F' || n == b'L' {
                return Direction::Left;
            }
        }
        if let Some(n) = self.peek(start, Direction::Up) {
            if n == b'|' || n == b'F' || n == b'7' {
                return Direction::Up;
            }
        }
        if let Some(n) = self.peek(start, Direction::Right) {
            if n == b'-' || n == b'7' || n == b'J' {
                return Direction::Right;
            }
        }
        if let Some(n) = self.peek(start, Direction::Down) {
            if n == b'|' || n == b'L' || n == b'J' {
                return Direction::Down;
            }
        }
        panic!("start is not connected to any pipes");
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

    fn follow_pipe(&self, dir: Direction, pos: Position) -> Direction {
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
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid = Grid::from(data.trim().as_bytes());
    let start = grid.find_start();
    let mut pos = start;
    let mut dir = grid.start_dir(pos);
    let mut steps = 0;
    loop {
        pos = grid.neighbor(pos, dir).unwrap();
        dir = grid.follow_pipe(dir, pos);
        steps += 1;
        if pos == start {
            break;
        }
    }
    (steps + 1) / 2
}

fn main() {
    assert_eq!(part1("example1"), 4);
    assert_eq!(part1("example2"), 4);
    assert_eq!(part1("example3"), 8);
    assert_eq!(part1("example4"), 8);
    assert_eq!(part1("input"), 7102);
}
