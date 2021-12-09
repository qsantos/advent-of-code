from typing import Iterator, List, Tuple


Grid = List[List[int]]


def read_grid(filename: str) -> Grid:
    with open(filename) as f:
        return [
            [int(c) for c in line.strip()]
            for line in f
        ]


def low_points(grid: Grid) -> Iterator[Tuple[int, int]]:
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            cur = grid[i][j]
            if all(
                grid[ni][nj] > cur
                for ni, nj in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                if len(grid) > ni >= 0 <= nj < len(grid[0])
            ):
                yield i, j


def total_risk(grid: Grid) -> int:
    return sum(
        1 + grid[i][j]
        for i, j in low_points(grid)
    )


def basin(grid: Grid, low: Tuple[int, int]) -> int:
    q = [low]
    total = 0
    seen = set()
    while q:
        i, j = q.pop()
        if (i, j) in seen:
            continue
        seen.add((i, j))
        total += 1
        cur = grid[i][j]
        for ni, nj in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
            if len(grid) > ni >= 0 <= nj < len(grid[0]):
                if cur < grid[ni][nj] < 9:
                    q.append((ni, nj))
    return total


def product_three_largest_basin(grid: Grid) -> int:
    basins = [basin(grid, low) for low in low_points(grid)]
    basins.sort(reverse=True)
    a, b, c, *_ = basins
    return a * b * c


example = read_grid('example')
input = read_grid('input')

assert total_risk(example) == 15
assert total_risk(input) == 494

assert basin(example, (0, 1)) == 3
assert basin(example, (0, 9)) == 9
assert basin(example, (2, 2)) == 14
assert basin(example, (4, 6)) == 9

assert product_three_largest_basin(example) == 1134
assert product_three_largest_basin(input) == 1048128

