use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;

#[derive(Eq, PartialEq, Clone)]
enum Value {
    Integer(u64),
    List(Vec<Value>),
}

impl Value {
    fn read(s: &str) -> Value {
        fn parse_value(s: &[char], i: &mut usize) -> Value {
            if s[*i] == '[' {
                let mut v = Vec::new();
                *i += 1;
                while s[*i] != ']' {
                    v.push(parse_value(s, i));
                    match s[*i] {
                        ',' => *i += 1,
                        ']' => (),
                        c => panic!("Unexpected character {c} at position {}", *i),
                    }
                }
                *i += 1;
                Value::List(v)
            } else {
                let start = *i;
                let mut stop = *i + 1;
                while stop < s.len() && s[stop] != ']' && s[stop] != ',' {
                    stop += 1;
                }
                *i = stop;
                Value::Integer(s[start..stop].iter().collect::<String>().parse().unwrap())
            }
        }
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let ret = parse_value(&chars, &mut i);
        assert_eq!(i, chars.len());
        ret
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::List(l) => write!(f, "{:?}", l),
            Value::Integer(i) => write!(f, "{:?}", i),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn vec_cmp<T>(a: &[T], b: &[T]) -> Ordering
where
    T: PartialOrd,
{
    let la = a.len();
    let lb = b.len();
    for i in 0..(la.min(lb)) {
        if a[i] < b[i] {
            return Ordering::Less;
        } else if a[i] > b[i] {
            return Ordering::Greater;
        }
    }
    la.cmp(&lb)
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => vec_cmp(a, b),
            (&Value::Integer(a), Value::List(_)) => Value::List(vec![Value::Integer(a)]).cmp(other),
            (Value::List(_), &Value::Integer(b)) => self.cmp(&Value::List(vec![Value::Integer(b)])),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let values: Vec<_> = pair.lines().map(Value::read).collect();
        assert_eq!(values.len(), 2);
        let left = &values[0];
        let right = &values[1];
        if left < right {
            total += i + 1;
        }
    }
    total
}

pub fn part2(input: &str) -> usize {
    let divider1 = Value::read("[[2]]");
    let divider2 = Value::read("[[6]]");
    let mut packets: Vec<Value> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Value::read)
        .collect();
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let idx1 = packets.iter().position(|v| v == &divider1).unwrap() + 1;
    let idx2 = packets.iter().position(|v| v == &divider2).unwrap() + 1;
    idx1 * idx2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day13.txt");
    const INPUT: &str = include_str!("../inputs/day13.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
        assert_eq!(part1(INPUT), 5393);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 140);
        assert_eq!(part2(INPUT), 26712);
    }
}
