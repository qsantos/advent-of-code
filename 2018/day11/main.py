from typing import Tuple


def fuel_cell_power(grid_serial_number: int, x: int, y: int) -> int:
    rack_id = x + 10
    power = rack_id * y
    power += grid_serial_number
    power *= rack_id
    power //= 100
    power %= 10
    power -= 5
    return power


def largest_total_3x3_square(grid_serial_number: int) -> Tuple[int, int]:
    grid = [
        [fuel_cell_power(grid_serial_number, x, y) for y in range(0, 301)]
        for x in range(0, 301)
    ]
    best_total = -5 * 9
    best_coord = 0, 0
    for x in range(1, 301 - 2):
        for y in range(1, 301 - 2):
            total = sum(grid[x + ox][y + oy] for ox in range(3) for oy in range(3))
            if total > best_total:
                best_total = total
                best_coord = (x, y)
    return best_coord


def largest_total_square(grid_serial_number: int) -> Tuple[int, int, int]:
    grid = [
        [fuel_cell_power(grid_serial_number, x, y) for y in range(0, 301)]
        for x in range(0, 301)
    ]
    squares = [[[0] * 301 for _ in range(301)] for _ in range(0, 301)]
    vertical_slices = [[[0] * 301 for _ in range(301)] for _ in range(0, 301)]
    horizontal_slices = [[[0] * 301 for _ in range(301)] for _ in range(0, 301)]
    for size in range(1, 301):
        for x in range(1, 301):
            for y in range(1, 301 - (size - 1) - 1):
                vertical_slices[size][x][y] = grid[x][y] + vertical_slices[size - 1][x][y + 1]
        for x in range(1, 301 - (size - 1) - 1):
            for y in range(1, 301):
                horizontal_slices[size][x][y] = grid[x][y] + horizontal_slices[size - 1][x + 1][y]
        for x in range(1, 301 - (size - 1) - 1):
            for y in range(1, 301 - (size - 1) - 1):
                squares[size][x][y] = (
                    grid[x][y]
                    + vertical_slices[size - 1][x][y + 1]
                    + horizontal_slices[size - 1][x + 1][y]
                    + squares[size - 1][x + 1][y + 1]
                )

    _, x, y, size = max(
        (squares[size][x][y], x, y, size)
        for size in range(1, 301)
        for x in range(1, 302 - size)
        for y in range(1, 302 - size)
    )
    return x, y, size


assert fuel_cell_power(8, 3, 5) == 4
assert fuel_cell_power(57, 122, 79) == -5
assert fuel_cell_power(39, 217, 196) == 0
assert fuel_cell_power(71, 101, 153) == 4

assert largest_total_3x3_square(42) == (21, 61)
assert largest_total_3x3_square(5153) == (235, 18)

assert largest_total_square(18) == (90, 269, 16)
assert largest_total_square(42) == (232, 251, 12)
assert largest_total_square(5153) == (236, 227, 12)
