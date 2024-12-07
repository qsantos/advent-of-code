use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

enum Operator {
    Plus,
    Minus,
    Times,
    Divided,
}

impl Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Times,
            "/" => Operator::Divided,
            _ => unreachable!(),
        }
    }

    fn eval(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Plus => left + right,
            Operator::Minus => left - right,
            Operator::Times => left * right,
            Operator::Divided => left / right,
        }
    }

    fn solve_left(&self, target: i64, right: i64) -> i64 {
        match self {
            Operator::Plus => target - right,
            Operator::Minus => target + right,
            Operator::Times => target / right,
            Operator::Divided => target * right,
        }
    }

    fn solve_right(&self, target: i64, left: i64) -> i64 {
        match self {
            Operator::Plus => target - left,
            Operator::Minus => left - target,
            Operator::Times => target / left,
            Operator::Divided => left / target,
        }
    }
}

enum Node {
    Literal(i64),
    Operation(Operator, String, String),
}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn from(input: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (name, children) = line.split_once(": ").unwrap();
            let parts: Vec<&str> = children.split(' ').collect();
            let node = match parts.len() {
                1 => {
                    // literal
                    let value: i64 = parts[0].parse().unwrap();
                    Node::Literal(value)
                }
                3 => {
                    // operation
                    let left = parts[0];
                    let op = parts[1];
                    let right = parts[2];
                    Node::Operation(Operator::from(op), String::from(left), String::from(right))
                }
                _ => unreachable!(),
            };
            nodes.insert(String::from(name), node);
        }
        Graph { nodes }
    }

    fn eval_at(&self, name: &str) -> i64 {
        match &self.nodes[name] {
            Node::Literal(value) => *value,
            Node::Operation(op, left, right) => {
                let left = self.eval_at(left);
                let right = self.eval_at(right);
                op.eval(left, right)
            }
        }
    }

    fn eval(&self) -> i64 {
        self.eval_at("root")
    }

    fn find_node(&self, name: &str) -> Vec<Direction> {
        fn aux(
            nodes: &HashMap<String, Node>,
            target: &str,
            current: &str,
        ) -> Option<Vec<Direction>> {
            match &nodes[current] {
                Node::Literal(_) => {
                    if current == target {
                        Some(vec![])
                    } else {
                        None
                    }
                }
                Node::Operation(_, left, right) => {
                    if let Some(mut dirs) = aux(nodes, target, left) {
                        dirs.push(Direction::Left);
                        return Some(dirs);
                    }
                    if let Some(mut dirs) = aux(nodes, target, right) {
                        dirs.push(Direction::Right);
                        return Some(dirs);
                    }
                    None
                }
            }
        }
        aux(&self.nodes, name, "root").unwrap()
    }

    fn solve(&self) -> i64 {
        let mut dirs = self.find_node("humn");
        let mut current = "root";
        let mut target;

        // special case the root to force operator to be equality
        let dir = dirs.pop().unwrap();
        match &self.nodes[current] {
            Node::Literal(_) => unreachable!(),
            Node::Operation(_, left, right) => match dir {
                Direction::Left => {
                    let other = self.eval_at(right);
                    target = other;
                    current = left;
                }
                Direction::Right => {
                    let other = self.eval_at(left);
                    target = other;
                    current = right;
                }
            },
        }

        while let Some(dir) = dirs.pop() {
            match &self.nodes[current] {
                Node::Literal(_) => unreachable!(),
                Node::Operation(op, left, right) => match dir {
                    Direction::Left => {
                        let other = self.eval_at(right);
                        target = op.solve_left(target, other);
                        current = left;
                    }
                    Direction::Right => {
                        let other = self.eval_at(left);
                        target = op.solve_right(target, other);
                        current = right;
                    }
                },
            }
        }

        target
    }
}

pub fn part1(input: &str) -> i64 {
    Graph::from(input).eval()
}

pub fn part2(input: &str) -> i64 {
    Graph::from(input).solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day21.txt");
    const INPUT: &str = include_str!("../inputs/day21.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 152);
        assert_eq!(part1(INPUT), 83056452926300);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 301);
        assert_eq!(part2(INPUT), 3469704905529);
    }
}
