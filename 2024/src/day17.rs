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

    fn combo_operand(&mut self) -> u64{
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
                println!("A = {} >> {} = {}", numerator, shift, self.registers[0]);
            },
            1 => {
                // bxl
                let left = self.registers[1];
                let right = self.literal_operand();
                self.registers[1] = left ^ right;
                println!("bxl: B = {} ^ {} = {}", left, right, left ^ right);
            },
            2 => {
                // bst
                let v = self.combo_operand();
                self.registers[1] = v % 8;
                println!("bst: C = {} % 8 = {}", v, v % 8);
            },
            3 => {
                // jnz
                let t = self.literal_operand();
                if self.registers[0] != 0 {
                    self.ip = t as usize;
                }
                println!("jnz: jump to {}", t);
            },
            4 => {
                // bxc
                let _ = self.literal_operand();
                let left = self.registers[1];
                let right = self.registers[2];
                self.registers[1] = left ^ right;
                println!("bxc: B = {} ^ {} = {}", left, right, left ^ right);
            },
            5 => {
                // out
                let v = self.combo_operand() % 8;
                self.output.push(v);
                println!("out: output {}", v);
            },
            6 => {
                // bdv
                let numerator = self.registers[0];
                let shift = self.combo_operand();
                self.registers[1] = if shift > 63 { 0 } else { numerator >> shift };
                println!("bdv: B = {} >> {} = {}", numerator, shift, self.registers[1]);
            },
            7 => {
                // cdv
                let numerator = self.registers[0];
                let shift = self.combo_operand();
                self.registers[2] = if shift > 63 { 0 } else { numerator >> shift };
                println!("cdv: C = {} >> {} = {}", numerator, shift, self.registers[2]);
            },
            v => panic!("unexpected opcode {}", v),
        }
    }
}

pub fn part1(input: &str) -> impl Display {
    let mut state = State::from_input(input);
    while state.ip < state.program.len() {
        println!("{:?}", state);
        state.step();
    }
    state.output.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day17.txt");
    const INPUT: &str = include_str!("../inputs/day17.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).to_string(), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part1(INPUT).to_string(), "7,6,1,5,3,1,4,2,6");
    }
}
