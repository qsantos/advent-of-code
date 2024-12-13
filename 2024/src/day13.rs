use std::fmt::Display;

fn parse_button(button: &str) -> (i128, i128) {
    let (_, xy) = button.split_once(": ").unwrap();
    let (x, y) = xy.split_once(", ").unwrap();
    let (_, x) = x.split_once("+").unwrap();
    let (_, y) = y.split_once("+").unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    (x, y)
}

fn parse_prize(prize: &str) -> (i128, i128) {
    let (_, xy) = prize.split_once(": ").unwrap();
    let (x, y) = xy.split_once(", ").unwrap();
    let (_, x) = x.split_once("=").unwrap();
    let (_, y) = y.split_once("=").unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    (x, y)
}

fn machine_cost(machine: &str, offset: i128) -> Option<i128> {
    let (button_a, rest) = machine.split_once("\n").unwrap();
    let (button_b, prize) = rest.split_once("\n").unwrap();
    let (ax, ay) = parse_button(button_a);
    let (bx, by) = parse_button(button_b);
    let (px, py) = parse_prize(prize);
    let px = px + offset;
    let py = py + offset;
    // na * ax + nb * bx = px
    // na * ay + nb * by = py
    // ⇓
    // na = (px - nb * bx) / ax
    // (px - nb * bx) / ax * ay + nb * by = py
    // ⇓
    // (px - nb * bx) * ay + nb * ax * by = ax * py
    // ⇓
    // px * ay - nb * bx * ay + nb * ax * by = ax * py
    // ⇓
    // nb * (ax * by - bx * ay) = ax * py - px * ay
    // ⇓
    // nb = (ax * py - px * ay) / (ax * by - bx * ay)
    let num = ax * py - px * ay;
    let denum = ax * by - bx * ay;
    if num % denum != 0 {
        return None;
    }
    let nb = num / denum;
    let na = (px - nb * bx) / ax;
    Some(3 * na + nb)
}

pub fn part1(input: &str) -> impl Display {
    let machines: Vec<&str> = input.trim().split("\n\n").collect();
    machines
        .into_iter()
        .filter_map(|machine| machine_cost(machine, 0))
        .sum::<i128>()
}

pub fn part2(input: &str) -> impl Display {
    let machines: Vec<&str> = input.trim().split("\n\n").collect();
    machines
        .into_iter()
        .filter_map(|machine| machine_cost(machine, 10000000000000))
        .sum::<i128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day13.txt");
    const INPUT: &str = include_str!("../inputs/day13.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "480");
        assert_eq!(part1(INPUT).to_string(), "29522");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).to_string(), "101214869433312");
    }
}
