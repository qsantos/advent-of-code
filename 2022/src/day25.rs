fn from_snafu_digit(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn to_snafu_digit(d: i64) -> char {
    match d {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!(),
    }
}

fn from_snafu_number(s: &str) -> i64 {
    let mut acc = 0;
    for c in s.chars() {
        acc = acc * 5 + from_snafu_digit(c);
    }
    acc
}

fn to_snafu_number(mut n: i64) -> String {
    let mut ret = String::new();
    while n > 0 {
        let r = match n.rem_euclid(5) {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => -2,
            4 => -1,
            _ => unreachable!(),
        };
        ret.push(to_snafu_digit(r));
        n -= r;
        assert_eq!(n % 5, 0);
        n /= 5;
    }
    ret.chars().rev().collect()
}

fn sum(filename: &str) -> String {
    let contents = std::fs::read_to_string(filename).unwrap();
    let sum = contents.lines().map(from_snafu_number).sum::<i64>();
    to_snafu_number(sum)
}

fn main() {
    assert_eq!(sum("example"), "2=-1=0");
    assert_eq!(sum("input"), "2--1=0=-210-1=00=-=1");
}
