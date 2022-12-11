use std::collections::BinaryHeap;
use std::collections::VecDeque;

enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn read(s: &str) -> Self {
        match s {
            "+" => Operator::Plus,
            "*" => Operator::Times,
            _ => unreachable!(),
        }
    }

    fn eval(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Plus => left + right,
            Operator::Times => left * right,
        }
    }
}

enum Operand {
    Old,
    Literal(u64),
}

impl Operand {
    fn read(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            s => Operand::Literal(s.parse().unwrap()),
        }
    }

    fn eval(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Literal(v) => *v,
        }
    }
}

struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

impl Operation {
    fn read(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        assert_eq!(parts.len(), 3);
        Operation {
            operator: Operator::read(parts[1]),
            left: Operand::read(parts[0]),
            right: Operand::read(parts[2]),
        }
    }

    fn eval(&self, old: u64) -> u64 {
        self.operator
            .eval(self.left.eval(old), self.right.eval(old))
    }
}

struct MonkeyId(usize);

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspection_count: u64,
}

impl Monkey {
    fn read(s: &str) -> Self {
        let mut lines = s.lines();
        assert!(lines.next().unwrap().starts_with("Monkey "));
        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|worry| worry.parse().unwrap())
            .collect();
        let operation = Operation::read(
            lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = ")
                .unwrap(),
        );
        let divisor = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let if_true = MonkeyId(
            lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        );
        let if_false = MonkeyId(
            lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        );
        Monkey {
            items,
            operation,
            divisor,
            if_true,
            if_false,
            inspection_count: 0,
        }
    }
}

fn round1(monkeys: &mut [Monkey]) {
    for id in 0..monkeys.len() {
        while let Some(item) = monkeys[id].items.pop_front() {
            monkeys[id].inspection_count += 1;
            let monkey = &monkeys[id];
            let worry = monkey.operation.eval(item) / 3;
            let monkey_id = if worry % monkey.divisor == 0 {
                monkey.if_true.0
            } else {
                monkey.if_false.0
            };
            monkeys[monkey_id].items.push_back(worry);
        }
    }
}

fn round2(monkeys: &mut [Monkey]) {
    let modulo: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    for id in 0..monkeys.len() {
        while let Some(item) = monkeys[id].items.pop_front() {
            monkeys[id].inspection_count += 1;
            let monkey = &monkeys[id];
            let worry = monkey.operation.eval(item) % modulo;
            let monkey_id = if worry % monkey.divisor == 0 {
                monkey.if_true.0
            } else {
                monkey.if_false.0
            };
            monkeys[monkey_id].items.push_back(worry);
        }
    }
}

fn top_monkeys(monkeys: &[Monkey]) -> u64 {
    let mut heap: BinaryHeap<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();
    let a = heap.pop().unwrap();
    let b = heap.pop().unwrap();
    a * b
}

fn find_top_monkeys<F>(filename: &str, n_rounds: usize, round: F) -> u64
where
    F: Fn(&mut [Monkey]),
{
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut monkeys: Vec<_> = contents.split("\n\n").map(Monkey::read).collect();
    for _ in 0..n_rounds {
        round(&mut monkeys);
    }
    top_monkeys(&monkeys)
}

fn puzzle1() {
    assert_eq!(find_top_monkeys("example", 20, round1), 10605);
    assert_eq!(find_top_monkeys("input", 20, round1), 120756);
}

fn puzzle2() {
    assert_eq!(find_top_monkeys("example", 10_000, round2), 2713310158);
    assert_eq!(find_top_monkeys("input", 10_000, round2), 39109444654);
}

fn main() {
    puzzle1();
    puzzle2();
}
