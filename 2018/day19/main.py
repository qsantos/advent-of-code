from typing import List, Tuple

Instruction = Tuple[str, int, int, int]
Program = Tuple[int, List[Instruction]]


def read_program(filename: str) -> Program:
    with open(filename) as f:
        line = next(f)
        assert line.startswith('#ip ')
        ip_register = int(line.strip()[len('#ip '):])
        instructions = []
        for line in f:
            opcode, a, b, c = line.strip().split()
            instructions.append((opcode, int(a), int(b), int(c)))
    return ip_register, instructions


def run_opcode_by_name(mem: List[int], name: str, a: int, b: int, c: int) -> None:
    if name == 'addr':
        mem[c] = mem[a] + mem[b]
    elif name == 'addi':
        mem[c] = mem[a] + b
    elif name == 'mulr':
        mem[c] = mem[a] * mem[b]
    elif name == 'muli':
        mem[c] = mem[a] * b
    elif name == 'banr':
        mem[c] = mem[a] & mem[b]
    elif name == 'bani':
        mem[c] = mem[a] & b
    elif name == 'borr':
        mem[c] = mem[a] | mem[b]
    elif name == 'bori':
        mem[c] = mem[a] | b
    elif name == 'setr':
        mem[c] = mem[a]
    elif name == 'seti':
        mem[c] = a
    elif name == 'gtir':
        mem[c] = 1 if a > mem[b] else 0
    elif name == 'gtri':
        mem[c] = 1 if mem[a] > b else 0
    elif name == 'gtrr':
        mem[c] = 1 if mem[a] > mem[b] else 0
    elif name == 'eqir':
        mem[c] = 1 if a == mem[b] else 0
    elif name == 'eqri':
        mem[c] = 1 if mem[a] == b else 0
    elif name == 'eqrr':
        mem[c] = 1 if mem[a] == mem[b] else 0
    else:
        assert False, name


def run_program(program: Program, input: int) -> int:
    ip_register, instructions = program
    mem = [input, 0, 0, 0, 0, 0]
    while 0 <= mem[ip_register] < len(instructions):
        ip = mem[ip_register]
        opcode, a, b, c = instructions[ip]
        run_opcode_by_name(mem, opcode, a, b, c)
        mem[ip_register] += 1
    mem[ip_register] -= 1
    return mem[0]


def sum_divisors(n: int) -> int:
    return sum(
        d
        for d in range(1, n + 1)
        if n % d == 0
    )

def main() -> None:
    example = read_program('example')
    input = read_program('input')

    assert run_program(example, 0) == 6
    assert run_program(input, 0) == 1140

    # print(run_program(input, 1))
    print(sum_divisors(10551331))


if __name__ == '__main__':
    main()
