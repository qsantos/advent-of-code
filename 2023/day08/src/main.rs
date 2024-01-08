use std::collections::HashMap;

fn parse_input(data: &str) -> (&str, HashMap<&str, [&str; 2]>) {
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
        network.insert(node, [left, right]);
    }

    (directions, network)
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let (directions, network) = parse_input(&data);

    let mut node = "AAA";
    let mut steps = 0;
    let mut directions = directions.chars().cycle();
    while node != "ZZZ" {
        let direction = directions.next().unwrap();
        let left_right = network[node];
        node = left_right[if direction == 'L' { 0 } else { 1 }];
        steps += 1;
    }
    steps
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(21, 6), 3);
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(21, 6), 42);
}

fn lcm_many<I: Iterator<Item = i64>>(it: I) -> i64 {
    it.reduce(lcm).unwrap()
}

fn detect_cycle(directions: &str, network: &HashMap<&str, [&str; 2]>, start: &str) -> i64 {
    assert!(start.ends_with('A'));
    let mut node = start;
    let mut visited = HashMap::new();
    let mut ending_steps = Vec::new();
    for (steps, (direction_step, direction)) in directions.chars().enumerate().cycle().enumerate() {
        if let Some(last_seen) = visited.insert((direction_step, node), steps) {
            let cycle_length = steps - last_seen;
            // not guaranteed, but the input seem to verify it
            assert_eq!(ending_steps.len(), 1);
            let ending_offset = ending_steps[0];
            // not guaranteed, but the input seem to verify it
            assert_eq!(ending_offset, cycle_length);
            return cycle_length as i64;
        }
        if node.ends_with('Z') {
            ending_steps.push(steps);
        }
        node = network[node][if direction == 'L' { 0 } else { 1 }];
    }
    unreachable!();
}

fn part2(filename: &str) -> i64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let (directions, network) = parse_input(&data);
    let nodes: Vec<&str> = network
        .keys()
        .copied()
        .filter(|n| n.ends_with('A'))
        .collect();
    // with the asserts in detect_cycle, we now we want to solve x = 0 mod n_i for various n_i. In
    // this case, we can just use LCM
    lcm_many(nodes
        .into_iter()
        .map(|n| detect_cycle(directions, &network, n))
    )
}

fn main() {
    assert_eq!(part1("example1"), 2);
    assert_eq!(part1("example2"), 6);
    assert_eq!(part1("input"), 13939);

    assert_eq!(part2("input"), 8906539031197);
}
