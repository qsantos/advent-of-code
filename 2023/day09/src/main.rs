use std::fs::File;
use std::io::{BufRead, BufReader};

/*
10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0

is the sum of

2 times
 0   0   0   1   4  10   20
   0   0   1   3   6   10
     0   1   2   3   4
       1   1   1   1
         0   0   0
(the last entry is 3 among 6)

0 times
 0   0   1   3   6  10  15
   0   1   2   3   4   5
     1   1   1   1   1
       0   0   0   0
         0   0   0
(the last entry is 2 among 6)

3 times
 0   1   2   3   4   5   6
   1   1   1   1   1   1
     0   0   0   0   0
       0   0   0   0
         0   0   0
(the last entry is 1 among 6)

10 times
 1   1   1   1   1   1   1
   0   0   0   0   0   0
     0   0   0   0   0
       0   0   0   0
         0   0   0
(the last entry is 0 among 6)
*/

fn binomial_coefficient(k: i64, n: i64) -> i64 {
    assert!(k <= n);
    let mut ret = 1;
    let k = k.min(n - k);
    for i in 1..=k {
        ret = (ret * (n + 1 - i)) / i;
    }
    ret
}

#[test]
fn test_binomial_coefficient() {
    assert_eq!(binomial_coefficient(0, 0), 1);
    assert_eq!(binomial_coefficient(2, 4), 6);
    assert_eq!(binomial_coefficient(20, 50), 47129212243960);
}

fn coefficients_of_series(mut series: Vec<i64>) -> Vec<i64> {
    let mut coeffs = Vec::new();
    while !series.iter().all(|n| *n == 0) {
        coeffs.push(series[0]);
        let diffs: Vec<i64> = series.iter().skip(1).zip(series.iter()).map(|(a, b)| a - b).collect();
        series = diffs;
    }
    coeffs
}

fn series_element_at(coeffs: &Vec<i64>, n: i64) -> i64 {
    coeffs.into_iter().enumerate().map(|(k, coeff)| coeff * binomial_coefficient(k as i64, n)).sum()
}

fn part1(filename: &str) -> i64 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buf).unwrap() != 0 {
        let line = buf.trim();
        let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        let n = numbers.len() as i64;
        let coeffs = coefficients_of_series(numbers);
        sum += series_element_at(&coeffs, n);
        buf.clear();
    }
    sum
}

fn main() {
    assert_eq!(part1("example"), 114);
    assert_eq!(part1("input"), 1995001648);
}
