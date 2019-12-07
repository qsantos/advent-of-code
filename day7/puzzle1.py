from itertools import permutations
from typing import Iterable, Iterator, List


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


def run_sequence(program: List[int], phases: Iterable[int]) -> int:
    signal = 0
    for phase in phases:
        (signal,) = run(program, phase, signal)
    return signal


def run_sequences(program: List[int]) -> Iterator[int]:
    for phases in permutations(range(5)):
        yield run_sequence(program, phases)


def run_best_sequence(program: List[int]) -> int:
    return max(run_sequences(program))


def main() -> None:
    assert run_sequence(
        [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
        [4, 3, 2, 1, 0],
    ) == 43210
    assert run_sequence(
        [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23,
            1, 24, 23, 23, 4, 23, 99, 0, 0,
        ],
        [0, 1, 2, 3, 4],
    ) == 54321
    assert run_sequence(
        [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
            1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ],
        [1, 0, 4, 3, 2],
    ) == 65210

    with open('input') as f:
        program = [int(x) for x in f.read().strip().split(',')]

    print(run_best_sequence(program))


if __name__ == '__main__':
    main()
