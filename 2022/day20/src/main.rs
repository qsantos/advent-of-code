fn decrypt(filename: &str, decryption_key: i64, iterations: usize) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let numbers: Vec<i64> = contents
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect();
    let n = (numbers.len() - 1) as i64;
    let mut number_refs: Vec<&i64> = numbers.iter().collect();
    for _ in 0..iterations {
        for number_ref in &numbers {
            let pos = number_refs
                .iter()
                .position(|&other_ref| std::ptr::eq(number_ref, other_ref))
                .unwrap();
            let new_pos = (pos as i64 + number_ref).rem_euclid(n) as usize;
            if new_pos < pos {
                number_refs[new_pos..=pos].rotate_right(1);
            } else {
                number_refs[pos..=new_pos].rotate_left(1);
            }
        }
    }
    let pos = number_refs
        .iter()
        .position(|&number_ref| *number_ref == 0)
        .unwrap();
    let a = *number_refs[(pos + 1000) % numbers.len()];
    let b = *number_refs[(pos + 2000) % numbers.len()];
    let c = *number_refs[(pos + 3000) % numbers.len()];
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
