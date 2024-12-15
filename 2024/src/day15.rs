use std::fmt::Display;

fn find_start(map: &[&mut [u8]]) -> (i32, i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'@' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start found");
}

fn dir_to_didj(dir: u8) -> (i32, i32) {
    match dir {
        b'^' => (-1, 0),
        b'v' => (1, 0),
        b'<' => (0, -1),
        b'>' => (0, 1),
        _ => panic!("Invalid direction '{}'", dir as char),
    }
}

pub fn print_map(map: &[&mut [u8]]) {
    for row in map {
        println!("{}", String::from_utf8_lossy(row));
    }
}

pub fn part1(input: &str) -> impl Display {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map.as_bytes().to_vec();
    let mut map: Vec<&mut [u8]> = map.split_mut(|&b| b == b'\n').collect();
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let (mut i, mut j) = find_start(&map);
    'outer: for &move_ in moves.as_bytes() {
        if move_ == b'\n' {
            continue;
        }
        //print_map(&map);
        //println!("Move: {}", move_ as char);
        //println!();
        let (di, dj) = dir_to_didj(move_);
        let mut n = 1;
        loop {
            let ni = i + di * n;
            let nj = j + dj * n;
            if !(0..rows).contains(&ni) || !(0..cols).contains(&nj) {
                continue 'outer; // skip move
            }
            let c = map[ni as usize][nj as usize];
            match c {
                b'#' => continue 'outer, // skip move
                b'.' => break,           // move the boxes
                b'O' => n += 1,
                _ => panic!("Invalid cell {}", c as char),
            }
        }
        // move the boxes
        if n > 1 {
            let ni = i + di * n;
            let nj = j + dj * n;
            map[ni as usize][nj as usize] = b'O';
        }
        // move the robot
        let ni = i + di;
        let nj = j + dj;
        map[ni as usize][nj as usize] = b'@';
        map[i as usize][j as usize] = b'.';
        (i, j) = (ni, nj);
    }
    // sum GPS coordinates
    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'O' {
                total += i * 100 + j;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day15-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day15-2.txt");
    const INPUT: &str = include_str!("../inputs/day15.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "2028");
        assert_eq!(part1(EXAMPLE2).to_string(), "10092");
        assert_eq!(part1(INPUT).to_string(), "1475249");
    }
}
