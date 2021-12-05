from collections import defaultdict
from typing import DefaultDict, List, Tuple

Point = Tuple[int, int]
Vent = Tuple[Point, Point]


def sign(a: int) -> int:
    return (a > 0) - (a < 0)


def parse_point(s: str) -> Point:
    x, y = s.split(',')
    return int(x), int(y)


def read_vents(filename: str) -> List[Vent]:
    vents = []
    with open(filename) as f:
        for line in f:
            a, b = line.strip().split(' -> ')
            vents.append((parse_point(a), parse_point(b)))
    return vents


def count_horizontal_and_vertical_overlaps(vents: List[Vent]) -> int:
    d: DefaultDict[Point, int] = defaultdict(int)
    for (ax, ay), (bx, by) in vents:
        if ax == bx:
            l, h = min(ay, by), max(ay, by)
            for y in range(l, h + 1):
                d[ax, y] += 1
        elif ay == by:
            l, h = min(ax, bx), max(ax, bx)
            for x in range(l, h + 1):
                d[x, ay] += 1
    return sum(count >= 2 for count in d.values())


def count_overlaps(vents: List[Vent]) -> int:
    d: DefaultDict[Point, int] = defaultdict(int)
    for (ax, ay), (bx, by) in vents:
        dx = sign(bx - ax)
        dy = sign(by - ay)
        x, y = ax, ay
        d[x, y] += 1
        while (x, y) != (bx, by):
            x += dx
            y += dy
            d[x, y] += 1
    return sum(count >= 2 for count in d.values())


example = read_vents('example')
input = read_vents('input')

assert count_horizontal_and_vertical_overlaps(example) == 5
assert count_horizontal_and_vertical_overlaps(input) == 7142

assert count_overlaps(example) == 12
assert count_overlaps(input) == 20012
