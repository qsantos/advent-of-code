from collections import defaultdict
from typing import DefaultDict, Set, Tuple

Coord = Tuple[int, int, int]
State = Set[Coord]
SIZE = 5
MIDDLE = 2


def read_state(filename: str) -> State:
    ret: State = set()
    with open(filename) as f:
        for i, row in enumerate(f):
            for j, c in enumerate(row.strip()):
                if c == '#':
                    ret.add((0, i, j))
    return ret


def step(state: State) -> State:
    counts: DefaultDict[Coord, int] = defaultdict(int)
    for depth, i, j in state:
        # same level
        for a, b in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if 0 <= a < SIZE and 0 <= b < SIZE and (a, b) != (MIDDLE, MIDDLE):
                counts[depth, a, b] += 1

        # exterior border
        if i == 0:
            counts[depth - 1, MIDDLE - 1, MIDDLE] += 1
        if i == SIZE - 1:
            counts[depth - 1, MIDDLE + 1, MIDDLE] += 1
        if j == 0:
            counts[depth - 1, MIDDLE, MIDDLE - 1] += 1
        if j == SIZE - 1:
            counts[depth - 1, MIDDLE, MIDDLE + 1] += 1

        # interior border
        if (i, j) == (MIDDLE - 1, MIDDLE):
            for k in range(SIZE):
                counts[depth + 1, 0, k] += 1
        if (i, j) == (MIDDLE + 1, MIDDLE):
            for k in range(SIZE):
                counts[depth + 1, SIZE - 1, k] += 1
        if (i, j) == (MIDDLE, MIDDLE - 1):
            for k in range(SIZE):
                counts[depth + 1, k, 0] += 1
        if (i, j) == (MIDDLE, MIDDLE + 1):
            for k in range(SIZE):
                counts[depth + 1, k, SIZE - 1] += 1

    ret: State = set()
    for depth, i, j in counts:
        c = counts[depth, i, j]
        if (depth, i, j) in state:
            if c == 1:
                ret.add((depth, i, j))
        else:
            if c in (1, 2):
                ret.add((depth, i, j))

    return ret


def print_state(state: State) -> None:
    depths = {depth for depth, _, _ in state}
    for depth in sorted(depths):
        print(f'Depth {depth}')
        for i in range(SIZE):
            print(''.join(
                '.#'[(depth, i, j) in state]
                for j in range(SIZE)
            ))
        print()


def count_bugs(state: State) -> int:
    return len(state)


def main() -> None:
    state = read_state('example1')
    for _ in range(10):
        state = step(state)
    assert count_bugs(state) == 99

    state = read_state('input')
    for _ in range(200):
        state = step(state)
    assert count_bugs(state) == 1932


if __name__ == '__main__':
    main()
