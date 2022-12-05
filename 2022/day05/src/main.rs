struct Map {
    stacks: Vec<Vec<char>>,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let grid: Vec<&str> = s.lines().collect();
        let max_height = grid.len() - 1;
        let stack_count = grid[max_height].split_whitespace().count();
        let stacks = (0..stack_count)
            .map(|stack_index| {
                let column_index = 1 + stack_index * 4;
                (0..max_height)
                    .rev()
                    .map(|height| grid[height].chars().nth(column_index).unwrap())
                    .take_while(|c| *c != ' ')
                    .collect()
            })
            .collect();
        Map { stacks }
    }

    #[allow(dead_code)]
    fn display(self) {
        for stack in &self.stacks {
            for item in stack {
                print!("{} ", item);
            }
            println!();
        }
    }

    fn code(self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

struct Action {
    count: usize,
    from: usize,
    to: usize,
}

impl Action {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(' ');
        assert_eq!(parts.next().unwrap(), "move");
        let count = parts.next().unwrap().parse().unwrap();
        assert_eq!(parts.next().unwrap(), "from");
        let from = parts.next().unwrap().parse().unwrap();
        assert_eq!(parts.next().unwrap(), "to");
        let to = parts.next().unwrap().parse().unwrap();
        Action { count, from, to }
    }

    fn apply(&self, map: &mut Map, reverse: bool) {
        let from = &mut map.stacks[self.from - 1];
        let from_start = from.len() - self.count;
        let mut moved: Vec<char> = from.drain(from_start..).collect();
        if reverse {
            moved.reverse();
        }
        let to = &mut map.stacks[self.to - 1];
        to.extend(moved);
    }
}

fn move_creates(filename: &str, reverse: bool) -> String {
    let contents = std::fs::read_to_string(filename).unwrap();
    let parts: Vec<&str> = contents.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let mut map = Map::from_str(parts[0]);
    for action in parts[1].lines() {
        let action = Action::from_str(action);
        action.apply(&mut map, reverse);
    }
    map.code()
}

fn puzzle1() {
    assert_eq!(move_creates("example", true), "CMZ");
    assert_eq!(move_creates("input", true), "FJSRQCFTN");
}

fn puzzle2() {
    assert_eq!(move_creates("example", false), "MCD");
    assert_eq!(move_creates("input", false), "CJVLJQPHS");
}

fn main() {
    puzzle1();
    puzzle2();
}
