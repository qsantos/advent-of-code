from collections import defaultdict, deque
from typing import DefaultDict, Deque, Dict, Iterator, List, Tuple

Program = List[int]
Grid = Dict[Tuple[int, int], str]
Scafold = List[str]


def read_program(filename: str) -> Program:
    with open(filename) as f:
        return [int(x) for x in f.read().strip().split(',')]


class AwaitingInput(Exception):
    pass


class ProgramRun(Iterator[int]):
    def __init__(self, program: Program):
        self.inputs: Deque[int] = deque()
        self.outputs: Deque[int] = deque()
        self.memory: DefaultDict[int, int] = defaultdict(int)
        for i, v in enumerate(program):
            self.memory[i] = v
        self.relative_base = 0
        self.i = 0
        self.awaiting_input = False
        self.awaiting_input_param_modes = 0

    def feed(self, v: int) -> None:
        self.inputs.append(v)

    def __iter__(self) -> Iterator[int]:
        return self

    def __next__(self) -> int:
        while not self.outputs:
            self.step()
        return self.outputs.popleft()

    def step(self) -> None:
        assert self.i < len(self.memory)
        param_modes = 0

        def read_input() -> None:
            try:
                v = self.inputs.popleft()
            except IndexError:
                self.awaiting_input = True
                raise AwaitingInput from None
            else:
                self.awaiting_input = False
                write_param(v)

        def read_param() -> int:
            nonlocal param_modes
            v = self.memory[self.i]
            self.i += 1
            param_modes, param_mode = divmod(param_modes, 10)
            if param_mode == 0:
                return self.memory[v]
            elif param_mode == 1:
                return v
            elif param_mode == 2:
                return self.memory[self.relative_base + v]
            else:
                assert False

        def write_param(a: int) -> None:
            nonlocal param_modes
            v = self.memory[self.i]
            self.i += 1
            param_modes, param_mode = divmod(param_modes, 10)
            if param_mode == 0:
                self.memory[v] = a
            elif param_mode == 1:
                assert False
            elif param_mode == 2:
                self.memory[self.relative_base + v] = a
            else:
                assert False

        if self.awaiting_input:
            param_modes = self.awaiting_input_param_modes
            read_input()
            return

        param_modes, opcode = divmod(self.memory[self.i], 100)

        self.i += 1
        if opcode == 99:
            raise StopIteration
        elif opcode == 1:
            write_param(read_param() + read_param())
        elif opcode == 2:
            write_param(read_param() * read_param())
        elif opcode == 3:
            self.awaiting_input_param_modes = param_modes
            read_input()
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
        elif opcode == 9:
            self.relative_base += read_param()
        else:
            print(opcode)
            return
            assert False, opcode


def run(program: Program, *inputs: int) -> List[int]:
    r = ProgramRun(program)
    for input in inputs:
        r.feed(input)
    return list(r)


def evaluate(program: Program, i: int, j: int) -> int:
    r = ProgramRun(program)
    r.feed(i)
    r.feed(j)
    return next(r)


def get_beam(program: Program, size: int) -> List[str]:
    rows = []
    for i in range(size):
        rows.append(''.join(
            '.#'[evaluate(program, i, j)]
            for j in range(size)
        ))
    return rows


def count_affected(program: Program, size: int) -> int:
    return sum(
        evaluate(program, i, j)
        for i in range(size)
        for j in range(size)
    )


def find_square(program: Program, size: int) -> Tuple[int, int]:
    square: Dict[Tuple[int, int], int] = {}

    j = 0
    while True:
        for i in range(j):
            if i > 30 and (i - 1, j) not in square and (i, j - 1) not in square:
                continue

            if not evaluate(program, i, j):
                continue

            new_square = min(
                square.get((i, j - 1), 0),
                square.get((i - 1, j), 0),
                square.get((i - 1, j - 1), 0),
            ) + 1

            if new_square >= 100:
                return i - size + 1, j - size + 1

            square[i, j] = new_square

        j += 1


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

    program = [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    assert run(program) == program
    assert run([1102, 34915192, 34915192, 7, 4, 7, 99, 0]) == [1219070632396864]
    assert run([104, 1125899906842624, 99]) == [1125899906842624]

    program = read_program('input')

    for row in get_beam(program, 30):
        print(row)

    # puzzle 1
    assert count_affected(program, 50) == 110

    # puzzle 2
    assert find_square(program, 100) == (1730, 2065)


if __name__ == '__main__':
    main()
