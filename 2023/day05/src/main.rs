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

fn part2(filename: &str) -> u64 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    // read seeds
    assert_ne!(reader.read_line(&mut buf).unwrap(), 0);
    let mut ranges = Vec::new();
    let line = buf.trim();
    let (_, seeds) = line.split_once(": ").unwrap();
    let mut numbers = seeds.split_whitespace().map(|seed| seed.parse::<u64>().unwrap());
    while let Some(start) = numbers.next() {
        let len = numbers.next().unwrap();
        ranges.push((start, len));
    }
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
        map.sort_unstable_by_key(|(_dst_start, src_start, _len)| *src_start);

        // check that the mapping ranges do not overlap
        let mut max = 0;
        for (_dst_start, src_start, len) in map.iter() {
            assert!(*src_start >= max);
            max = *src_start + *len;
        }
        maps.push(map);

        if buf.len() == 0 {
            break;
        }
    }

    for map in maps.iter() {
        let mut new_ranges = Vec::new();
        for (mut start, mut len) in ranges.iter() {
            for (map_dst_start, map_src_start, map_len) in map {
                if *map_src_start + *map_len <= start {
                    // map is before range, do nothing
                } else if *map_src_start <= start {
                    // range starts in map range
                    if start + len <= *map_src_start + *map_len {
                        // range contained in map range, map the full range
                        new_ranges.push((start - *map_src_start + *map_dst_start, len));
                        // nothing more to map
                        len = 0;
                        break;
                    } else {
                        // range ends after map range, keep remaining range
                        let mapped_range_len = *map_src_start + *map_len - start;
                        new_ranges.push((start - *map_src_start + *map_dst_start, mapped_range_len));
                        // remaining part
                        start = *map_src_start + *map_len;
                        len = len - mapped_range_len;
                    }
                } else if *map_src_start <= start + len {
                    // map range starts in range
                    if *map_src_start + *map_len <= start + len {
                        // map range ends in range
                        // this part remains unmapped
                        let unmapped_part_len = *map_src_start - start;
                        new_ranges.push((start, unmapped_part_len));
                        // mapped part
                        new_ranges.push((*map_dst_start, *map_len));
                        // remaining part
                        start = start + unmapped_part_len + *map_len;
                        len = len - unmapped_part_len - *map_len;
                    } else {
                        // map range ends after range
                        // this part remains unmapped
                        let unmapped_part_len = *map_src_start - start;
                        new_ranges.push((start, unmapped_part_len));
                        // mapped part
                        new_ranges.push((*map_dst_start, len - unmapped_part_len));
                        // nothing more to map
                        len = 0;
                        break;
                    }
                } else {
                    // map is after range, do nothing
                }
            }
            if len != 0 {
                new_ranges.push((start, len));
            }
        }
        ranges = new_ranges;
    }
    ranges.into_iter().min().unwrap().0
}

fn main() {
    assert_eq!(part1("example"), 35);
    assert_eq!(part1("input"), 484023871);

    println!("{}", part2("example"));
    println!("{}", part2("input"));

}
