from collections import defaultdict, deque
from typing import (
    DefaultDict, Deque, Dict, Iterator, List, Optional, Set, Tuple,
)

Grid = Dict[Tuple[int, int], str]


def read_program(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(x) for x in f.read().strip().split(',')]


class AwaitingInput(Exception):
    pass


class ProgramRun(Iterator[int]):
    def __init__(self, program: List[int]):
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


def run(program: List[int], *inputs: int) -> List[int]:
    r = ProgramRun(program)
    for input in inputs:
        r.feed(input)
    return list(r)


def print_grid(grid: Grid, droid: Optional[Tuple[int, int]] = None) -> None:
    min_i = min(i for i, j in grid)
    max_i = max(i for i, j in grid)
    min_j = min(j for i, j in grid)
    max_j = max(j for i, j in grid)
    for i in range(min_i, max_i + 1):
        print(''.join(
            'D' if (i, j) == droid else grid.get((i, j), ' ')
            for j in range(min_j, max_j + 1)
        ))


def clear_screen() -> None:
    print('\x1b[2J\x1b[H', end='')


def getch() -> str:
    import sys
    import termios
    import tty
    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setraw(sys.stdin.fileno())
        ch = sys.stdin.read(1)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
    return ch


def explore_grid_interactive(program: List[int]) -> None:
    r = ProgramRun(program)
    grid: DefaultDict[Tuple[int, int], str] = defaultdict(str)
    i, j = 0, 0
    grid[i, j] = '.'
    while True:
        clear_screen()
        print_grid(grid, (i, j))
        prev_ij = i, j
        c = getch()
        if c == 'q':
            break
        elif c == '\x1b' and getch() == '[':
            while True:
                c = getch()
                if c == 'A':  # up
                    r.feed(1)
                    i -= 1
                elif c == 'B':  # down
                    r.feed(2)
                    i += 1
                elif c == 'C':  # right
                    r.feed(4)
                    j += 1
                elif c == 'D':  # left
                    r.feed(3)
                    j -= 1
                else:
                    continue
                break
            status = next(r)
            if status == 0:
                grid[i, j] = '#'
                i, j = prev_ij
            elif status == 1:
                grid[i, j] = '.'
            elif status == 2:
                grid[i, j] = 'O'
            else:
                assert False
    clear_screen()
    print_grid(grid, (i, j))


def explore_grid(program: List[int]) -> Grid:
    r = ProgramRun(program)
    grid: DefaultDict[Tuple[int, int], str] = defaultdict(str)
    i, j = 0, 0
    grid[i, j] = '.'
    steps: List[Tuple[Optional[int], List[int]]] = [(None, [1, 2, 3, 4])]
    while True:
        prev_ij = i, j

        source_direction, candidate_directions = steps[-1]

        if not candidate_directions:
            steps.pop()
            if source_direction is None:
                break
            elif source_direction == 1:  # north
                r.feed(2)
                i += 1
            elif source_direction == 2:  # south
                r.feed(1)
                i -= 1
            elif source_direction == 3:  # west
                r.feed(4)
                j += 1
            elif source_direction == 4:  # east
                r.feed(3)
                j -= 1
            else:
                assert False
            assert grid[i, j] in '.O', grid[i, j]
            assert next(r) == 1
            continue

        direction = candidate_directions.pop()

        if direction == 1:  # north
            i -= 1
        elif direction == 2:  # south
            i += 1
        elif direction == 3:  # west
            j -= 1
        elif direction == 4:  # east
            j += 1
        else:
            assert False

        if (i, j) in grid:
            i, j = prev_ij
            continue

        r.feed(direction)

        status = next(r)

        if status == 0:
            grid[i, j] = '#'
            i, j = prev_ij
        elif status == 1:
            grid[i, j] = '.'
            steps.append((direction, [1, 2, 3, 4]))
        elif status == 2:
            grid[i, j] = 'O'
            steps.append((direction, [1, 2, 3, 4]))
        else:
            assert False

    return dict(grid)


def find_oxygen(grid: Grid) -> int:
    q: Deque[Tuple[int, Tuple[int, int]]] = deque()
    visited: Set[Tuple[int, int]] = set()
    q.append((0, (0, 0)))
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        if grid[state] == 'O':
            return steps
        assert grid[state] == '.'

        i, j = state
        for neighbor in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if grid[neighbor] != '#':
                q.append((steps + 1, neighbor))

    return -1


def fill_oxygen(grid: Grid) -> int:
    oxygen_tank = next(state for state in grid if grid[state] == 'O')
    q: Deque[Tuple[int, Tuple[int, int]]] = deque()
    visited: Set[Tuple[int, int]] = set()
    q.append((0, oxygen_tank))
    max_steps = 0
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        if steps > max_steps:
            max_steps = steps

        assert grid[state] in 'O.'

        i, j = state
        for neighbor in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if grid[neighbor] != '#':
                q.append((steps + 1, neighbor))

    return max_steps


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
    grid = explore_grid(program)
    print_grid(grid)

    # puzzle 1
    assert find_oxygen(grid) == 230

    # puzzle 2
    assert fill_oxygen(grid) == 288


if __name__ == '__main__':
    main()
