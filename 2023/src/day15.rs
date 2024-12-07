use std::collections::BTreeMap;

fn hash(s: &[u8]) -> u8 {
    let mut ret = 0u8;
    for c in s {
        ret = ret.wrapping_add(*c).wrapping_mul(17);
    }
    ret
}

pub fn part1(input: &str) -> u64 {
    let mut total = 0u64;
    for part in input.trim().as_bytes().split(|b| *b == b',') {
        total += hash(part) as u64;
    }
    total
}

pub fn part2(input: &str) -> u64 {
    let mut boxes: BTreeMap<u8, Vec<(Vec<u8>, u8)>> = BTreeMap::new();
    for part in input.trim().as_bytes().split(|b| *b == b',') {
        let c = *part.last().unwrap();
        if c == b'-' {
            let label = &part[..part.len() - 1];
            if let Some(r#box) = boxes.get_mut(&hash(label)) {
                r#box.retain(|v| &*v.0 != label);
            }
        } else {
            let sep = part.len() - 2;
            assert_eq!(part[sep], b'=');
            let (label, focal_length) = part.split_at(sep);
            let focal_length = focal_length[1] - b'0';
            boxes
                .entry(hash(label))
                .and_modify(|e| {
                    if let Some(v) = e.iter_mut().find(|v| &*v.0 == label) {
                        v.1 = focal_length;
                    } else {
                        e.push((label.to_vec(), focal_length));
                    }
                })
                .or_insert_with(|| vec![(label.to_vec(), focal_length)]);
        }
    }
    let mut total = 0u64;
    for i in 0..256 {
        if let Some(r#box) = boxes.get(&(i as u8)) {
            for (j, v) in r#box.iter().enumerate() {
                total += (i + 1) * ((j + 1) as u64) * (v.1 as u64);
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day15.txt");
    const INPUT: &str = include_str!("../inputs/day15.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1320);
        assert_eq!(part1(INPUT), 498538);

        assert_eq!(part2(EXAMPLE), 145);
        assert_eq!(part2(INPUT), 286278);
    }
}
