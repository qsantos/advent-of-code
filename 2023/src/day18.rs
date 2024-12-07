#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn to_delta(self) -> (i64, i64) {
        match self {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        }
    }
}

fn parse_line1(line: &str) -> (Direction, u64) {
    // direction
    let (direction, rest) = line.split_once(' ').unwrap();
    let direction = match direction.as_bytes()[0] {
        b'L' => Direction::Left,
        b'U' => Direction::Up,
        b'R' => Direction::Right,
        b'D' => Direction::Down,
        _ => unreachable!(),
    };
    // distance
    let (distance, _) = rest.split_once(' ').unwrap();
    let distance = distance.parse().unwrap();
    // return
    (direction, distance)
}

fn parse_line2(line: &str) -> (Direction, u64) {
    let (_, color) = line.rsplit_once('#').unwrap();
    // direction
    let direction = color.as_bytes()[5];
    let direction = match direction {
        b'0' => Direction::Right,
        b'1' => Direction::Down,
        b'2' => Direction::Left,
        b'3' => Direction::Up,
        _ => unreachable!(),
    };
    // distance
    let distance = u64::from_str_radix(&color[..5], 16).unwrap();
    // return
    (direction, distance)
}

struct Trench {
    boundary: u64,
    vertices: Vec<(i64, i64)>,
}

fn parse_trench<F: Fn(&str) -> (Direction, u64)>(input: &str, parse_line: F) -> Trench {
    let mut boundary = 0;
    let mut vertices = Vec::new();
    let (mut i, mut j) = (0, 0);
    for line in input.lines() {
        let (direction, distance) = parse_line(line);
        let (di, dj) = direction.to_delta();
        boundary += distance;
        i += di * distance as i64;
        j += dj * distance as i64;
        vertices.push((i, j));
    }
    assert!(
        vertices.contains(&(0, 0)),
        "the instructions must get back to the starting point"
    );
    Trench { boundary, vertices }
}

/// Compute the area of a given polygon
fn shoelace_formula(vertices: &[(i64, i64)]) -> u64 {
    let mut twice_area = 0;
    for (p1, p2) in vertices.iter().zip(vertices.iter().skip(1).cycle()) {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        twice_area += x1 * y2 - x2 * y1;
    }
    twice_area.unsigned_abs() / 2
}

fn solve(trench: &Trench) -> u64 {
    let Trench { boundary, vertices } = trench;
    let area = shoelace_formula(vertices);
    // By Pick's theorem, area = interior + boundary / 2 - 1.
    // So interior + boundary = area + 1 + boundary / 2.
    area + 1 + boundary / 2
}

pub fn part1(input: &str) -> u64 {
    solve(&parse_trench(input, parse_line1))
}

pub fn part2(input: &str) -> u64 {
    solve(&parse_trench(input, parse_line2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day18.txt");
    const INPUT: &str = include_str!("../inputs/day18.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 62);
        assert_eq!(part1(INPUT), 44436);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 952408144115);
        assert_eq!(part2(INPUT), 106941819907437);
    }
}
