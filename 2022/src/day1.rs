fn read_elves(input: &str) -> Vec<i32> {
    let mut ret = vec![];
    let mut current_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            ret.push(current_elf);
            current_elf = 0;
        } else {
            let number: i32 = line.parse().expect("Could not parse line");
            current_elf += number;
        }
    }
    ret.push(current_elf);
    ret
}

pub fn part1(input: &str) -> i32 {
    let elves = read_elves(input);
    *elves.iter().max().expect("No elves!")
}

pub fn part2(input: &str, count: usize) -> i32 {
    let mut elves = read_elves(input);
    elves.sort();
    elves.iter().rev().take(count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day1.txt");
    const INPUT: &str = include_str!("../inputs/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 24000);
        assert_eq!(part1(INPUT), 69310);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE, 3), 45000);
        assert_eq!(part2(INPUT, 3), 206104);
    }
}
