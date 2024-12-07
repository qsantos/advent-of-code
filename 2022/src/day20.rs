mod implicit_treap;

use implicit_treap::ImplicitTreap;

fn decrypt(filename: &str, decryption_key: i64, iterations: usize) -> i64 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut treap = ImplicitTreap::new();
    let mut node_keys = Vec::new();
    let mut zero_idx = 0;
    for (i, line) in contents.lines().enumerate() {
        let number = line.parse::<i64>().unwrap();
        let node_key = treap.push(number * decryption_key);
        node_keys.push(node_key);
        if number == 0 {
            zero_idx = i;
        }
    }
    let n = (node_keys.len() - 1) as i64;
    for _ in 0..iterations {
        for node_key in &mut node_keys {
            let pos = treap.node_index(*node_key);
            let number = treap.remove_at(pos).unwrap();
            let new_pos = (pos as i64 + number).rem_euclid(n) as usize;
            *node_key = treap.insert(new_pos, number);
        }
    }
    let zero_node = node_keys[zero_idx];
    let pos = treap.node_index(zero_node);
    assert_eq!(treap[pos], 0);
    let a = treap[(pos + 1000) % treap.len()];
    let b = treap[(pos + 2000) % treap.len()];
    let c = treap[(pos + 3000) % treap.len()];
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
