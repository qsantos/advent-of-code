struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        assert_eq!(parts.len(), 2);
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

fn count_relations<T>(filename: &str, relation: T) -> i32
where
    T: Fn(&Range, &Range) -> bool,
{
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            assert_eq!(parts.len(), 2);
            let first = Range::from_str(parts[0]);
            let second = Range::from_str(parts[1]);
            (relation(&first, &second) || relation(&second, &first)) as i32
        })
        .sum()
}

fn puzzle1() {
    assert_eq!(count_relations("example", Range::contains), 2);
    assert_eq!(count_relations("input", Range::contains), 540);
}

fn puzzle2() {
    assert_eq!(count_relations("example", Range::overlaps), 4);
    assert_eq!(count_relations("input", Range::overlaps), 872);
}

fn main() {
    puzzle1();
    puzzle2();
}
