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

fn main() {
    assert_eq!(part1("example"), 136);
    assert_eq!(part1("input"), 110565);
}
