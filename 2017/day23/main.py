from typing import Dict, List, Tuple

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


def count_muls(instructions: List[Instruction]) -> int:
    mem: Dict[str, int] = {}
    ip = 0
    count_muls = 0
    while 0 <= ip < len(instructions):
        opcode, a, b = instructions[ip]
        try:
            va = int(a)
        except ValueError:
            va = mem.get(a, 0)
        try:
            vb = int(b)
        except ValueError:
            vb = mem.get(b, 0)
        if opcode == 'set':
            mem[a] = vb
        elif opcode == 'sub':
            mem[a] = mem.get(a, 0) - vb
        elif opcode == 'mul':
            mem[a] = mem.get(a, 0) * vb
            count_muls += 1
        elif opcode == 'jnz':
            if va != 0:
                ip += vb - 1
        else:
            assert False, opcode
        ip += 1
    return count_muls


def run_program(a: int) -> int:
    if a == 0:
        b = 93
        c = b
    else:
        b = 109_300
        c = 126_300
    h = 0
    while b <= c:
        if any(b % d == 0 for d in range(2, b)):
            h += 1
        b += 17
    return h


def main() -> None:
    input = read_instructions('input')

    # debug
    assert count_muls(input) == 8281

    # non-debug
    assert run_program(0) == 1
    assert run_program(1) == 911


if __name__ == '__main__':
    main()
