use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => '<',
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
            }
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    i: usize,
    j: usize,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn next_position(&self, rows: usize, cols: usize, position: &Position) -> Option<Position> {
        let &Position { i, j } = position;
        match self {
            Direction::Left => (j > 0).then(|| Position { i, j: j - 1 }),
            Direction::Up => (i > 0).then(|| Position { i: i - 1, j }),
            Direction::Right => (j < cols - 1).then(|| Position { i, j: j + 1 }),
            Direction::Down => (i < rows - 1).then(|| Position { i: i + 1, j }),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
    straight_steps: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct HeatLossAndState {
    heat_loss: usize,
    state: State,
}

impl HeatLossAndState {
    fn key(&self) -> usize {
        usize::MAX - self.heat_loss
        // NOTE: this assumes that the grid is square
        // NOTE: this assumes that the grid contains no zeroes
        // usize::MAX / 2 - self.heat_loss + self.state.position.i + self.state.position.j
    }
}

impl PartialOrd for HeatLossAndState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key().partial_cmp(&other.key())
    }
}

impl Ord for HeatLossAndState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

fn show_path(grid: &[&[u8]], previous: &HashMap<State, State>, state: &State) {
    let mut visited = HashMap::new();
    let mut state = state;
    visited.insert(state.position.clone(), state.direction);
    while let Some(prev) = previous.get(state) {
        state = prev;
        visited.insert(state.position.clone(), state.direction);
    }
    for (i, row) in grid.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if let Some(direction) = visited.get(&Position { i, j }) {
                print!("{direction:?}");
            } else {
                print!("{}", *c as char);
            }
        }
        println!();
    }
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.trim().as_bytes().split(|b| *b == b'\n').collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut q = BinaryHeap::new();
    for direction in [
        Direction::Left,
        Direction::Up,
        Direction::Right,
        Direction::Down,
    ] {
        q.push(HeatLossAndState {
            heat_loss: 0,
            state: State {
                position: Position { i: 0, j: 0 },
                direction,
                straight_steps: 1,
            },
        });
    }
    let end = Position {
        i: rows - 1,
        j: cols - 1,
    };
    let mut previous: HashMap<State, State> = HashMap::new();
    while let Some(HeatLossAndState { heat_loss, state }) = q.pop() {
        let State {
            position,
            direction,
            straight_steps,
        } = state.clone();
        if position == end {
            // show_path(&grid, &previous, &state);
            return heat_loss;
        }
        for (direction, straight_steps) in [
            (direction, straight_steps + 1),
            (direction.turn_left(), 1),
            (direction.turn_right(), 1),
        ] {
            if straight_steps > 3 {
                continue;
            }
            if let Some(position) = direction.next_position(rows, cols, &position) {
                let heat_loss = heat_loss + (grid[position.i][position.j] - b'0') as usize;
                let next_state = State {
                    position,
                    direction,
                    straight_steps,
                };
                if let Entry::Vacant(e) = previous.entry(next_state.clone()) {
                    e.insert(state.clone());
                    q.push(HeatLossAndState {
                        heat_loss,
                        state: next_state,
                    });
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(part1("example"), 102);
    assert_eq!(part1("input"), 1128);
}
