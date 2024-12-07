fn signal_strength(filename: &str) -> i32 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut cycle = 0;
    let mut regx = 1;
    let mut total_signal_strength = 0;

    let mut next_cycle = |regx| {
        cycle += 1;
        if cycle % 40 == 20 {
            total_signal_strength += cycle * regx;
        }
    };

    for line in contents.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        let opcode = command[0];
        match opcode {
            "noop" => next_cycle(regx),
            "addx" => {
                next_cycle(regx);
                next_cycle(regx);
                let arg: i32 = command[1].parse().unwrap();
                regx += arg;
            }
            _ => unreachable!(),
        }
    }
    total_signal_strength
}

fn draw(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut cycle = 0;
    let mut regx = 1;

    let mut next_cycle = |regx: i32| {
        let col = cycle % 40;
        if regx.abs_diff(col) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if col == 39 {
            println!();
        }
        cycle += 1;
    };

    for line in contents.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        let opcode = command[0];
        match opcode {
            "noop" => next_cycle(regx),
            "addx" => {
                next_cycle(regx);
                next_cycle(regx);
                let arg: i32 = command[1].parse().unwrap();
                regx += arg;
            }
            _ => unreachable!(),
        }
    }
}

fn puzzle1() {
    assert_eq!(signal_strength("example"), 13140);
    assert_eq!(signal_strength("input"), 14540);
}

fn puzzle2() {
    draw("example");
    draw("input");
}

fn main() {
    puzzle1();
    puzzle2();
}
