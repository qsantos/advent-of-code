use std::collections::HashMap;

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.trim().as_bytes().split(|b| *b == b'\n').collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_load = 0;
    for j in 0..cols {
        let mut stack_start = 0;
        let mut stack_size = 0;
        for i in 0..rows {
            let c = grid[i][j];
            if c == b'#' {
                stack_start = i + 1;
                stack_size = 0;
            } else if c == b'O' {
                total_load += rows - (stack_start + stack_size);
                stack_size += 1;
            }
        }
    }
    total_load
}

fn move_north(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cols = grid[0].len();
    for j in 0..cols {
        let mut stack_start = 0;
        let mut stack_size = 0;
        for i in 0..rows {
            let c = grid[i][j];
            if c == b'#' {
                stack_start = i + 1;
                stack_size = 0;
            } else if c == b'O' {
                grid[i][j] = b'.';
                grid[stack_start + stack_size][j] = b'O';
                stack_size += 1;
            }
        }
    }
    grid
}

fn rotate(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut ret = Vec::new();
    for j in 0..cols {
        let mut row = Vec::new();
        for i in 0..rows {
            row.push(grid[rows - 1 - i][j]);
        }
        ret.push(row);
    }
    ret
}

fn total_load(grid: &Vec<Vec<u8>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_load = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == b'O' {
                total_load += rows - i;
            }
        }
    }
    total_load
}

fn cycle(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    grid = move_north(grid);
    grid = rotate(grid);
    grid = move_north(grid);
    grid = rotate(grid);
    grid = move_north(grid);
    grid = rotate(grid);
    grid = move_north(grid);
    grid = rotate(grid);
    grid
}

fn part2(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut grid: Vec<Vec<u8>> = data
        .trim()
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|row| row.to_vec())
        .collect();
    println!("{}", total_load(&grid));
    let mut seen = HashMap::new();
    let cycle_target = 1000000000;
    for cycle_count in 0..cycle_target {
        if let Some(last_seen) = seen.insert(grid.clone(), cycle_count) {
            let loop_length = cycle_count - last_seen;
            let remaining = (cycle_target - cycle_count) % loop_length;
            for _ in 0..remaining {
                grid = cycle(grid);
            }
            break;
        }
        grid = cycle(grid);
    }
    total_load(&grid)
}

fn main() {
    assert_eq!(part1("example"), 136);
    assert_eq!(part1("input"), 110565);

    assert_eq!(part2("example"), 64);
    assert_eq!(part2("input"), 89845);
}
