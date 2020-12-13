use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        let tmp_r = r; r = old_r - q * r; old_r = tmp_r;
        let tmp_s = s; s = old_s - q * s; old_s = tmp_s;
        let tmp_t = t; t = old_t - q * t; old_t = tmp_t;
    }
    return (old_r, old_s, old_r);
}

fn invert(a: i64, m: i64) -> i64 {
    let (_g, x, _y) = extended_gcd(a, m);
    assert_eq!(_g, 1);
    return (x + m) % m;
}

fn crt(remainders: &Vec::<i64>, modulos: &Vec::<i64>) -> i64 {
    assert_eq!(remainders.len(), modulos.len());
    let bn = modulos.iter().fold(1, |a, b| a * b);
    let mut x = 0;
    for (ai, ni) in remainders.iter().zip(modulos.iter()) {
        let bni = bn / ni;
        let bmi = invert(bni, *ni);
        x += ai * bni * bmi;
        x %= bn;
    }
    return x;
}

fn puzzle1(buses: &Vec::<(i64, i64)>, earliest: i64) {
    let (_offset, bus_id) = buses.iter().min_by_key(|(_offset, bus_id)| bus_id - (earliest % bus_id)).unwrap();
    let wait = bus_id - (earliest % bus_id);
    println!("{}", bus_id * wait);
}

fn puzzle2(buses: &Vec::<(i64, i64)>) {
    let remainders: Vec<i64> = buses.iter().map(|(offset, bus_id)| bus_id - (offset % bus_id)).collect();
    let modulos: Vec<i64> = buses.iter().map(|(_offset, bus_id)| *bus_id).collect();
    println!("{}", crt(&remainders, &modulos));
}

fn main() {
    let f = File::open("input").expect("Could not open file");
    let mut lines = BufReader::new(f).lines();
    let earliest = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let bus_line = lines.next().unwrap().unwrap();
    let mut buses = Vec::<(i64, i64)>::new();
    let mut offset = 0;
    for bus_id in bus_line.split(',') {
        if bus_id != "x" {
            buses.push((offset, bus_id.parse().unwrap()));
        }
        offset += 1;
    }

    puzzle1(&buses, earliest);
    puzzle2(&buses);
}
