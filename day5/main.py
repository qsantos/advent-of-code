from typing import List


def run(program: List[int], *inputs: int) -> List[int]:
    program = list(program)
    i = 0
    iter_inputs = iter(inputs)
    outputs: List[int] = []
    param_modes = 0

    def read_param() -> int:
        nonlocal i
        nonlocal param_modes
        v = program[i]
        i += 1
        param_modes, param_mode = divmod(param_modes, 10)
        if param_mode == 0:
            return program[v]
        elif param_mode == 1:
            return v
        else:
            assert False

    def write_param(a: int) -> None:
        nonlocal i
        nonlocal param_modes
        v = program[i]
        i += 1
        param_modes, param_mode = divmod(param_modes, 10)
        assert param_mode == 0
        program[v] = a

    while True:
        assert i < len(program)
        param_modes, opcode = divmod(program[i], 100)
        i += 1
        if opcode == 99:
            return outputs
        elif opcode == 1:
            write_param(read_param() + read_param())
        elif opcode == 2:
            write_param(read_param() * read_param())
        elif opcode == 3:
            write_param(next(iter_inputs))
        elif opcode == 4:
            outputs.append(read_param())
        elif opcode == 5:
            c = read_param()
            t = read_param()
            if c:
                i = t
        elif opcode == 6:
            c = read_param()
            t = read_param()
            if not c:
                i = t
        elif opcode == 7:
            write_param(1 if read_param() < read_param() else 0)
        elif opcode == 8:
            write_param(1 if read_param() == read_param() else 0)
        else:
            assert False
    raise StopIteration


def main() -> None:
    assert run([104, 42, 99]) == [42]
    assert run([3, 5, 4, 5, 99, 0], 42) == [42]
    assert run([3, 15, 3, 16, 1, 15, 16, 15, 1002, 15, 2, 15, 4, 15, 99, 0, 0], 10, 20) == [60]

    assert run([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8) == [1]
    assert run([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7) == [0]
    assert run([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7) == [1]
    assert run([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9) == [0]

    assert run([3, 3, 1108, -1, 8, 3, 4, 3, 99], 8) == [1]
    assert run([3, 3, 1108, -1, 8, 3, 4, 3, 99], 7) == [0]
    assert run([3, 3, 1107, -1, 8, 3, 4, 3, 99], 7) == [1]
    assert run([3, 3, 1107, -1, 8, 3, 4, 3, 99], 9) == [0]

    assert run([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 0) == [0]
    assert run([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 4) == [1]
    assert run([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0) == [0]
    assert run([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 5) == [1]

    program = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
        1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
        999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
    ]
    assert run(program, 7) == [999]
    assert run(program, 8) == [1000]
    assert run(program, 9) == [1001]

    with open('input') as f:
        program = [int(x) for x in f.read().strip().split(',')]

    assert run(program, 1) == [0, 0, 0, 0, 0, 0, 0, 0, 0, 5577461]
    print(run(program, 5))


if __name__ == '__main__':
    main()
