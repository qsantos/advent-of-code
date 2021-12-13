from typing import List, Tuple

Coord = Tuple[int, int]


def read_path(filename: str) -> List[str]:
    with open(filename) as f:
        return f.read().strip().split(',')


def coord_of_path(path: List[str]) -> Coord:
    q = 0
    r = 0
    for step in path:
        if step == 'n':
            q += 1
        elif step == 's':
            q -= 1
        elif step == 'ne':
            r += 1
        elif step == 'nw':
            q += 1
            r -= 1
        elif step == 'se':
            q -= 1
            r += 1
        elif step == 'sw':
            r -= 1
        else:
            assert False, step
    return q, r


def distance(c: Coord) -> int:
    q, r = c
    return (abs(q) + abs(q + r) + abs(r)) // 2


def distance_to_origin(path: List[str]) -> int:
    return distance(coord_of_path(path))


def furthest_position(path: List[str]) -> int:
    max_d = 0
    q = 0
    r = 0
    for step in path:
        if step == 'n':
            q += 1
        elif step == 's':
            q -= 1
        elif step == 'ne':
            r += 1
        elif step == 'nw':
            q += 1
            r -= 1
        elif step == 'se':
            q -= 1
            r += 1
        elif step == 'sw':
            r -= 1
        else:
            assert False, step
        max_d = max(max_d, distance((q, r)))
    return max_d


def main() -> None:
    input = read_path('input')

    assert distance_to_origin('ne,ne,ne'.split(',')) == 3
    assert distance_to_origin('ne,ne,sw,sw'.split(',')) == 0
    assert distance_to_origin('ne,ne,s,s'.split(',')) == 2
    assert distance_to_origin('se,sw,se,sw,sw'.split(',')) == 3
    assert distance_to_origin(input) == 743

    assert furthest_position(input) == 1493


if __name__ == '__main__':
    main()
