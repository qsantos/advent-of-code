from collections import defaultdict
from typing import DefaultDict, Tuple

Grid2D = DefaultDict[Tuple[int, int, int], str]
Grid3D = DefaultDict[Tuple[int, int, int, int], str]


def step_2d(grid: Grid2D) -> Grid2D:
    new_grid: Grid2D = defaultdict(lambda: '.')
    min_x = min(x for x, y, z in grid)
    max_x = max(x for x, y, z in grid)
    min_y = min(y for x, y, z in grid)
    max_y = max(y for x, y, z in grid)
    min_z = min(z for x, y, z in grid)
    max_z = max(z for x, y, z in grid)
    for x in range(min_x - 1, max_x + 2):
        for y in range(min_y - 1, max_y + 2):
            for z in range(min_z - 1, max_z + 2):
                count = sum(
                    grid[nx, ny, nz] == '#'
                    for nx in (x - 1, x, x + 1)
                    for ny in (y - 1, y, y + 1)
                    for nz in (z - 1, z, z + 1)
                    if (nx, ny, nz) != (x, y, z)
                )
                if count == 2 and grid[x, y, z] == '#' or count == 3:
                    new_grid[x, y, z] = '#'
    return new_grid


def puzzle1() -> None:
    grid: Grid2D = defaultdict(lambda: '.')
    with open('input') as f:
        for x, line in enumerate(f):
            for y, ny in enumerate(line.strip()):
                grid[x, y, 0] = ny

    for _ in range(6):
        grid = step_2d(grid)
    print(sum(v == '#' for v in grid.values()))


def step_3d(grid: Grid3D) -> Grid3D:
    new_grid: Grid3D = defaultdict(lambda: '.')
    min_x = min(x for x, y, z, w in grid)
    max_x = max(x for x, y, z, w in grid)
    min_y = min(y for x, y, z, w in grid)
    max_y = max(y for x, y, z, w in grid)
    min_z = min(z for x, y, z, w in grid)
    max_z = max(z for x, y, z, w in grid)
    min_w = min(w for x, y, z, w in grid)
    max_w = max(w for x, y, z, w in grid)
    for x in range(min_x - 1, max_x + 2):
        for y in range(min_y - 1, max_y + 2):
            for z in range(min_z - 1, max_z + 2):
                for w in range(min_w - 1, max_w + 2):
                    count = sum(
                        grid[nx, ny, nz, nw] == '#'
                        for nx in (x - 1, x, x + 1)
                        for ny in (y - 1, y, y + 1)
                        for nz in (z - 1, z, z + 1)
                        for nw in (w - 1, w, w + 1)
                        if (nx, ny, nz, nw) != (x, y, z, w)
                    )
                    if count == 2 and grid[x, y, z, w] == '#' or count == 3:
                        new_grid[x, y, z, w] = '#'
    return new_grid


def puzzle2() -> None:
    grid: Grid3D = defaultdict(lambda: '.')
    with open('input') as f:
        for x, line in enumerate(f):
            for y, ny in enumerate(line.strip()):
                grid[x, y, 0, 0] = ny

    for _ in range(6):
        grid = step_3d(grid)
    print(sum(v == '#' for v in grid.values()))


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
