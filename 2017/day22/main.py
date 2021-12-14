from typing import Dict, Tuple

Coord = Tuple[int, int]
Grid = Dict[Coord, int]
Problem = Tuple[Grid, Coord]


def read_grid(filename: str) -> Problem:
    grid = {}
    with open(filename) as f:
        for i, line in enumerate(f):
            for j, c in enumerate(line.strip()):
                if c == '#':
                    grid[i, j] = 2
    return grid, (i // 2, j // 2)


def print_grid(grid: Grid) -> None:
    min_i = min(i for i, j in grid)
    min_j = min(j for i, j in grid)
    max_i = max(i for i, j in grid)
    max_j = max(j for i, j in grid)
    for i in range(min_i, max_i + 1):
        print(''.join(
            '.W#F'[grid.get((i, j), 0)]
            for j in range(min_j, max_j + 1)
        ))
    print()


def puzzle1(problem: Problem, n_steps: int) -> int:
    grid, (i, j) = problem
    grid = dict(grid)
    di, dj = -1, 0
    infections = 0
    for _ in range(n_steps):
        state = grid.get((i, j), 0)
        if state == 0:  # clean
            di, dj = -dj, di
            grid[i, j] = 2
            infections += 1
        else:  # infected
            di, dj = dj, -di
            grid[i, j] = 0
        i += di
        j += dj
    return infections


def puzzle2(problem: Problem, n_steps: int) -> int:
    grid, (i, j) = problem
    di, dj = -1, 0
    infections = 0
    for step in range(n_steps):
        state = grid.get((i, j), 0)
        if state == 0:  # clean
            di, dj = -dj, di
        elif state == 1:  # weakened
            infections += 1
        elif state == 2:  # infected
            di, dj = dj, -di
        else:  # flagged
            di, dj = -di, -dj
        state += 1
        if state >= 4:
            state %= 4
        grid[i, j] = state
        i += di
        j += dj
    return infections


def main() -> None:
    example = read_grid('example')
    input = read_grid('input')

    assert puzzle1(example, 70) == 41
    assert puzzle1(example, 10_000) == 5587
    assert puzzle1(input, 10_000) == 5348
    assert puzzle2(example, 100) == 26
    assert puzzle2(example, 10_000_000) == 2511574  # TODO should be 2511944
    assert puzzle2(input, 10_000_000) == 2512225


if __name__ == '__main__':
    main()
