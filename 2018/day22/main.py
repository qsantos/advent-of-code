from functools import cache
from heapq import heappush, heappop
from sys import setrecursionlimit
from typing import Tuple

setrecursionlimit(10000)

Coord = Tuple[int, int]
# Grid = List[List[int]]


@cache
def erosion_level(depth: int, target: Coord, x: int, y: int) -> int:
    if (x, y) == (0, 0):
        geologic_index = 0
    elif (x, y) == target:
        geologic_index = 0
    elif y == 0:
        geologic_index = x * 16807
    elif x == 0:
        geologic_index = y * 48271
    else:
        a = erosion_level(depth, target, x - 1, y)
        b = erosion_level(depth, target, x, y - 1)
        geologic_index = a * b
    return (geologic_index + depth) % 20183


def risk_level(depth: int, target: Coord, x: int, y: int) -> int:
    return erosion_level(depth, target, x, y) % 3


# def print_grid(grid: Grid) -> None:
#     region_type_of_erosion_level = {
#         0: '.',  # rocky
#         1: '=',  # wet
#         2: '|',  # narrow
#     }
#     for row in grid:
#         print(''.join(
#             region_type_of_erosion_level[c]
#             for c in row
#         ))
#
#
# def generate_grid(depth: int, target: Coord) -> Grid:
#     max_x, max_y = target
#     erosion_level_prev_row: List[int] = []
#     grid = []
#     for y in range(max_y + 1):
#         cur_row = []
#         erosion_level_cur_row: List[int] = []
#         for x in range(max_x + 1):
#             if (x, y) == (0, 0):
#                 geologic_index = 0
#             elif (x, y) == target:
#                 geologic_index = 0
#             elif y == 0:
#                 geologic_index = x * 16807
#             elif x == 0:
#                 geologic_index = y * 48271
#             else:
#                 a = erosion_level_cur_row[-1]  # x - 1, y
#                 b = erosion_level_prev_row[x]  # x, y - 1
#                 geologic_index = a * b
#             erosion_level = (geologic_index + depth) % 20183
#             erosion_level_cur_row.append(erosion_level)
#             cur_row.append(erosion_level % 3)
#         erosion_level_prev_row = erosion_level_cur_row
#         grid.append(cur_row)
#     return grid
#
#
# def total_risk_level(depth: int, target: Coord) -> int:
#     grid = generate_grid(depth, target)
#     return sum(sum(row) for row in grid)


def total_risk_level(depth: int, target: Coord) -> int:
    max_x, max_y = target
    return sum(
        risk_level(depth, target, x, y)
        for x in range(0, max_x + 1)
        for y in range(0, max_y + 1)
    )


neither = 'neither'
torch = 'torch'
climbing_gear = 'climbing_gear'


@cache
def equipment_is_compatible(depth: int, target: Coord, x: int, y: int, equipment: str) -> bool:
    c = risk_level(depth, target, x, y)
    if c == 0 and equipment != neither:
        return True
    if c == 1 and equipment != torch:
        return True
    if c == 2 and equipment != climbing_gear:
        return True
    return False


def fastest_way(depth: int, target: Coord) -> int:
    tx, ty = target
    end = (*target, torch)
    q = [(tx + ty, 0, (0, 0, torch))]
    seen = set()
    while q:
        _, d, state = heappop(q)
        if state in seen:
            continue
        seen.add(state)
        x, y, equipment = state
        if state == end:
            return d
        for nx, ny in (x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y):
            if nx < 0 or ny < 0:
                continue
            if equipment_is_compatible(depth, target, nx, ny, equipment):
                heappush(q, (d + 1 + abs(nx - tx) + abs(ny - ty), d + 1, (nx, ny, equipment)))
        for other in [neither, torch, climbing_gear]:
            if other != equipment and equipment_is_compatible(depth, target, x, y, other):
                heappush(q, (d + 7 + abs(x - tx) + abs(y - ty), d + 7, (x, y, other)))
    assert False


def main() -> None:
    example = (510, (10, 10))
    input = (11394, (7, 701))

    assert erosion_level(*example, 0, 0) == 510
    assert erosion_level(*example, 1, 0) == 17317
    assert erosion_level(*example, 0, 1) == 8415
    assert erosion_level(*example, 1, 1) == 1805
    assert erosion_level(*example, 10, 10) == 510

    assert total_risk_level(*example) == 114
    assert total_risk_level(*input) == 5637

    assert fastest_way(*example) == 45
    assert fastest_way(*input) == 969


if __name__ == '__main__':
    main()
