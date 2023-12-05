use std::fs::File;
use std::io::{BufReader, BufRead};

fn part1(filename: &str) -> u64 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    // read seeds
    assert_ne!(reader.read_line(&mut buf).unwrap(), 0);
    let seeds: Vec<u64> = {
        let line = buf.trim();
        let (_, seeds) = line.split_once(": ").unwrap();
        seeds.split_whitespace().map(|seed| seed.parse().unwrap()).collect()
    };
    buf.clear();
    // empty line
    assert_ne!(reader.read_line(&mut buf).unwrap(), 0);
    buf.clear();

    let mut maps = Vec::new();
    loop {
        // map name
        assert_ne!(reader.read_line(&mut buf).unwrap(), 0);
        buf.clear();

        // rules
        let mut map = Vec::new();
        while reader.read_line(&mut buf).unwrap() > 1 {
            let line = buf.trim();

            let (starts, len) = line.rsplit_once(' ').unwrap();
            let (dst_start, src_start) = starts.split_once(' ').unwrap();

            let dst_start: u64 = dst_start.parse().unwrap();
            let src_start: u64 = src_start.parse().unwrap();
            let len: u64 = len.parse().unwrap();

            map.push((dst_start, src_start, len));

            buf.clear();
        }
        maps.push(map);

        if buf.len() == 0 {
            break;
        }
    }

    let mut numbers = Vec::new();
    for mut number in seeds {
        for map in maps.iter() {
            let mut new_number = number;
            for (dst_start, src_start, len) in map.iter() {
                if (*src_start..=(*src_start + *len)).contains(&number) {
                    new_number = dst_start + (number - src_start);
                    break;
                }
            }
            number = new_number;
        }
        numbers.push(number);
    }
    numbers.into_iter().min().unwrap()
}

fn main() {
    assert_eq!(part1("example"), 35);
    assert_eq!(part1("input"), 484023871);

}
