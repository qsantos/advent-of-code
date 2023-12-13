fn transpose(block: &[&[u8]]) -> Vec<Vec<u8>> {
    let rows = block.len();
    let cols = block[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| block[i][j]).collect())
        .collect()
}

fn middle_row<R: AsRef<[u8]>>(block: &[R]) -> Option<usize> {
    let rows = block.len();
    for middle in 1..rows {
        let width = middle.min(rows - middle);
        if (0..width).all(|i| block[middle - 1 - i].as_ref() == block[middle + i].as_ref()) {
            return Some(middle);
        }
    }
    None
}

fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut sum = 0;
    for block in data.trim().split("\n\n") {
        let block: Vec<&[u8]> = block.as_bytes().split(|b| *b == b'\n').collect();
        if let Some(middle) = middle_row(&block) {
            sum += middle * 100;
        } else {
            let block = transpose(&block);
            let middle = middle_row(&block).unwrap();
            sum += middle;
        }
    }
    sum
}

fn main() {
    assert_eq!(part1("example"), 405);
    assert_eq!(part1("input"), 37975);
}
