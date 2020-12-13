use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn count_slope(grid: &Vec<Vec<u8>>, dx: usize, dy: usize) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut c = 0;
    while y < grid.len() {
        if grid[y][x] == b'#' {
            c += 1;
        }
        x = (x + dx) % grid[0].len();
        y += dy;
    }
    return c;
}

fn puzzle1(grid: &Vec<Vec<u8>>) {
    println!("{}", count_slope(grid, 3, 1));
}

fn puzzle2(grid: &Vec<Vec<u8>>) {
    println!("{}",
        count_slope(grid, 1, 1) *
        count_slope(grid, 3, 1) *
        count_slope(grid, 5, 1) *
        count_slope(grid, 7, 1) *
        count_slope(grid, 1, 2)
    );
}

fn main() {
    let f = File::open("input").expect("Could not open file");
    let grid: Vec<Vec<u8>> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().trim_end().bytes().collect())
        .collect();

    puzzle1(&grid);
    puzzle2(&grid);
}
