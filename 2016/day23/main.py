from math import factorial
from typing import List, Set, Tuple

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


def run_instructions(instructions: List[Instruction], initial_value: int = 0) -> int:
    mem = {'a': initial_value}
    ip = 0
    toggled_ips: Set[int] = set()
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

        if ip in toggled_ips:
            opcode = {
                'cpy': 'jnz',
                'inc': 'dec',
                'dec': 'inc',
                'jnz': 'cpy',
                'tgl': 'inc',
            }[opcode]

        if opcode == 'cpy':
            mem[b] = va
        elif opcode == 'inc':
            mem[a] = va + 1
        elif opcode == 'dec':
            mem[a] = va - 1
        elif opcode == 'jnz':
            if va != 0:
                ip += vb - 1
        elif opcode == 'tgl':
            target = ip + va
            if target in toggled_ips:
                toggled_ips.remove(target)
            else:
                toggled_ips.add(target)
        else:
            assert False, opcode
        ip += 1
    return mem['a']


def run_program(a: int) -> int:
    return factorial(a) + 75 * 85


def main() -> None:
    example = read_instructions('example')
    input = read_instructions('input')

    assert run_instructions(example) == 3
    assert run_instructions(input, 7) == 11415

    assert run_program(7) == 11415
    assert run_program(12) == 479007975


if __name__ == '__main__':
    main()
