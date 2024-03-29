fn transpose(block: &[&[u8]]) -> Vec<Vec<u8>> {
    let rows = block.len();
    let cols = block[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| block[i][j]).collect())
        .collect()
}

fn hamming(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).map(|(a, b)| if a == b { 0 } else { 1 }).sum()
}

fn middle_row<R: AsRef<[u8]>>(block: &[R], d: usize) -> Option<usize> {
    let rows = block.len();
    for middle in 1..rows {
        let width = middle.min(rows - middle);
        let h: usize = (0..width).map(|i| hamming(block[middle - 1 - i].as_ref(), block[middle + i].as_ref())).sum();
        if h == d {
            return Some(middle);
        }
    }
    None
}

fn part12(filename: &str, d: usize) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut sum = 0;
    for block in data.trim().split("\n\n") {
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

fn main() {
    assert_eq!(part12("example", 0), 405);
    assert_eq!(part12("input", 0), 37975);

    assert_eq!(part12("example", 1), 400);
    assert_eq!(part12("input", 1), 32497);
}
