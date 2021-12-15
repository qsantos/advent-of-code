from typing import List, Tuple

Instruction = Tuple[str, str, str]


def read_instructions(filename: str) -> List[Instruction]:
    instructions = []
    with open(filename) as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) == 2:
                opcode, a = parts
                b = ''
            else:
                opcode, a, b = parts
            instructions.append((opcode, a, b))
    return instructions


def run_instructions(instructions: List[Instruction], c: int = 0) -> int:
    mem = {'c': c}
    ip = 0
    while 0 <= ip < len(instructions):
        opcode, a, b = instructions[ip]
        print(ip, opcode, a, b)
        try:
            va = int(a)
        except ValueError:
            va = mem.get(a, 0)
        try:
            vb = int(b)
        except ValueError:
            vb = mem.get(b, 0)
        if opcode == 'cpy':
            mem[b] = va
        elif opcode == 'inc':
            mem[a] = va + 1
        elif opcode == 'dec':
            mem[a] = va - 1
        elif opcode == 'jnz':
            if va != 0:
                ip += vb - 1
        else:
            assert False, opcode
        ip += 1
        print(mem)
    return mem['a']


def run_program(c: int) -> int:
    a, b, d = 1, 1, 26
    if c != 0:
        d += 7
    for _ in range(d):
        a, b = a + b, a
    a += 11 * 18
    return a


def main() -> None:
    example = read_instructions('example')
    input = read_instructions('input')

    assert run_instructions(example) == 42
    assert run_instructions(input) == 318009

    assert run_program(0) == 318009
    assert run_program(1) == 9227663


if __name__ == '__main__':
    main()
