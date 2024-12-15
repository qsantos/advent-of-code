use std::fmt::Display;

fn find_start(map: &[&mut [u8]]) -> (i32, i32) {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'@' {
                return (i as i32, j as i32);
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

fn try_move_v(map: &[&mut [u8]], i: i32, j: i32, di: i32) -> bool {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
        return false;
    }
    let c = map[i as usize][j as usize];
    match c {
        b'.' => true,
        b'@' => try_move_v(map, i + di, j, di),
        b'[' => try_move_v(map, i + di, j, di) && try_move_v(map, i + di, j + 1, di),
        b']' => try_move_v(map, i + di, j - 1, di) && try_move_v(map, i + di, j, di),
        b'#' => false,
        _ => panic!("Invalid cell {}", c as char),
    }
}

fn do_move_v(map: &mut [&mut [u8]], i: i32, j: i32, di: i32) {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
        panic!("Trying to move out of the map");
    }
    let c = map[i as usize][j as usize];
    match c {
        b'.' => (),
        b'@' => {
            do_move_v(map, i + di, j, di);
            map[(i + di) as usize][j as usize] = b'@';
            map[i as usize][j as usize] = b'.';
        }
        b'[' => {
            do_move_v(map, i + di, j, di);
            do_move_v(map, i + di, j + 1, di);
            map[(i + di) as usize][j as usize] = b'[';
            map[(i + di) as usize][(j + 1) as usize] = b']';
            map[i as usize][j as usize] = b'.';
            map[i as usize][(j + 1) as usize] = b'.';
        }
        b']' => {
            do_move_v(map, i + di, j - 1, di);
            do_move_v(map, i + di, j, di);
            map[(i + di) as usize][(j - 1) as usize] = b'[';
            map[(i + di) as usize][j as usize] = b']';
            map[i as usize][(j - 1) as usize] = b'.';
            map[i as usize][j as usize] = b'.';
        }
        b'#' => {
            panic!("Trying to move to a wall");
        }
        _ => panic!("Invalid cell {}", c as char),
    }
}

pub fn part2(input: &str) -> impl Display {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let mut map = map.as_bytes().to_vec();
    let mut map: Vec<&mut [u8]> = map.split_mut(|&b| b == b'\n').collect();
    let cols = map[0].len() as i32;
    let (mut i, mut j) = find_start(&map);
    'outer: for &move_ in moves.as_bytes() {
        if move_ == b'\n' {
            continue;
        }
        //print_map(&map);
        //println!("move: {}", move_ as char);
        //println!();
        let (di, dj) = dir_to_didj(move_);
        if di == 0 {
            // horizontal move
            // check first cell
            let mut n = 1;
            let nj = j + dj * n;
            let fc = map[i as usize][nj as usize];
            if fc == b'#' {
                continue 'outer; // skip move
            } else if fc == b'.' {
            } else {
                n += 2;
                // check next cells
                loop {
                    let nj = j + dj * n;
                    if !(0..cols).contains(&nj) {
                        continue 'outer; // skip move
                    }
                    let c = map[i as usize][nj as usize];
                    if c == b'.' {
                        break; // move the boxes
                    } else if c == fc {
                        n += 2;
                    } else {
                        continue 'outer; // skip move
                    }
                }
            }
            // move the boxes
            if n > 1 {
                let a = map[i as usize][(j + dj) as usize];
                let b = map[i as usize][(j + dj * 2) as usize];
                // left:
                // ..[][]@
                // a = ]
                // b = [
                // n = 5
                //  k = 5: [ (b)
                //  k = 4: ] (a)
                //  k = 3: [ (b)
                //  k = 2: ] (a)
                //  k = 1: @
                //  k = 0: .
                // right:
                // @[][]..
                // a = [
                // b = ]
                // n = 5
                //  k = 5: ] (b)
                //  k = 4: [ (a)
                //  k = 3: ] (b)
                //  k = 2: [ (a)
                //  k = 1: @
                //  k = 0: .
                for k in 2..=n {
                    let side = if k % 2 == 0 { a } else { b };
                    map[i as usize][(j + k * dj) as usize] = side;
                }
            }
            // move the robot
            let nj = j + dj;
            map[i as usize][nj as usize] = b'@';
            map[i as usize][j as usize] = b'.';
            j = nj;
        } else {
            // vertical move
            if try_move_v(&map, i, j, di) {
                do_move_v(&mut map, i, j, di);
                i += di;
            }
        }
    }
    // sum GPS coordinates
    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'[' {
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2).to_string(), "9021");
        assert_eq!(part2(INPUT).to_string(), "1509724");
    }
}
