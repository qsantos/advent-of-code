from typing import Tuple

Grid = Tuple[Tuple[str, ...], ...]


def read_map(filename: str) -> Grid:
    with open(filename) as f:
        return tuple(
            tuple(line.strip())
            for line in f
        )


def count_neighbors(grid: Grid, i: int, j: int) -> Tuple[int, int]:
    n_trees = 0
    n_lumberyards = 0
    for ni in (i - 1, i, i + 1):
        for nj in (j - 1, j, j + 1):
            if (ni, nj) != (i, j) and 0 <= ni < len(grid) and 0 <= nj < len(grid[ni]):
                c = grid[ni][nj]
                if c == '|':
                    n_trees += 1
                elif c == '#':
                    n_lumberyards += 1
    return n_trees, n_lumberyards


def print_grid(grid: Grid) -> None:
    for row in grid:
        print(''.join(row))


def next_step(grid: Grid) -> Grid:
    new_grid = []
    for i, row in enumerate(grid):
        new_row = []
        for j, c in enumerate(row):
            n_trees, n_lumberyards = count_neighbors(grid, i, j)
            new_row.append(
                '|' if c == '.' and n_trees >= 3 else
                '#' if c == '|' and n_lumberyards >= 3 else
                '.' if c == '#' and not (n_lumberyards >= 1 and n_trees >= 1) else
                c
            )
        new_grid.append(tuple(new_row))
    return tuple(new_grid)


def run_simulation(grid: Grid, minutes: int, log: bool = False) -> int:
    if log:
        print('Initial state:')
        print_grid(grid)
    last_seen = {grid: 0}
    minute = 0
    while minute < minutes:
        grid = next_step(grid)
        minute += 1
        if grid in last_seen:
            cycle_length = minute - last_seen[grid]
            if log:
                print(f'Cycle of length {cycle_length} detected!')
            minutes = minute + (minutes - minute) % cycle_length
        last_seen[grid] = minute
        if log:
            print()
            print(f'After {minute} minute{"" if minute == 1 else "s"}:')
            print_grid(grid)
    n_trees = sum(c == '|' for row in grid for c in row)
    n_lumberyards = sum(c == '#' for row in grid for c in row)
    return n_trees * n_lumberyards


def main() -> None:
    example = read_map('example')
    input = read_map('input')

    assert run_simulation(example, 10) == 1147
    assert run_simulation(input, 10) == 355918

    assert run_simulation(input, 1_000_000_000) == 202806


if __name__ == '__main__':
    main()
