use regex::Regex;

#[derive(Clone, Copy)]
enum Tile {
    None,
    Open,
    Wall,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::None,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    rows: i64,
    cols: i64,
}

impl Board {
    fn from(s: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let rows = tiles.len() as i64;
        let cols = tiles.iter().map(Vec::len).max().unwrap();
        for row in &mut tiles {
            row.resize(cols, Tile::None);
        }
        let cols = cols as i64;
        Board { tiles, rows, cols }
    }

    fn tile_at(&self, state: &State) -> Tile {
        self.tiles[state.y as usize][state.x as usize]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

impl Facing {
    fn value(&self) -> i64 {
        match self {
            Facing::Up => 3,
            Facing::Left => 2,
            Facing::Down => 1,
            Facing::Right => 0,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Facing::Up => Facing::Right,
            Facing::Left => Facing::Up,
            Facing::Down => Facing::Left,
            Facing::Right => Facing::Down,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    x: i64,
    y: i64,
    facing: Facing,
}

impl State {
    fn value(&self) -> i64 {
        let State { x, y, facing } = self;
        1000 * (y + 1) + 4 * (x + 1) + facing.value()
    }

    fn next_torus(&self, board: &Board) -> State {
        let mut next_state = self.advance().wrap_torus(board);
        while let Tile::None = board.tile_at(&next_state) {
            next_state = next_state.advance().wrap_torus(board)
        }
        next_state
    }

    fn next_cube_example(&self, _board: &Board) -> State {
        self.advance().wrap_cube_example()
    }

    fn next_cube_input(&self, _board: &Board) -> State {
        self.advance().wrap_cube_input()
    }

    fn advance(&self) -> State {
        let &State { x, y, facing } = self;
        match facing {
            Facing::Up => State {
                x,
                y: y - 1,
                facing,
            },
            Facing::Left => State {
                x: x - 1,
                y,
                facing,
            },
            Facing::Down => State {
                x,
                y: y + 1,
                facing,
            },
            Facing::Right => State {
                x: x + 1,
                y,
                facing,
            },
        }
    }

    fn wrap_torus(&self, board: &Board) -> State {
        State {
            x: self.x.rem_euclid(board.cols),
            y: self.y.rem_euclid(board.rows),
            facing: self.facing,
        }
    }

    //         0123456789012356 x
    //          /--------\
    //          |        |
    // y -1     |        A          -1 y
    //    0     |      ....          0
    //    1     |   ,-I....L-----\   1
    //    2     |   |  ....      |   2
    //    3     B   C  ....      |   3
    //    4    ............      |   4
    //    5    ............M-\   |   5
    //    6 /-J............  |   |   6
    //    7 |  ............  D   |   7
    //    8 |   E   F  ........  |   8
    //    9 |   |   |  ........N-/   9
    //   10 |   |   \-K........     10
    //   11 |   |      ........     11
    //   12 |   |        G   H      12
    //      |   |        |   |
    //      |   \--------/   |
    //      \----------------/
    //         0123456789012345 x
    fn wrap_cube_example(&self) -> State {
        let &State { x, y, facing } = self;
        match facing {
            Facing::Up => {
                if y == -1 {
                    // A
                    State {
                        x: 11 - x,
                        y: 4,
                        facing: Facing::Down,
                    }
                } else if y == 3 && (0..4).contains(&x) {
                    // B
                    State {
                        x: 11 - x,
                        y: 0,
                        facing: Facing::Down,
                    }
                } else if y == 3 && (4..8).contains(&x) {
                    // C
                    State {
                        x: 8,
                        y: x - 4,
                        facing: Facing::Right,
                    }
                } else if y == 7 && (12..16).contains(&x) {
                    // D
                    State {
                        x: 11,
                        y: 19 - x,
                        facing: Facing::Left,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Down => {
                if y == 8 && (0..4).contains(&x) {
                    // E
                    State {
                        x: 11 - x,
                        y: 11,
                        facing: Facing::Up,
                    }
                } else if y == 8 && (4..8).contains(&x) {
                    // F
                    State {
                        x: 8,
                        y: 15 - x,
                        facing: Facing::Right,
                    }
                } else if y == 12 && (8..12).contains(&x) {
                    // G
                    State {
                        x: 11 - x,
                        y: 7,
                        facing: Facing::Up,
                    }
                } else if y == 12 && (12..16).contains(&x) {
                    // H
                    State {
                        x: 0,
                        y: 19 - x,
                        facing: Facing::Right,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Left => {
                if x == 7 && (0..4).contains(&y) {
                    // I
                    State {
                        x: y + 4,
                        y: 4,
                        facing: Facing::Down,
                    }
                } else if x == -1 {
                    // J
                    State {
                        x: 19 - y,
                        y: 11,
                        facing: Facing::Up,
                    }
                } else if x == 7 && (8..12).contains(&y) {
                    // K
                    State {
                        x: 15 - y,
                        y: 7,
                        facing: Facing::Up,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Right => {
                if x == 12 && (0..4).contains(&y) {
                    // L
                    State {
                        x: 15,
                        y: 11 - y,
                        facing: Facing::Left,
                    }
                } else if x == 12 && (4..8).contains(&y) {
                    // M
                    State {
                        x: 19 - y,
                        y: 8,
                        facing: Facing::Down,
                    }
                } else if x == 16 {
                    // N
                    State {
                        x: 11,
                        y: 11 - y,
                        facing: Facing::Left,
                    }
                } else {
                    self.clone()
                }
            }
        }
    }

    //           0123456789012356 x (in tens)
    //      /-----------\    /-------\
    //      |           |    |       |
    // y -1 |           A    B       | -1 y
    // (  0 |         ..........     |  0 (
    // i  1 |         ..........     |  1 i
    // n  2 | /------G..........K-\  |  2 n
    //    3 | |       ..........  |  |  3
    // t  4 | |       ..........  |  |  4 t
    // e  5 | |       .....  D    |  |  5 e
    // n  6 | |       .....  |    |  |  6 n
    // s  7 | |    /-H.....L-/    |  |  7 s
    // )  8 | |    |  .....       |  |  8 )
    //    9 | |    C  .....       |  |  9
    //   10 | |  ..........       |  | 10
    //   11 | |  ..........       |  | 11
    //   12 | \-I..........M------/  | 12
    //   13 |    ..........          | 13
    //   14 |    ..........          | 14
    //   15 |    .....  E            | 15
    //   16 |    .....  |            | 16
    //   17 \---J.....N-/            | 17
    //   18      .....               | 18
    //   19      .....               | 19
    //             F                 |
    //             |                 |
    //             \-----------------/
    //           0123456789012345 x (in tens)
    fn wrap_cube_input(&self) -> State {
        let &State { x, y, facing } = self;
        match facing {
            Facing::Up => {
                if y == -1 && (50..100).contains(&x) {
                    // A
                    State {
                        x: 0,
                        y: x + 100,
                        facing: Facing::Right,
                    }
                } else if y == -1 && (100..150).contains(&x) {
                    // B
                    State {
                        x: x - 100,
                        y: 199,
                        facing: Facing::Up,
                    }
                } else if y == 99 && (0..50).contains(&x) {
                    // C
                    State {
                        x: 50,
                        y: x + 50,
                        facing: Facing::Right,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Down => {
                if y == 50 && (100..150).contains(&x) {
                    // D
                    State {
                        x: 99,
                        y: x - 50,
                        facing: Facing::Left,
                    }
                } else if y == 150 && (50..100).contains(&x) {
                    // E
                    State {
                        x: 49,
                        y: x + 100,
                        facing: Facing::Left,
                    }
                } else if y == 200 && (0..50).contains(&x) {
                    // F
                    State {
                        x: x + 100,
                        y: 0,
                        facing: Facing::Down,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Left => {
                if x == 49 && (0..50).contains(&y) {
                    // G
                    State {
                        x: 0,
                        y: 149 - y,
                        facing: Facing::Right,
                    }
                } else if x == 49 && (50..100).contains(&y) {
                    // H
                    State {
                        x: y - 50,
                        y: 100,
                        facing: Facing::Down,
                    }
                } else if x == -1 && (100..150).contains(&y) {
                    // I
                    State {
                        x: 50,
                        y: 149 - y,
                        facing: Facing::Right,
                    }
                } else if x == -1 && (150..200).contains(&y) {
                    // J
                    State {
                        x: y - 100,
                        y: 0,
                        facing: Facing::Down,
                    }
                } else {
                    self.clone()
                }
            }
            Facing::Right => {
                if x == 150 && (0..50).contains(&y) {
                    // K
                    State {
                        x: 99,
                        y: 149 - y,
                        facing: Facing::Left,
                    }
                } else if x == 100 && (50..100).contains(&y) {
                    // L
                    State {
                        x: y + 50,
                        y: 49,
                        facing: Facing::Up,
                    }
                } else if x == 100 && (100..150).contains(&y) {
                    // M
                    State {
                        x: 149,
                        y: 149 - y,
                        facing: Facing::Left,
                    }
                } else if x == 50 && (150..200).contains(&y) {
                    // N
                    State {
                        x: y - 100,
                        y: 149,
                        facing: Facing::Up,
                    }
                } else {
                    self.clone()
                }
            }
        }
    }
}

enum Instruction {
    Forward(i64),
    Left,
    Right,
}

impl Instruction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            s => Instruction::Forward(s.parse().unwrap()),
        }
    }

    fn apply<F>(&self, board: &Board, state: &State, next_function: F) -> State
    where
        F: Fn(&State, &Board) -> State,
    {
        let &State { x, y, facing } = state;
        match self {
            Instruction::Forward(d) => {
                let mut state = state.clone();
                for _ in 0..*d {
                    let next_state = next_function(&state, board);
                    state = match board.tile_at(&next_state) {
                        Tile::None => unreachable!(),
                        Tile::Open => next_state,
                        Tile::Wall => break,
                    };
                }
                state
            }
            Instruction::Left => State {
                x,
                y,
                facing: facing.turn_left(),
            },
            Instruction::Right => State {
                x,
                y,
                facing: facing.turn_right(),
            },
        }
    }
}

fn follow_path<F>(input: &str, next_function: F) -> i64
where
    F: Fn(&State, &Board) -> State,
{
    let (board, instructions) = input.trim_end().split_once("\n\n").unwrap();
    let board = Board::from(board);
    let regex = Regex::new(r"(\d+|L|R)").unwrap();
    let instructions: Vec<Instruction> = regex
        .captures_iter(instructions)
        .map(|c| Instruction::from(&c[0]))
        .collect();

    // start with the leftmost empty tile of the top row
    let mut state = State {
        x: 0,
        y: 0,
        facing: Facing::Right,
    };
    while let Tile::None = board.tile_at(&state) {
        state.x += 1;
    }

    for instruction in instructions {
        state = instruction.apply(&board, &state, &next_function);
    }

    state.value()
}

pub fn part1(input: &str) -> i64 {
    follow_path(input, State::next_torus)
}

pub fn part2(input: &str) -> i64 {
    follow_path(input, State::next_cube_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day22.txt");
    const INPUT: &str = include_str!("../inputs/day22.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 6032);
        assert_eq!(part1(INPUT), 133174);
    }

    fn part2_example(input: &str) -> i64 {
        follow_path(input, State::next_cube_example)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_example(EXAMPLE), 5031);
        assert_eq!(part2(INPUT), 15410);
    }
}
