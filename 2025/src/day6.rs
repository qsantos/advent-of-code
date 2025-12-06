fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let numbers_lines: Vec<Vec<u64>> = lines[0..lines.len() - 1].iter().map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
    let ops: Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();
    for numbers in numbers_lines.iter() {
        assert_eq!(numbers.len(), ops.len());
    }
    let mut ret = 0u64;
    for (i, &op) in ops.iter().enumerate() {
        let numbers = numbers_lines.iter().map(|numbers| numbers[i]);
        if op == "+" {
            ret += numbers.sum::<u64>();
        } else {
            ret += numbers.product::<u64>();
        }
    }
    ret
}

fn part2(input: &str) -> u64 {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    for row in grid.iter() {
        assert_eq!(row.len(), cols);
    }
    let mut total = 0;
    let mut numbers = Vec::new();
    for col in (0..cols).rev() {
        let mut number = 0u64;
        for row in 0..rows - 1 {
            let c = grid[row][col];
            if c != b' ' {
                number *= 10;
                number += (c - b'0') as u64;
            }
        }
        if number != 0 {
            numbers.push(number);
        }
        let op = grid[rows - 1][col];
        if op == b'+' {
            total += numbers.drain(..).sum::<u64>();
        } else if op == b'*' {
            total += numbers.drain(..).product::<u64>();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day6.txt");
    const INPUT: &str = include_str!("../inputs/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
        assert_eq!(part1(INPUT), 3968933219902);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
        assert_eq!(part2(INPUT), 6019576291014);
    }
}
