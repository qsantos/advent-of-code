fn transpose(block: &[&[u8]]) -> Vec<Vec<u8>> {
    let rows = block.len();
    let cols = block[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| block[i][j]).collect())
        .collect()
}

fn hamming(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}

fn middle_row<R: AsRef<[u8]>>(block: &[R], d: usize) -> Option<usize> {
    let rows = block.len();
    for middle in 1..rows {
        let width = middle.min(rows - middle);
        let h: usize = (0..width)
            .map(|i| hamming(block[middle - 1 - i].as_ref(), block[middle + i].as_ref()))
            .sum();
        if h == d {
            return Some(middle);
        }
    }
    None
}

pub fn part12(input: &str, d: usize) -> usize {
    let mut sum = 0;
    for block in input.trim().split("\n\n") {
        let block: Vec<&[u8]> = block.as_bytes().split(|b| *b == b'\n').collect();
        if let Some(middle) = middle_row(&block, d) {
            sum += middle * 100;
        } else {
            let block = transpose(&block);
            let middle = middle_row(&block, d).unwrap();
            sum += middle;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day13.txt");
    const INPUT: &str = include_str!("../inputs/day13.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part12(EXAMPLE, 0), 405);
        assert_eq!(part12(INPUT, 0), 37975);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part12(EXAMPLE, 1), 400);
        assert_eq!(part12(INPUT, 1), 32497);
    }
}
