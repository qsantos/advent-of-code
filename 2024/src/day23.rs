use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::iter::once;

fn parse(input: &str) -> (HashSet<&str>, HashSet<(&str, &str)>) {
    let mut computers = HashSet::new();
    let mut links = HashSet::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        computers.insert(left);
        computers.insert(right);
        links.insert((left, right));
        links.insert((right, left));
    }
    (computers, links)
}

pub fn part1(input: &str) -> impl Display {
    let (computers, links) = parse(input);
    let computers: Vec<_> = computers.into_iter().collect();
    let mut count = 0;
    for (i, &a) in computers.iter().enumerate() {
        for (j, &b) in computers[i + 1..].iter().enumerate() {
            if !links.contains(&(a, b)) {
                continue;
            }
            for &c in computers[i + 1 + j + 1..].iter() {
                if !links.contains(&(a, c)) || !links.contains(&(b, c)) {
                    continue;
                }
                if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
                    continue;
                }
                count += 1;
            }
        }
    }
    count
}

fn bron_kerbosch<'a>(
    n: &'a HashMap<&'a str, Vec<&'a str>>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
) -> Option<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }
    let pv: Vec<&str> = p.iter().copied().collect();
    let mut ret: Option<HashSet<&str>> = None;
    for v in pv {
        let new_r: HashSet<&str> = r.union(&once(v).collect()).copied().collect();
        let new_p = p
            .intersection(&n[v].iter().cloned().collect())
            .cloned()
            .collect();
        let new_x = x
            .intersection(&n[v].iter().cloned().collect())
            .cloned()
            .collect();
        if let Some(r) = bron_kerbosch(n, new_r, new_p, new_x) {
            if r.len() > ret.as_ref().map(|ret| ret.len()).unwrap_or(0) {
                ret = Some(r);
            }
        }
        p.remove(v);
        x.insert(v);
    }
    ret
}

pub fn part2(input: &str) -> impl Display {
    let (computers, links) = parse(input);
    let mut n = HashMap::new();
    for (a, b) in links.iter().copied() {
        n.entry(a).or_insert_with(Vec::new).push(b);
        n.entry(b).or_insert_with(Vec::new).push(a);
    }
    let r = HashSet::new();
    let p = computers.clone();
    let x = HashSet::new();
    let solution = bron_kerbosch(&n, r, p, x).unwrap();
    let mut v: Vec<&str> = solution.into_iter().collect();
    v.sort();
    v.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day23.txt");
    const INPUT: &str = include_str!("../inputs/day23.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "7");
        assert_eq!(part1(INPUT).to_string(), "1175");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).to_string(), "co,de,ka,ta");
        assert_eq!(
            part2(INPUT).to_string(),
            "bw,dr,du,ha,mm,ov,pj,qh,tz,uv,vq,wq,xw"
        );
    }
}
