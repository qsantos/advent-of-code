fn part12(input: &str) -> (u32, u32) {
    let mut sum = 0;
    let mut total_power = 0;
    let mut game_id = 1;
    for line in input.lines() {
        let (_, sets) = line.split_once(": ").unwrap();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                if color == "red" {
                    max_red = max_red.max(count);
                } else if color == "green" {
                    max_green = max_green.max(count);
                } else if color == "blue" {
                    max_blue = max_blue.max(count);
                } else {
                    panic!("unexpected color {color}");
                }
            }
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += game_id;
        }
        let power = max_red * max_green * max_blue;
        total_power += power;
        game_id += 1;
    }
    (sum, total_power)
}

pub fn part1(input: &str) -> u32 {
    part12(input).0
}

pub fn part2(input: &str) -> u32 {
    part12(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day2.txt");
    const INPUT: &str = include_str!("../inputs/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 8);
        assert_eq!(part1(INPUT), 2156);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 2286);
        assert_eq!(part2(INPUT), 66909);
    }
}
