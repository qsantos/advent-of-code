from typing import List, Tuple

Program = List[Tuple[str, int]]


def read_program() -> List[Tuple[str, int]]:
    program = []
    with open('input') as f:
        for line in f:
            instr, arg = line.split()
            program.append((instr, int(arg)))
    return program


def run(program: Program) -> Tuple[bool, int]:
    ip = 0
    acc = 0
    seen = set()
    while ip < len(program):
        if ip in seen:
            return False, acc
        seen.add(ip)
        instr, arg = program[ip]
        if instr == 'acc':
            acc += arg
            ip += 1
        elif instr == 'jmp':
            ip += arg
        elif instr == 'nop':
            ip += 1
        else:
            assert False
    return True, acc


def main() -> None:
    program = read_program()

    # puzzle 1
    ok, ret = run(program)
    assert not ok
    print(ret)

    # puzzle 2
    for ip in range(len(program)):
        instr, arg = program[ip]
        if instr == 'acc':
            continue
        elif instr == 'jmp':
            program[ip] = 'nop', arg
        elif instr == 'nop':
            program[ip] = 'jmp', arg
        else:
            assert False
        ok, ret = run(program)
        if ok:
            print(ret)
            break
        program[ip] = instr, arg


if __name__ == '__main__':
    main()
