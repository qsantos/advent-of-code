from math import ceil, sqrt
from typing import Dict, Iterator, Tuple


def spiral_distance(n: int) -> int:
    # last_number of k-th spiral = (2k + 1)²
    # select last spiral that ends below n
    k = ceil((sqrt(n) - 1) / 2) - 1
    # end of k-th spiral is at (k, k)
    n -= (2 * k + 1)**2
    if n <= 2 * k + 2:
        # right
        return abs(k + 1) + abs(k + 1 - n)
    n -= 2 * k + 2
    if n <= 2 * k + 2:
        # top
        return abs(k + 1 - n) + abs(- k - 1)
    n -= 2 * k + 2
    if n <= 2 * k + 2:
        # left
        return abs(- k - 1) + abs(- k - 1 + n)
    n -= 2 * k + 2
    # bottom
    return abs(- k - 1 + n) + abs(k + 1)


def iter_spiral_coords() -> Iterator[Tuple[int, int]]:
    yield 0, 0
    k = 1
    while True:
        size = 2 * k + 1
        # right
        for m in range(1, size):
            yield k, k - m
        # top
        for m in range(1, size):
            yield k - m, -k
        # left
        for m in range(1, size):
            yield -k, -k + m
        # bottom
        for m in range(1, size):
            yield -k + m, k
        k += 1


def iter_weird_spiral_values() -> Iterator[int]:
    values: Dict[Tuple[int, int], int] = {}
    for x, y in iter_spiral_coords():
        s = sum(
            values.get((nx, ny), 0)
            for nx in (x - 1, x, x + 1)
            for ny in (y - 1, y, y + 1)
        ) or 1
        yield s
        values[x, y] = s


def main() -> None:
    assert spiral_distance(1) == 0
    assert spiral_distance(2) == 1
    assert spiral_distance(3) == 2
    assert spiral_distance(4) == 1
    assert spiral_distance(5) == 2
    assert spiral_distance(6) == 1
    assert spiral_distance(7) == 2
    assert spiral_distance(8) == 1
    assert spiral_distance(9) == 2
    assert spiral_distance(12) == 3
    assert spiral_distance(23) == 2
    assert spiral_distance(1024) == 31

    assert spiral_distance(265149) == 438

    for value in iter_weird_spiral_values():
        if value > 265149:
            assert value == 266330
            break


if __name__ == '__main__':
    main()
