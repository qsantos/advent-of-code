from typing import Dict, List, Tuple


Instruction = Tuple[str, str, str]


def read_instructions(filename: str) -> List[Instruction]:
    with open(filename) as f:
        instructions = []
        for line in f:
            opcode, rest = line.strip().split(' ', 1)
            operands = rest.split(', ')
            if len(operands) == 1:
                a, = operands
                b = ''
            elif len(operands) == 2:
                a, b = operands
            else:
                assert False, line.strip()
            instructions.append((opcode, a, b))
        return instructions


def run_program(instructions: List[Instruction], var: str, *, start: int = 0) -> int:
    mem: Dict[str, int] = {'a': start}
    ip = 0
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
        if opcode == 'hlf':
            mem[a] = va // 2
        elif opcode == 'tpl':
            mem[a] = va * 3
        elif opcode == 'inc':
            mem[a] = va + 1
        elif opcode == 'jmp':
            ip += va - 1
        elif opcode == 'jie':
            if va % 2 == 0:
                ip += vb - 1
        elif opcode == 'jio':
            if va == 1:
                ip += vb - 1
        else:
            assert False, opcode
        ip += 1
    return mem.get(var, 0)


example = read_instructions('example')
input = read_instructions('input')

assert run_program(example, 'a') == 2
assert run_program(input, 'b') == 255
assert run_program(input, 'b', start=1) == 334
