fn hash(s: &[u8]) -> u8 {
    let mut ret = 0u8;
    for c in s {
        ret = ret.wrapping_add(*c).wrapping_mul(17);
    }
    ret
}

fn part1(filename: &str) -> u64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut total = 0u64;
    for part in data.trim().as_bytes().split(|b| *b == b',') {
        total += hash(part) as u64;
    }
    total
}

fn main() {
    assert_eq!(part1("example"), 1320);
    assert_eq!(part1("input"), 498538);
}
