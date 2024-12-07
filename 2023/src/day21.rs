use std::collections::HashSet;

fn reachable(grid: &[&[u8]], (si, sj): (usize, usize), steps: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut previous = HashSet::new();
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert((si as isize, sj as isize));
    let mut odds = 0;
    let mut evens = 1; // {start} at step 0
    for step in 1..=steps {
        next.clear();
        for (i, j) in current.iter().copied() {
            for (ni, nj) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if !(0 <= ni && ni < rows && 0 <= nj && nj < cols) {
                    continue;
                }
                if grid[ni as usize][nj as usize] == b'#' {
                    continue;
                }
                if previous.contains(&(ni, nj)) {
                    continue;
                }
                next.insert((ni, nj));
            }
        }
        (previous, current, next) = (current, next, previous);
        if step % 2 == 0 {
            evens += current.len();
        } else {
            odds += current.len();
        }
    }
    if steps % 2 == 0 {
        evens
    } else {
        odds
    }
}

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|c| *c == b'S').map(|j| (i, j)))
        .next()
        .unwrap()
}

pub fn part1(input: &str, steps: usize) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let start = find_start(&grid);
    reachable(&grid, start, steps)
}

pub fn part2(input: &str, steps: usize) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // see https://github.com/Manitary/advent-of-code/blob/master/2023/python/day21.png

    // the blocks are square
    assert_eq!(rows, cols);

    // the blocks have a middle
    assert_eq!(rows % 2, 1);

    // we start right in the middle of the block
    let start = find_start(&grid);
    assert_eq!(start, (rows / 2, cols / 2));

    // axes are clear of rocks
    assert!(grid.iter().all(|row| row[cols / 2] != b'#'));
    assert!(grid[rows / 2].iter().all(|c| *c != b'#'));

    // we reach exactly the extremity of another block
    let full_blocks = (steps - rows / 2) / rows;
    assert_eq!(steps, full_blocks * rows + rows / 2);

    let in_extremities = {
        let d = rows - 1;
        let in_bottom_block = reachable(&grid, (rows - 1, cols / 2), d);
        let in_top_block = reachable(&grid, (0, cols / 2), d);
        let in_left_block = reachable(&grid, (rows / 2, cols - 1), d);
        let in_right_block = reachable(&grid, (rows / 2, 0), d);
        in_bottom_block + in_top_block + in_left_block + in_right_block
    };

    let in_full_blocks = {
        let in_even_block = reachable(&grid, start, 2 * rows);
        let in_odd_block = reachable(&grid, start, 2 * rows + 1);
        // make sure we have found all the reachable plots
        assert_eq!(reachable(&grid, start, 2 * rows + 2), in_even_block);
        assert_eq!(reachable(&grid, start, 2 * rows + 3), in_odd_block);
        // the middle block is a odd block
        assert_eq!(steps % 2, 1);
        let even_blocks = full_blocks * full_blocks;
        let odd_blocks = (full_blocks - 1) * (full_blocks - 1);
        even_blocks * in_even_block + odd_blocks * in_odd_block
    };

    let in_small_corners = {
        let d = rows / 2 - 1;
        let in_upper_right = reachable(&grid, (0, 0), d);
        let in_upper_left = reachable(&grid, (0, cols - 1), d);
        let in_lower_right = reachable(&grid, (rows - 1, 0), d);
        let in_lower_left = reachable(&grid, (rows - 1, cols - 1), d);
        let corners = full_blocks;
        corners * (in_upper_left + in_upper_right + in_lower_left + in_lower_right)
    };

    let in_big_corners = {
        let d = rows + rows / 2 - 1;
        let in_upper_right = reachable(&grid, (0, 0), d);
        let in_upper_left = reachable(&grid, (0, cols - 1), d);
        let in_lower_right = reachable(&grid, (rows - 1, 0), d);
        let in_lower_left = reachable(&grid, (rows - 1, cols - 1), d);
        let corners = full_blocks - 1;
        corners * (in_upper_left + in_upper_right + in_lower_left + in_lower_right)
    };

    in_full_blocks + in_extremities + in_small_corners + in_big_corners
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day21.txt");
    const INPUT: &str = include_str!("../inputs/day21.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 6), 16);
        assert_eq!(part1(INPUT, 64), 3666);
    }

    #[test]
    fn test_part2() {
        // part2 designed for specific format of input; does not work with example
        assert_eq!(part2(INPUT, 26501365), 609298746763952);
    }
}
