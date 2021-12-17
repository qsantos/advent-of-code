import re
from typing import Tuple

Coord = Tuple[int, int]
TargetArea = Tuple[int, int, int, int]

pattern_target_area = re.compile(r'^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$')


def read_target_area(filename: str) -> TargetArea:
    with open(filename) as f:
        m = pattern_target_area.match(f.read().strip())
        assert m is not None
        min_x, max_x, min_y, max_y = (int(group) for group in m.groups())
        return min_x, max_x, min_y, max_y


def reaches_target_y(target_area: TargetArea, vy0: int) -> int:
    # vy(t) = vy0 - t
    # y(t) = sum(vy(t - 1) for t)
    #      = sum(vy0 - t + 1 for t)
    #      = vy0 * t - t * (t - 1) // 2
    _, _, min_y, max_y = target_area
    vy = abs(vy0)
    y = 0
    t = 0
    while y >= max_y:
        y += vy
        vy -= 1
        t += 1
        assert y == vy0 * t - t * (t - 1) // 2
    return t if vy >= min_y else 0


def reaches_target(target_area: TargetArea, vx0: int, vy0: int) -> int:
    min_x, max_x, min_y, max_y = target_area
    vx = vx0
    vy = vy0
    x = 0
    y = 0
    t = 0
    while y >= min_y:
        if min_y <= y <= max_y and min_x <= x <= max_x:
            return t
        x += vx
        y += vy
        if vx > 0:
            vx -= 1
        vy -= 1
        t += 1
    return 0


def max_vy0_y(target_area: TargetArea) -> Tuple[int, int]:
    min_x, max_x, min_y, max_y = target_area
    vy0 = -min_y
    while not reaches_target_y(target_area, vy0):
        vy0 -= 1
    vy0 += 1
    highest = vy0 * (vy0 + 1) // 2 if vy0 > 0 else 0
    return vy0, highest


def count_initial_velocities(target_area: TargetArea) -> int:
    min_x, max_x, min_y, max_y = target_area
    max_vy0, _ = max_vy0_y(target_area)
    count = 0
    for vy0 in range(min_y, max_vy0 + 1):
        for vx0 in range(max_x + 2):
            if reaches_target(target_area, vx0, vy0):
                count += 1
    return count


def main() -> None:
    example = read_target_area('example')
    input = read_target_area('input')

    assert max_vy0_y(example) == (9, 45)
    assert max_vy0_y(input) == (147, 10878)

    assert count_initial_velocities(example) == 112
    assert count_initial_velocities(input) == 4716


if __name__ == '__main__':
    main()
