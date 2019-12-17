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


def read_scafold(program: Program) -> Scafold:
    r = ProgramRun(program)
    return ''.join(
        chr(c)
        for c in r
    ).strip().split('\n')


def scafold_alignment(scafold: Scafold) -> int:
    r = 0
    for i in range(1, len(scafold) - 1):
        for j in range(1, len(scafold[0]) - 1):
            if scafold[i][j] != '#':
                continue
            if scafold[i - 1][j] != '#':
                continue
            if scafold[i + 1][j] != '#':
                continue
            if scafold[i][j - 1] != '#':
                continue
            if scafold[i][j + 1] != '#':
                continue
            r += i * j
    return r


def show_scafold(scafold: Scafold, robot_i: int, robot_j: int, robot_d: str) -> None:
    h, w = len(scafold), len(scafold[0])
    for i in range(h):
        print(''.join(
            robot_d if (robot_i, robot_j) == (i, j) else scafold[i][j]
            for j in range(w)
        ))


def follow_path(scafold: Scafold) -> Iterator[str]:
    directions = '<^>v'
    h, w = len(scafold), len(scafold[0])

    for i in range(h):
        for j in range(w):
            if scafold[i][j] in directions:
                break
        else:
            continue
        break
    robot_dir = scafold[i][j]
    assert scafold[i][j] in directions

    if i > 0 and scafold[i - 1][j] == '#':
        start_dir = '^'
    elif i < h - 1 and scafold[i + 1][j] == '#':
        start_dir = 'v'
    elif j > 0 and scafold[i][j - 1] == '#':
        start_dir = '<'
    elif j < w - 1 and scafold[i][j + 1] == '#':
        start_dir = '>'
    else:
        assert False

    rotations = (directions.index(robot_dir) - directions.index(start_dir)) % 4
    if rotations <= 2:
        yield from 'L' * rotations
    else:
        yield 'R'

    steps = 0
    dir = start_dir

    def turn_left() -> Iterator[str]:
        nonlocal dir, steps
        yield str(steps)
        yield 'L'
        steps = 0
        dir = directions[(directions.index(dir) - 1) % 4]

    def turn_right() -> Iterator[str]:
        nonlocal dir, steps
        yield str(steps)
        yield 'R'
        steps = 0
        dir = directions[(directions.index(dir) + 1) % 4]

    while scafold[i][j] != '.':
        if dir == '^':
            if i == 0 or scafold[i - 1][j] == '.':
                if j > 0 and scafold[i][j - 1] == '#':
                    yield from turn_left()
                elif j < w - 1 and scafold[i][j + 1] == '#':
                    yield from turn_right()
                else:
                    break
        elif dir == 'v':
            if i == h - 1 or scafold[i + 1][j] == '.':
                if j > 0 and scafold[i][j - 1] == '#':
                    yield from turn_right()
                elif j < w - 1 and scafold[i][j + 1] == '#':
                    yield from turn_left()
                else:
                    break
        elif dir == '<':
            if j == 0 or scafold[i][j - 1] == '.':
                if i > 0 and scafold[i - 1][j] == '#':
                    yield from turn_right()
                elif i < h - 1 and scafold[i + 1][j] == '#':
                    yield from turn_left()
                else:
                    break
        elif dir == '>':
            if j == w - 1 or scafold[i][j + 1] == '.':
                if i > 0 and scafold[i - 1][j] == '#':
                    yield from turn_left()
                elif i < h - 1 and scafold[i + 1][j] == '#':
                    yield from turn_right()
                else:
                    break
        else:
            assert False

        if dir == '^':
            i -= 1
        elif dir == 'v':
            i += 1
        elif dir == '<':
            j -= 1
        elif dir == '>':
            j += 1
        else:
            assert False

        assert 0 <= i < h
        assert 0 <= j < w
        steps += 1

    yield str(steps)


def run_commands(program: Program, main_routine: Tuple[Tuple[str, ...], ...]) -> int:
    A, B, C = set(main_routine)

    main_code = ','.join(
        'ABC'[(A, B, C).index(sub)]
        for sub in main_routine
    )
    assert len(main_code) <= 20

    parts = [main_code]
    for sub in (A, B, C):
        sub_code = ','.join(sub)
        assert len(sub_code) <= 20
        parts.append(sub_code)

    r = ProgramRun(program)
    r.memory[0] = 2

    code = '\n'.join(parts) + '\nn\n'
    for c in code:
        r.feed(ord(c))

    for last in r:
        pass

    return last


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
    scafold = read_scafold(program)

    # puzzle  1
    assert scafold_alignment(scafold) == 9544

    # puzzle 2
    path = list(follow_path(scafold))
    assert path == [
        'L', '12', 'L', '12', 'R', '12', 'L', '12', 'L', '12', 'R', '12', 'L',
        '8', 'L', '8', 'R', '12', 'L', '8', 'L', '8', 'L', '10', 'R', '8', 'R',
        '12', 'L', '10', 'R', '8', 'R', '12', 'L', '12', 'L', '12', 'R', '12',
        'L', '8', 'L', '8', 'R', '12', 'L', '8', 'L', '8', 'L', '10', 'R', '8',
        'R', '12', 'L', '12', 'L', '12', 'R', '12', 'L', '8', 'L', '8', 'R',
        '12', 'L', '8', 'L', '8',
    ]

    A = ('L', '12', 'L', '12', 'R', '12')
    B = ('L', '8', 'L', '8', 'R', '12', 'L', '8', 'L', '8')
    C = ('L', '10', 'R', '8', 'R', '12')
    main_routine: Tuple[Tuple[str, ...], ...] = (A, A, B, C, C, A, B, C, A, B)
    assert [step for sub in main_routine for step in sub] == path

    assert run_commands(program, main_routine) == 1499679


if __name__ == '__main__':
    main()
