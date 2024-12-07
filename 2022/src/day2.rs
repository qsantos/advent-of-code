#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl Move {
    fn from_abc(s: &str) -> Self {
        match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_xyz(s: &str) -> Self {
        match s {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn play(&self, other: Self) -> RoundResult {
        match self {
            Move::Rock => match other {
                Move::Rock => RoundResult::Draw,
                Move::Paper => RoundResult::Lose,
                Move::Scissors => RoundResult::Win,
            },
            Move::Paper => match other {
                Move::Rock => RoundResult::Win,
                Move::Paper => RoundResult::Draw,
                Move::Scissors => RoundResult::Lose,
            },
            Move::Scissors => match other {
                Move::Rock => RoundResult::Lose,
                Move::Paper => RoundResult::Win,
                Move::Scissors => RoundResult::Draw,
            },
        }
    }

    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl RoundResult {
    fn from_xyz(s: &str) -> Self {
        match s {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => unreachable!(),
        }
    }

    fn needed_move(&self, opponent_move: &Move) -> Move {
        match self {
            RoundResult::Win => match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            RoundResult::Draw => opponent_move.clone(),
            RoundResult::Lose => match opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }

    fn score(&self) -> i32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        }
    }
}

fn score_of_file<F>(input: &str, move_selector: F) -> i32
where
    F: Fn(&Move, &str) -> Move,
{
    let mut total_score = 0;
    for line in input.lines() {
        let parts = line.split(' ');
        let parts: Vec<&str> = parts.collect();
        assert!(parts.len() == 2);
        let opponent_move = Move::from_abc(parts[0]);
        let my_move = move_selector(&opponent_move, parts[1]);
        total_score += my_move.score() + my_move.play(opponent_move).score();
    }
    total_score
}

pub fn part1(input: &str) -> i32 {
    let move_selector = |_opponent_move: &Move, xyz: &str| Move::from_xyz(xyz);
    score_of_file(input, move_selector)
}

pub fn part2(input: &str) -> i32 {
    let move_selector =
        |opponent_move: &Move, xyz: &str| RoundResult::from_xyz(xyz).needed_move(opponent_move);
    score_of_file(input, move_selector)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day2.txt");
    const INPUT: &str = include_str!("../inputs/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 15);
        assert_eq!(part1(INPUT), 12855);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 12);
        assert_eq!(part2(INPUT), 13726);
    }
}
