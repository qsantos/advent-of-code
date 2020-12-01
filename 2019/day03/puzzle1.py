from typing import Iterable, Tuple

Path = Iterable[Tuple[str, int]]


def parse_path(s: str) -> Path:
    for part in s.strip().split(','):
        yield part[0], int(part[1:])


def walk(path: Path) -> Iterable[Tuple[int, int]]:
    pos_i, pos_j = 0, 0
    for direction, steps in path:
        if direction == 'U':
            rel_i, rel_j = -1, 0
        elif direction == 'D':
            rel_i, rel_j = 1, 0
        elif direction == 'L':
            rel_i, rel_j = 0, -1
        elif direction == 'R':
            rel_i, rel_j = 0, 1

        for step in range(steps):
            pos_i += rel_i
            pos_j += rel_j

            yield pos_i, pos_j


def intersections(path1: Path, path2: Path) -> Iterable[Tuple[int, int]]:
    visited = set()
    for pos in walk(path1):
        visited.add(pos)
    for pos in walk(path2):
        if pos in visited:
            yield pos


def closest_intersection(path1: Path, path2: Path) -> int:
    return min(abs(i) + abs(j) for i, j in intersections(path1, path2))


def main() -> None:
    assert closest_intersection(parse_path('R8,U5,L5,D3'), parse_path('U7,R6,D4,L4')) == 6
    assert closest_intersection(
        parse_path('R75,D30,R83,U83,L12,D49,R71,U7,L72'),
        parse_path('U62,R66,U55,R34,D71,R55,D58,R83'),
    ) == 159
    assert closest_intersection(
        parse_path('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51'),
        parse_path('U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'),
    ) == 135

    with open('input') as f:
        path1 = parse_path(next(f))
        path2 = parse_path(next(f))

    print(closest_intersection(path1, path2))


if __name__ == '__main__':
    main()
