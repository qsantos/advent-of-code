pub fn part1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut regx = 1;
    let mut total_signal_strength = 0;

    let mut next_cycle = |regx| {
        cycle += 1;
        if cycle % 40 == 20 {
            total_signal_strength += cycle * regx;
        }
    };

    for line in input.lines() {
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

pub fn part2(input: &str) -> String {
    let mut cycle = 0;
    let mut regx = 1;

    let mut output = String::new();

    let mut next_cycle = |regx: i32| {
        let col = cycle % 40;
        if regx.abs_diff(col) <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }
        if col == 39 {
            output.push('\n');
        }
        cycle += 1;
    };

    for line in input.lines() {
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

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day10.txt");
    const INPUT: &str = include_str!("../inputs/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13140);
        assert_eq!(part1(INPUT), 14540);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_OUTPUT: &str = "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n\
        ";

        // EHZFZHCZ
        const INPUT_OUTPUT: &str = "\
            ####.#..#.####.####.####.#..#..##..####.\n\
            #....#..#....#.#.......#.#..#.#..#....#.\n\
            ###..####...#..###....#..####.#......#..\n\
            #....#..#..#...#.....#...#..#.#.....#...\n\
            #....#..#.#....#....#....#..#.#..#.#....\n\
            ####.#..#.####.#....####.#..#..##..####.\n\
        ";

        assert_eq!(part2(EXAMPLE), EXAMPLE_OUTPUT);
        assert_eq!(part2(INPUT), INPUT_OUTPUT);
    }
}
