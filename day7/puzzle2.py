from collections import deque
from itertools import permutations
from typing import Deque, Iterable, Iterator, List, Sequence


class ProgramRun(Iterator[int]):
    def __init__(self, program: List[int]):
        self.inputs: Deque[int] = deque()
        self.outputs: Deque[int] = deque()
        self.program = list(program)
        self.i = 0

    def feed(self, v: int) -> None:
        self.inputs.append(v)

    def __iter__(self) -> Iterator[int]:
        return self

    def __next__(self) -> int:
        while not self.outputs:
            self.step()
        return self.outputs.popleft()

    def step(self) -> None:
        assert self.i < len(self.program)
        param_modes, opcode = divmod(self.program[self.i], 100)

        def read_param() -> int:
            nonlocal param_modes
            v = self.program[self.i]
            self.i += 1
            param_modes, param_mode = divmod(param_modes, 10)
            if param_mode == 0:
                return self.program[v]
            elif param_mode == 1:
                return v
            else:
                assert False

        def write_param(a: int) -> None:
            nonlocal param_modes
            v = self.program[self.i]
            self.i += 1
            param_modes, param_mode = divmod(param_modes, 10)
            assert param_mode == 0
            self.program[v] = a

        self.i += 1
        if opcode == 99:
            raise StopIteration
        elif opcode == 1:
            write_param(read_param() + read_param())
        elif opcode == 2:
            write_param(read_param() * read_param())
        elif opcode == 3:
            write_param(self.inputs.popleft())
        elif opcode == 4:
            self.outputs.append(read_param())
        elif opcode == 5:
            c = read_param()
            t = read_param()
            if c:
                self.i = t
        elif opcode == 6:
            c = read_param()
            t = read_param()
            if not c:
                self.i = t
        elif opcode == 7:
            write_param(1 if read_param() < read_param() else 0)
        elif opcode == 8:
            write_param(1 if read_param() == read_param() else 0)
        else:
            assert False


def run(program: List[int], *inputs: int) -> List[int]:
    r = ProgramRun(program)
    for input in inputs:
        r.feed(input)
    return list(r)


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


def run_loop(program: List[int], phases: Sequence[int]) -> int:
    ps = [ProgramRun(program) for _ in range(len(phases))]
    for p, phase in zip(ps, phases):
        p.feed(phase)

    signal = 0
    while True:
        for p in ps:
            p.feed(signal)
            try:
                signal = next(p)
            except StopIteration:
                return signal


def run_loops(program: List[int]) -> Iterator[int]:
    for phases in permutations(range(5, 10)):
        yield run_loop(program, phases)


def run_best_loop(program: List[int]) -> int:
    return max(run_loops(program))


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

    # first puzzle
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

    # second puzzle

    assert run_loop(
        [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
            27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
        ],
        [9, 8, 7, 6, 5],
    ) == 139629729
    assert run_loop(
        [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55,
            1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008,
            54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56,
            1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ],
        [9, 7, 8, 5, 6],
    ) == 18216
    print(run_best_loop(program))


if __name__ == '__main__':
    main()
