use std::fmt::Display;

#[derive(Debug)]
struct State {
    registers: Vec<u64>,
    program: Vec<u64>,
    ip: usize,
    output: Vec<u64>,
}

impl State {
    fn from_input(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let registers: Vec<u64> = registers
            .lines()
            .map(|line| {
                let (_name, value) = line.split_once(": ").unwrap();
                value.parse().unwrap()
            })
            .collect();
        let (_label, program) = program.split_once(": ").unwrap();
        let program: Vec<u64> = program
            .trim()
            .as_bytes()
            .split(|&b| b == b',')
            .map(|b| (b[0] - b'0') as u64)
            .collect();
        Self {
            registers,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn opcode(&mut self) -> u64 {
        let value = self.program[self.ip];
        self.ip += 1;
        value
    }

    fn literal_operand(&mut self) -> u64 {
        let value = self.program[self.ip];
        self.ip += 1;
        value
    }

    fn combo_operand(&mut self) -> u64 {
        let value = self.program[self.ip];
        self.ip += 1;
        match value {
            0..=3 => value,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => panic!("reserved opcode"),
            _ => panic!("unexpected opcode {}", value),
        }
    }

    fn step(&mut self) {
        match self.opcode() {
            0 => {
                // adv
                let numerator = self.registers[0];
                let shift = self.combo_operand();
                self.registers[0] = if shift > 63 { 0 } else { numerator >> shift };
            }
            1 => {
                // bxl
                let left = self.registers[1];
                let right = self.literal_operand();
                self.registers[1] = left ^ right;
            }
            2 => {
                // bst
                let v = self.combo_operand();
                self.registers[1] = v % 8;
            }
            3 => {
                // jnz
                let t = self.literal_operand();
                if self.registers[0] != 0 {
                    self.ip = t as usize;
                }
            }
            4 => {
                // bxc
                let _ = self.literal_operand();
                let left = self.registers[1];
                let right = self.registers[2];
                self.registers[1] = left ^ right;
            }
            5 => {
                // out
                let v = self.combo_operand() % 8;
                self.output.push(v);
            }
            6 => {
                // bdv
                let numerator = self.registers[0];
                let shift = self.combo_operand();
                self.registers[1] = if shift > 63 { 0 } else { numerator >> shift };
            }
            7 => {
                // cdv
                let numerator = self.registers[0];
                let shift = self.combo_operand();
                self.registers[2] = if shift > 63 { 0 } else { numerator >> shift };
            }
            v => panic!("unexpected opcode {}", v),
        }
    }

    fn evaluate(&mut self, a: u64) {
        self.registers[0] = a;
        self.registers[1] = 0;
        self.registers[2] = 0;
        self.ip = 0;
        self.output.clear();
        while self.ip < self.program.len() {
            self.step();
        }
    }
}

fn format_output(output: &[u64]) -> String {
    output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part1(input: &str) -> impl Display {
    let mut state = State::from_input(input);
    state.evaluate(state.registers[0]);
    format_output(&state.output)
}

pub fn part2_example(_input: &str) -> impl Display {
    let numbers = [0, 3, 5, 4, 3, 0];

    // The example contains the following pairs of numbers, which correspond to the sequence of
    // instructions below:
    // 0,3  A = A >> 3
    // 5,4  output A % 8
    // 3,0  if A != 0 goto 0

    // That means that each tribit of A directly maps to a number of the output. We can easily
    // reconstruct A from the output:
    let mut a = 0;
    for number in numbers.into_iter().rev() {
        a = a << 3 | number;
    }
    // the program _starts_ by shifting A, so we need to shift one more time here
    a << 3
}

pub fn part2(_input: &str) -> impl Display {
    let numbers = [2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 0, 3, 5, 5, 3, 0];

    // My input contains the following pairs of numbers, which correspond to the sequence of
    // instructions below:
    // 2,4  B = A % 8
    // 1,1  B = B ^ 1
    // 7,5  C = A >> B
    // 1,5  B = B ^ 5
    // 4,3  B = B ^ C
    // 0,3  A = A >> 3
    // 5,5  output B % 8
    // 3,0  if A != 0 goto 0

    // This correspond to this pseudo-code, with `round_output` being defined below.
    // ```
    // do {
    //     output round_output(A);
    //     A = A >> 3
    // } while (A != 0);
    // ```
    fn round_output(a: u64) -> u64 {
        let b = a % 8;
        let b = b ^ 1;
        let c = a >> b;
        let b = b ^ 5;
        let b = b ^ c;
        b % 8
    }

    // From this, we can see that there is one tribit in A for each number in the output.
    //
    // To reconstruct A, we start from the most significant tribit, which corresponds to the last
    // number of the program. Only certain values of that tribit will output the required number (0
    // here). We keep all possible values, and move to the previous number of the program.
    //
    // We then iterate for each number in reverse order, trying all possible values of the tribit
    // for each of the candidates of the previous round. We need to keep all the candidates, since
    // some of the candidates might not output a certain number for any tribit we add to it.
    let mut candidates = vec![0];
    let mut new_candidates = Vec::new();
    for number in numbers.into_iter().rev() {
        new_candidates.clear();
        for candidate in candidates.iter().copied() {
            for tribit in 0..8 {
                let new_candidate = candidate << 3 | tribit;
                if round_output(new_candidate) == number {
                    new_candidates.push(new_candidate);
                }
            }
        }
        (candidates, new_candidates) = (new_candidates, candidates);
    }
    // candidates are kept in sorted order, so the smallest possible solution is the first
    // remaining candidate
    candidates[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../examples/day17-1.txt");
    const EXAMPLE2: &str = include_str!("../examples/day17-2.txt");
    const INPUT: &str = include_str!("../inputs/day17.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1).to_string(), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part1(INPUT).to_string(), "7,6,1,5,3,1,4,2,6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_example(EXAMPLE2).to_string(), "117440");
        assert_eq!(part2(INPUT).to_string(), "164541017976509");
    }
}
