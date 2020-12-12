from typing import Callable, List

Grid = List[List[str]]


def count_adjacent(grid: Grid, row: int, col: int) -> int:
    rows = len(grid)
    cols = len(grid[0])
    return sum(
        grid[nr][nc] == '#'
        for nr in (row - 1, row, row + 1)
        for nc in (col - 1, col, col + 1)
        if rows > nr >= 0 <= nc < cols and (nr, nc) != (row, col)
    )


def count_lineofsight(grid: Grid, row: int, col: int) -> int:
    rows = len(grid)
    cols = len(grid[0])
    count = 0
    for dr in (-1, 0, +1):
        for dc in (-1, 0, +1):
            if dr == dc == 0:
                continue
            i = 1
            while True:
                nr = row + dr * i
                nc = col + dc * i
                if not rows > nr >= 0 <= nc < cols:
                    break
                if grid[nr][nc] == '#':
                    count += 1
                    break
                elif grid[nr][nc] == 'L':
                    break
                i += 1
    return count


def step(
    grid: Grid,
    counter: Callable[[Grid, int, int], int],
    max_neighbors: int
) -> Grid:
    rows = len(grid)
    cols = len(grid[0])

    ret = []
    for row in range(rows):
        r = []
        for col in range(cols):
            if grid[row][col] == '.':
                r.append('.')
                continue
            count = counter(grid, row, col)
            if count == 0:
                r.append('#')
            elif count > max_neighbors:
                r.append('L')
            else:
                r.append(grid[row][col])
        ret.append(r)
    return ret


def count_final(
    grid: Grid,
    counter: Callable[[Grid, int, int], int],
    max_neighbors: int,
) -> int:
    while True:
        new_grid = step(grid, counter, max_neighbors)
        if new_grid == grid:
            return sum(line.count('#') for line in grid)
        grid = new_grid


def main() -> None:
    with open('input') as f:
        grid = [list(line.strip()) for line in f]

    print(count_final(grid, count_adjacent, 3))  # puzzle 1
    print(count_final(grid, count_lineofsight, 4))  # puzzle 2


if __name__ == '__main__':
    main()
