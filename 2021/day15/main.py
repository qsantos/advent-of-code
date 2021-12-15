from heapq import heappush, heappop
from typing import List

Grid = List[List[int]]


def read_grid(filename: str) -> Grid:
    with open(filename) as f:
        return [
            [int(x) for x in line.strip()]
            for line in f
        ]


def lowest_total_risk(grid: Grid, size_multiplier: int) -> int:
    rows = len(grid)
    cols = len(grid[0])
    assert all(len(row) == cols for row in grid)
    total_rows = size_multiplier * rows
    total_cols = size_multiplier * cols
    start = (0, 0)
    end = (total_cols - 1, total_cols - 1)
    initial_cost = 0
    q = [(initial_cost, start)]
    seen = set()
    while q:
        accumulated_cost, state = heappop(q)
        if state in seen:
            continue
        seen.add(state)
        if state == end:
            return accumulated_cost
        i, j = state
        for ni, nj in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
            if 0 <= ni < total_rows and 0 <= nj < total_cols:
                cost = grid[ni % rows][nj % cols] + ni // rows + nj // cols
                cost = 1 + (cost - 1) % 9
                heappush(q, (accumulated_cost + cost, (ni, nj)))
    assert False


def main() -> None:
    example = read_grid('example')
    input = read_grid('input')

    assert lowest_total_risk(example, 1) == 40
    assert lowest_total_risk(input, 1) == 415

    assert lowest_total_risk(example, 5) == 315
    assert lowest_total_risk(input, 5) == 2864


if __name__ == '__main__':
    main()
