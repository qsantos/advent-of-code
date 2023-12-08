use std::collections::HashMap;

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut lines = data.lines();

    // read directions
    let directions = lines.next().unwrap();

    // skip empty lines
    lines.next();
    // read networks
    let mut network = HashMap::new();
    for line in lines {
        let (node, left_right) = line.split_once(" = ").unwrap();
        // trim opening and closing parentheses
        let left_right = &left_right[1..left_right.len() - 1];
        let (left, right) = left_right.split_once(", ").unwrap();
        network.insert(node, (left, right));
    }

    let mut node = "AAA";
    let mut steps = 0;
    let mut directions = directions.chars().cycle();
    while node != "ZZZ" {
        let direction = directions.next().unwrap();
        let left_right = network[node];
        node = if direction == 'L' { left_right.0 } else { left_right.1 };
        steps += 1;
    }
    steps
}

fn main() {
    assert_eq!(part1("example1"), 2);
    assert_eq!(part1("example2"), 6);
    assert_eq!(part1("input"), 13939);
}
