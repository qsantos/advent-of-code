fn decrypt(filename: &str, decryption_key: i64, iterations: usize) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut numbers: Vec<(usize, i64)> = contents
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<i64>().unwrap() * decryption_key))
        .collect();
    for _ in 0..iterations {
        for i in 0..numbers.len() {
            let pos = numbers.iter().position(|&(j, _)| i == j).unwrap();
            let (_, number) = numbers[pos];
            let new_pos = (pos as i64 + number).rem_euclid((numbers.len() - 1) as i64) as usize;
            if new_pos < pos {
                numbers[new_pos..=pos].rotate_right(1);
            } else {
                numbers[pos..=new_pos].rotate_left(1);
            }
        }
    }
    let pos = numbers.iter().position(|&(_, n)| n == 0).unwrap();
    let (_, a) = numbers[(pos + 1000) % numbers.len()];
    let (_, b) = numbers[(pos + 2000) % numbers.len()];
    let (_, c) = numbers[(pos + 3000) % numbers.len()];
    a + b + c
}

fn puzzle1() {
    assert_eq!(decrypt("example", 1, 1), 3);
    assert_eq!(decrypt("input", 1, 1), 5904);
}

fn puzzle2() {
    assert_eq!(decrypt("example", 811589153, 10), 1623178306);
    assert_eq!(decrypt("input", 811589153, 10), 8332585833851);
}

fn main() {
    puzzle1();
    puzzle2();
}
