from typing import List


def run(program: List[int], *inputs: int) -> List[int]:
    program = list(program)
    for i, v in enumerate(inputs):
        program[i + 1] = v
    for i in range(0, len(program), 4):
        opcode = program[i]
        if opcode == 99:
            return program
        elif opcode == 1:
            a, b, r = program[i + 1], program[i + 2], program[i + 3]
            program[r] = program[a] + program[b]
        elif opcode == 2:
            a, b, r = program[i + 1], program[i + 2], program[i + 3]
            program[r] = program[a] * program[b]
        else:
            assert False
    raise StopIteration


def main() -> None:
    assert run([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]) == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    assert run([1, 0, 0, 0, 99]) == [2, 0, 0, 0, 99]
    assert run([2, 3, 0, 3, 99]) == [2, 3, 0, 6, 99]
    assert run([2, 4, 4, 5, 99, 0]) == [2, 4, 4, 5, 99, 9801]
    assert run([1, 1, 1, 4, 99, 5, 6, 0, 99]) == [30, 1, 1, 4, 2, 5, 6, 0, 99]

    with open('input') as f:
        program = [int(x) for x in f.read().strip().split(',')]

    assert run(program, 12, 2)[0] == 5110675

    for noun in range(100):
        for verb in range(100):
            if run(program, noun, verb)[0] == 19690720:
                print(100 * noun + verb)
                break


if __name__ == '__main__':
    main()
