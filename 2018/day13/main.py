from typing import List, Optional, Tuple

Cart = Tuple[int, int, int, int, int]
Grid = List[List[str]]
Tracks = Tuple[Grid, List[Cart]]


def read_tracks(filename: str) -> Tracks:
    with open(filename) as f:
        grid = [
            list(line.rstrip('\r\n'))
            for line in f
        ]
    carts = []
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c not in '<>^v':
                continue
            if c == '<':
                carts.append((x, y, -1, 0, 0))
                row[x] = '-'
            elif c == '>':
                carts.append((x, y, +1, 0, 0))
                row[x] = '-'
            elif c == '^':
                carts.append((x, y, 0, -1, 0))
                row[x] = '|'
            elif c == 'v':
                carts.append((x, y, 0, +1, 0))
                row[x] = '|'
    return grid, carts


def print_grid(grid: Grid) -> None:
    for row in grid:
        print(''.join(row))


def print_carts(grid: Grid, carts: List[Cart]) -> None:
    dir_of_delta = {
        (-1, 0): '<',
        (+1, 0): '>',
        (0, -1): '^',
        (0, +1): 'v',
    }
    dirs = {
        (x, y): dir_of_delta[dx, dy]
        for x, y, dx, dy, choice in carts
    }
    for y, row in enumerate(grid):
        print(''.join(
            dirs[x, y] if (x, y) in dirs else c
            for x, c in enumerate(row)
        ))


def next_step(grid: Grid, carts: List[Cart]) -> Tuple[List[Cart], Optional[Tuple[int, int]]]:
    occupied = {
        (x, y)
        for (x, y, dx, dy, choice) in carts
    }
    crash = None
    crashed = set()
    next_carts = []
    for x, y, dx, dy, choice in sorted(carts):
        occupied.remove((x, y))
        if (x, y) in crashed:
            continue
        x += dx
        y += dy
        if (x, y) in occupied:
            crash = (x, y)
            crashed.add((x, y))
            continue
        c = grid[y][x]
        if c == '/':
            dx, dy = -dy, -dx
        elif c == '\\':
            dx, dy = dy, dx
        elif c == '+':
            if choice == 0:
                # turn left
                dx, dy = dy, -dx
                choice = 1
            elif choice == 1:
                # go straight
                choice = 2
            else:
                # turn right
                dx, dy = -dy, dx
                choice = 0
        elif c == '-' or c == '|':
            pass
        else:
            assert False, c
        occupied.add((x, y))
        next_carts.append((x, y, dx, dy, choice))
    next_carts = [
        (x, y, dx, dy, choice)
        for x, y, dx, dy, choice in next_carts
        if (x, y) not in crashed
    ]
    return next_carts, crash


def first_crash(tracks: Tracks) -> Tuple[int, int]:
    grid, carts = tracks
    while True:
        carts, crash = next_step(grid, carts)
        if crash is not None:
            return crash


def last_cart(tracks: Tracks) -> Tuple[int, int]:
    grid, carts = tracks
    while len(carts) > 1:
        carts, _ = next_step(grid, carts)
    (x, y, dx, dy, choice), = carts
    return x, y


example1 = read_tracks('example1')
example2 = read_tracks('example2')
input = read_tracks('input')

assert first_crash(example1) == (7, 3)
assert first_crash(input) == (130, 104)

assert last_cart(example2) == (6, 4)
assert last_cart(input) == (29, 83)
