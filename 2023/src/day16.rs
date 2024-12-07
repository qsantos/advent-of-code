use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    i: isize,
    j: isize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
}

impl Direction {
    fn next_position(&self, position: &Position) -> Position {
        let &Position { i, j } = position;
        match self {
            Direction::Left => Position { i, j: j - 1 },
            Direction::Up => Position { i: i - 1, j },
            Direction::Right => Position { i, j: j + 1 },
            Direction::Down => Position { i: i + 1, j },
        }
    }
}

fn count_energized(grid: &[&[u8]], start: State) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut q = Vec::new();
    let mut visited_states = HashSet::new();
    let mut visited_tiles = HashSet::new();
    q.push(start);
    while let Some(state) = q.pop() {
        if !visited_states.insert(state.clone()) {
            continue;
        }
        let State {
            position,
            direction,
        } = state;
        visited_tiles.insert(position.clone());
        let position = direction.next_position(&position);
        let i = position.i;
        let j = position.j;
        if !((0..rows).contains(&i) && (0..cols).contains(&j)) {
            continue;
        }
        let c = grid[i as usize][j as usize];
        match (c, direction) {
            (b'.', _)
            | (b'-', Direction::Left | Direction::Right)
            | (b'|', Direction::Up | Direction::Down) => q.push(State {
                position,
                direction,
            }),
            (b'/', Direction::Left) => q.push(State {
                position,
                direction: Direction::Down,
            }),
            (b'/', Direction::Up) => q.push(State {
                position,
                direction: Direction::Right,
            }),
            (b'/', Direction::Right) => q.push(State {
                position,
                direction: Direction::Up,
            }),
            (b'/', Direction::Down) => q.push(State {
                position,
                direction: Direction::Left,
            }),
            (b'\\', Direction::Left) => q.push(State {
                position,
                direction: Direction::Up,
            }),
            (b'\\', Direction::Up) => q.push(State {
                position,
                direction: Direction::Left,
            }),
            (b'\\', Direction::Right) => q.push(State {
                position,
                direction: Direction::Down,
            }),
            (b'\\', Direction::Down) => q.push(State {
                position,
                direction: Direction::Right,
            }),
            (b'-', Direction::Up | Direction::Down) => {
                q.push(State {
                    position: position.clone(),
                    direction: Direction::Left,
                });
                q.push(State {
                    position,
                    direction: Direction::Right,
                });
            }
            (b'|', Direction::Left | Direction::Right) => {
                q.push(State {
                    position: position.clone(),
                    direction: Direction::Up,
                });
                q.push(State {
                    position,
                    direction: Direction::Down,
                });
            }
            (b, d) => panic!("unexpected cell {} and direction {d:?}", b as char),
        }
    }
    visited_tiles.len() - 1
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|b| *b == b'\n').collect();
    count_energized(&grid, State {
        position: Position { i: 0, j: -1 },
        direction: Direction::Right,
    })
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.trim().as_bytes().split(|b| *b == b'\n').collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut v = Vec::new();
    for i in 0..rows {
        v.push(count_energized(&grid, State {
            position: Position { i, j: -1 },
            direction: Direction::Right,
        }));
        v.push(count_energized(&grid, State {
            position: Position { i, j: cols },
            direction: Direction::Left,
        }));
    }
    for j in 0..cols {
        v.push(count_energized(&grid, State {
            position: Position { i: -1, j },
            direction: Direction::Down,
        }));
        v.push(count_energized(&grid, State {
            position: Position { i: rows, j },
            direction: Direction::Up,
        }));
    }
    v.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day16.txt");
    const INPUT: &str = include_str!("../inputs/day16.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 46);
        assert_eq!(part1(INPUT), 7632);

        assert_eq!(part2(EXAMPLE), 51);
        assert_eq!(part2(INPUT), 8023);
    }
}
