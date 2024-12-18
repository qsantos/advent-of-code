use std::collections::HashSet;
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

pub fn part2(input: &str) -> impl Display {
    let mut state = State::from_input(input);
    // number of numbers in the output that should match the program
    let mut matching_output_prefix_length = 4;
    // numbers of least-significant bits that are restricted to the set of candidates
    let mut set_bits_count = 1;
    // candidates for set least-significant bits
    let mut set_bits_candidates = HashSet::new();
    set_bits_candidates.insert(0u64);
    set_bits_candidates.insert(1u64);
    loop {
        let mut candidates = HashSet::new();
        let mut a = 0;
        while candidates.len() < 100 {
            for suffix in set_bits_candidates.iter() {
                let candidate = (a << set_bits_count) | suffix;
                state.evaluate(a);
                if state.output.len() < matching_output_prefix_length {
                    continue;
                }
                let output_prefix = &state.output[..matching_output_prefix_length];
                let program_prefix = &state.program[..matching_output_prefix_length];
                if output_prefix != program_prefix {
                    continue;
                }
                candidates.insert(candidate);
            }
            a += 1;
        }
        set_bits_candidates = candidates
            .iter()
            .map(|c| c & ((1 << set_bits_count) - 1))
            .collect();
        loop {
            set_bits_count += 1;
            let new_set_bits_candidates: HashSet<_> = candidates
                .iter()
                .map(|c| c & ((1 << set_bits_count) - 1))
                .collect();
            for candidate in &new_set_bits_candidates {
                print!("{:20b}", candidate);
            }
            println!();
            if new_set_bits_candidates.len() > 5 {
                set_bits_count -= 1;
                break;
            }
            matching_output_prefix_length += 1;
            set_bits_candidates = new_set_bits_candidates;
        }
        break;
    }
    panic!("");

    let ends: Vec<u64> = vec![0, 1];
    let ends: Vec<u64> = vec![0b10110101, 0b10111101];
    let ends: Vec<u64> = vec![
        0b10100100111100110110101,
        0b11011010110111010111101,
        0b11011010110111110111101,
    ];
    let n = ends.iter().max().unwrap().ilog2() + 1;

    let prefix = 14;

    let mut candidates = HashSet::new();
    let mut a = 0;
    loop {
        for end in ends {
            let candidate = a << n | end;
            state.evaluate(candidate);
            if state.output == state.program {
                return a;
            }
            if state.output.len() >= prefix && state.output[..prefix] == state.program[..prefix] {
                candidates.insert(candidate);
            }
        }
        a += 1;
    }
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
        //assert_eq!(part2(EXAMPLE2).to_string(), "117440");
        assert_eq!(part2(INPUT).to_string(), "164541017976509");
    }
}

/*
2,4 B = A % 8
1,1 B = B ^ 1
7,5 C = A >> B
1,5 B = B ^ 5
4,3 B = B ^ C
0,3 A = A >> 3
5,5 output B % 8
3,0 if A != 0 goto 0

do {
    B = (A % 8) ^ 1
    output B ^ 5 ^ (A >> B) % 8
    A = A >> 3
} while (A != 0);
*/
