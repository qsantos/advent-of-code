use std::collections::HashSet;

fn count_tail_visits(filename: &str, length: usize) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut visited_positions = HashSet::new();
    let mut knots = [(0i32, 0i32)].repeat(length + 1);
    visited_positions.insert(*knots.last().unwrap());
    for line in contents.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        assert_eq!(command.len(), 2);
        let direction = command[0];
        let (di, dj) = match direction {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => unreachable!(),
        };
        let n_steps: u32 = command[1].parse().unwrap();
        for _ in 0..n_steps {
            let (mut hi, mut hj) = knots[0];
            hi += di;
            hj += dj;
            knots[0] = (hi, hj);

            for knot in &mut knots[1..] {
                let (mut ti, mut tj) = knot;
                if hi.abs_diff(ti) > 1 || hj.abs_diff(tj) > 1 {
                    ti += (hi - ti).signum();
                    tj += (hj - tj).signum();
                    *knot = (ti, tj);
                }
                hi = ti;
                hj = tj;
            }
            visited_positions.insert(*knots.last().unwrap());
            // println!("head:{hi},{hj} tail:{ti},{tj}");
        }
    }
    visited_positions.len()
}

fn puzzle1() {
    assert_eq!(count_tail_visits("example1", 1), 13);
    assert_eq!(count_tail_visits("example2", 1), 88);
    assert_eq!(count_tail_visits("input", 1), 5902);
}

fn puzzle2() {
    assert_eq!(count_tail_visits("example1", 9), 1);
    assert_eq!(count_tail_visits("example2", 9), 36);
    assert_eq!(count_tail_visits("input", 9), 2445);
}

fn main() {
    puzzle1();
    puzzle2();
}
