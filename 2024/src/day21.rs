use std::collections::HashMap;
use std::fmt::Display;

fn num_go_and_press(start: u8, end: u8) -> &'static [&'static [u8]] {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    match (start, end) {
        (b'A', b'A') => &[b"A"],
        (b'A', b'0') => &[b"<A"],
        (b'A', b'1') => &[b"^<<A"],
        (b'A', b'2') => &[b"^<A", b"<^A"],
        (b'A', b'3') => &[b"^A"],
        (b'A', b'4') => &[b"^^<<A"],
        (b'A', b'5') => &[b"^^<A", b"<^^A"],
        (b'A', b'6') => &[b"^^A"],
        (b'A', b'7') => &[b"^^^<<A"],
        (b'A', b'8') => &[b"^^^<A", b"<^^^A"],
        (b'A', b'9') => &[b"^^^A"],
        (b'0', b'A') => &[b">A"],
        (b'0', b'0') => &[b"A"],
        (b'0', b'1') => &[b"^<A"],
        (b'0', b'2') => &[b"^A"],
        (b'0', b'3') => &[b"^>A", b">^A"],
        (b'0', b'4') => &[b"^^<A"],
        (b'0', b'5') => &[b"^^A"],
        (b'0', b'6') => &[b"^^>A", b">^^A"],
        (b'0', b'7') => &[b"^^^<A"],
        (b'0', b'8') => &[b"^^^A"],
        (b'0', b'9') => &[b"^^^>A", b">^^^A"],
        (b'1', b'A') => &[b">>vA"],
        (b'1', b'0') => &[b">vA"],
        (b'1', b'1') => &[b"A"],
        (b'1', b'2') => &[b">A"],
        (b'1', b'3') => &[b">>A"],
        (b'1', b'4') => &[b"^A"],
        (b'1', b'5') => &[b">^A", b"^>A"],
        (b'1', b'6') => &[b"^>>A", b">>^A"],
        (b'1', b'7') => &[b"^^A"],
        (b'1', b'8') => &[b"^^>A", b">^^A"],
        (b'1', b'9') => &[b"^^>>A", b">>^^A"],
        (b'2', b'A') => &[b">vA", b"v>A"],
        (b'2', b'0') => &[b"vA"],
        (b'2', b'1') => &[b"<A"],
        (b'2', b'2') => &[b"A"],
        (b'2', b'3') => &[b">A"],
        (b'2', b'4') => &[b"^<A", b"<^A"],
        (b'2', b'5') => &[b"^A"],
        (b'2', b'6') => &[b">^A", b"^>A"],
        (b'2', b'7') => &[b"^^<A", b"<^^A"],
        (b'2', b'8') => &[b"^^A"],
        (b'2', b'9') => &[b"^^>A", b">^^A"],
        (b'3', b'A') => &[b"vA"],
        (b'3', b'0') => &[b"v<A", b"<vA"],
        (b'3', b'1') => &[b"<<A"],
        (b'3', b'2') => &[b"<A"],
        (b'3', b'3') => &[b"A"],
        (b'3', b'4') => &[b"^<<A", b"<<^A"],
        (b'3', b'5') => &[b"^<A", b"<^A"],
        (b'3', b'6') => &[b"^A"],
        (b'3', b'7') => &[b"<<^^A", b"^^<<A"],
        (b'3', b'8') => &[b"^^<A", b"<^^A"],
        (b'3', b'9') => &[b"^^A"],
        (b'4', b'A') => &[b">>vvA"],
        (b'4', b'0') => &[b">vvA"],
        (b'4', b'1') => &[b"vA"],
        (b'4', b'2') => &[b">vA", b"v>A"],
        (b'4', b'3') => &[b">>vA", b"v>>A"],
        (b'4', b'4') => &[b"A"],
        (b'4', b'5') => &[b">A"],
        (b'4', b'6') => &[b">>A"],
        (b'4', b'7') => &[b"^A"],
        (b'4', b'8') => &[b">^A", b"^>A"],
        (b'4', b'9') => &[b">>^A", b"^>>A"],
        (b'5', b'A') => &[b">vvA", b"vv>A"],
        (b'5', b'0') => &[b"vvA"],
        (b'5', b'1') => &[b"v<A", b"<vA"],
        (b'5', b'2') => &[b"vA"],
        (b'5', b'3') => &[b">vA", b"v>A"],
        (b'5', b'4') => &[b"<A"],
        (b'5', b'5') => &[b"A"],
        (b'5', b'6') => &[b">A"],
        (b'5', b'7') => &[b"^<A", b"<^A"],
        (b'5', b'8') => &[b"^A"],
        (b'5', b'9') => &[b">^A", b"^>A"],
        (b'6', b'A') => &[b"vvA"],
        (b'6', b'0') => &[b"vv<A", b"<vvA"],
        (b'6', b'1') => &[b"v<<A", b"<<vA"],
        (b'6', b'2') => &[b"v<A", b"<vA"],
        (b'6', b'3') => &[b"vA"],
        (b'6', b'4') => &[b"<<A"],
        (b'6', b'5') => &[b"<A"],
        (b'6', b'6') => &[b"A"],
        (b'6', b'7') => &[b"^<<A", b"<<^A"],
        (b'6', b'8') => &[b"^<A", b"<^A"],
        (b'6', b'9') => &[b"^A"],
        (b'7', b'A') => &[b">>vvvA"],
        (b'7', b'0') => &[b">vvvA"],
        (b'7', b'1') => &[b"vvA"],
        (b'7', b'2') => &[b">vvA", b"vv>A"],
        (b'7', b'3') => &[b">>vvA", b"vv>>A"],
        (b'7', b'4') => &[b"vA"],
        (b'7', b'5') => &[b">vA", b"v>A"],
        (b'7', b'6') => &[b">>vA", b"v>>A"],
        (b'7', b'7') => &[b"A"],
        (b'7', b'8') => &[b">A"],
        (b'7', b'9') => &[b">>A"],
        (b'8', b'A') => &[b">vvvA", b"vvv>A"],
        (b'8', b'0') => &[b"vvvA"],
        (b'8', b'1') => &[b"<vvA", b"vv<A"],
        (b'8', b'2') => &[b"vvA"],
        (b'8', b'3') => &[b">vvA", b"vv>A"],
        (b'8', b'4') => &[b"<vA", b"v<A"],
        (b'8', b'5') => &[b"vA"],
        (b'8', b'6') => &[b">vA", b"v>A"],
        (b'8', b'7') => &[b"<A"],
        (b'8', b'8') => &[b"A"],
        (b'8', b'9') => &[b">A"],
        (b'9', b'A') => &[b"vvvA"],
        (b'9', b'0') => &[b"vvv<A", b"<vvvA"],
        (b'9', b'1') => &[b"vv<<A", b"<<vvA"],
        (b'9', b'2') => &[b"vv<A", b"<vvA"],
        (b'9', b'3') => &[b"vvA"],
        (b'9', b'4') => &[b"<<vA", b"v<<A"],
        (b'9', b'5') => &[b"<vA", b"v<A"],
        (b'9', b'6') => &[b"vA"],
        (b'9', b'7') => &[b"<<A"],
        (b'9', b'8') => &[b"<A"],
        (b'9', b'9') => &[b"A"],
        _ => unreachable!(),
    }
}

fn dir_go_and_press(start: u8, end: u8) -> &'static [&'static [u8]] {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match (start, end) {
        (b'A', b'A') => &[b"A"],
        (b'A', b'<') => &[b"v<<A"],
        (b'A', b'>') => &[b"vA"],
        (b'A', b'^') => &[b"<A"],
        (b'A', b'v') => &[b"v<A", b"<vA"],
        (b'<', b'A') => &[b">>^A"],
        (b'<', b'<') => &[b"A"],
        (b'<', b'>') => &[b">>A"],
        (b'<', b'^') => &[b">^A"],
        (b'<', b'v') => &[b">A"],
        (b'>', b'A') => &[b"^A"],
        (b'>', b'<') => &[b"<<A"],
        (b'>', b'>') => &[b"A"],
        (b'>', b'^') => &[b"<^A", b"^<A"],
        (b'>', b'v') => &[b"<A"],
        (b'^', b'A') => &[b">A"],
        (b'^', b'<') => &[b"v<A"],
        (b'^', b'>') => &[b"v>A", b">vA"],
        (b'^', b'^') => &[b"A"],
        (b'^', b'v') => &[b"vA"],
        (b'v', b'A') => &[b"^>A", b">^A"],
        (b'v', b'<') => &[b"<A"],
        (b'v', b'>') => &[b">A"],
        (b'v', b'^') => &[b"^A"],
        (b'v', b'v') => &[b"A"],
        _ => unreachable!(),
    }
}

fn path_cost(costs: &HashMap<(u8, u8), usize>, path: &[u8]) -> usize {
    let mut c = b'A';
    let mut cost = 0;
    for &x in path {
        println!("{} -> {}", c as char, x as char);
        cost += costs[&(c, x)];
        c = x;
    }
    cost
}

pub fn type_on_keypads(line: &str, n_robots: usize) -> usize {
    // cost to move from button X to button Y and press button Y
    let mut costs: HashMap<(u8, u8), usize> = HashMap::new();
    // top-level directional keypad (cost of moving is 0)
    for &start in b"A<>^v" {
        for &end in b"A<>^v" {
            costs.insert((start, end), 1);
        }
    }
    // intermediate directional keypads
    for _ in 0..n_robots {
        let mut new_costs: HashMap<(u8, u8), usize> = HashMap::new();
        for &start in b"A<>^v" {
            for &end in b"A<>^v" {
                new_costs.insert(
                    (start, end),
                    dir_go_and_press(start, end)
                        .iter()
                        .map(|&path| path_cost(&costs, path))
                        .min()
                        .unwrap(),
                );
            }
        }
        costs = new_costs;
    }
    // numeric keypad
    let mut new_costs: HashMap<(u8, u8), usize> = HashMap::new();
    for &start in b"A0123456789" {
        for &end in b"A0123456789" {
            new_costs.insert(
                (start, end),
                num_go_and_press(start, end)
                    .iter()
                    .map(|&path| path_cost(&costs, path))
                    .min()
                    .unwrap(),
            );
        }
    }
    costs = new_costs;
    path_cost(&costs, line.as_bytes())
}

pub fn part1(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypads(line, 2);
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        println!("{length} * {value} = {}", length * value);
        total += length * value;
    }
    total
}

pub fn part2(input: &str) -> impl Display {
    let mut total = 0;
    for line in input.lines() {
        let length = type_on_keypads(line, 25);
        let numeric = line.strip_suffix("A").unwrap();
        let value: usize = numeric.parse().unwrap();
        total += length * value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day21.txt");
    const INPUT: &str = include_str!("../inputs/day21.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "126384");
        assert_eq!(part1(INPUT).to_string(), "162740");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).to_string(), "203640915832208");
    }
}
