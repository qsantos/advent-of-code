from sys import setrecursionlimit
from typing import Optional, Set, Tuple

setrecursionlimit(10000)

Coord = Tuple[int, int]


def read_clay(filename: str) -> Set[Coord]:
    clay = set()
    with open(filename) as f:
        for line in f:
            left, right = line.strip().split(', ')
            a = int(left[len('x='):])
            b_start, b_end = right[len('x='):].split('..')
            for b in range(int(b_start), int(b_end) + 1):
                if left[0] == 'x':
                    clay.add((a, b))
                else:
                    clay.add((b, a))
    return clay


def print_state(
    clay: Set[Coord],
    wet: Set[Coord] = set(),
    full: Set[Coord] = set(),
    cur: Optional[Coord] = None,
) -> None:
    occupied = clay | wet | full
    min_x = min(x for x, y in occupied)
    max_x = max(x for x, y in occupied)
    max_y = max(y for x, y in occupied)
    for y in range(max_y + 1):
        print(''.join(
            'X' if (x, y) == cur else
            '#' if (x, y) in clay else
            '~' if (x, y) in full else
            '|' if (x, y) in wet else
            '.'
            for x in range(min_x, max_x + 1)
        ))


def run_water(clay: Set[Coord]) -> Tuple[Set[Coord], Set[Coord]]:
    def can_fill_dir(x: int, y: int, dx: int) -> bool:
        while (x, y) not in full:
            x += dx
            wet.add((x, y))
            aux(x, y + 1)
            if (x, y + 1) not in full:
                return False
        return True

    def fill_dir(x: int, y: int, dx: int) -> None:
        while (x, y) not in full:
            full.add((x, y))
            x += dx

    def aux(x: int, y: int) -> None:
        if (x, y) in wet:
            return
        wet.add((x, y))
        if y == max_y:
            return
        aux(x, y + 1)
        if (x, y + 1) in full and can_fill_dir(x, y, -1) & can_fill_dir(x, y, +1):
            fill_dir(x, y, -1)
            fill_dir(x + 1, y, +1)

    full = set(clay)
    wet = set(clay)
    max_y = max(y for x, y in clay)
    aux(500, 0)
    assert full <= wet
    return wet, full


def count_all_water(clay: Set[Coord]) -> int:
    wet, full = run_water(clay)
    print_state(clay, wet, full)
    min_y = min(y for x, y in clay)
    max_y = max(y for x, y in clay)
    water = wet - clay
    return sum(min_y <= y <= max_y for x, y in water)


def count_retained_water(clay: Set[Coord]) -> int:
    wet, full = run_water(clay)
    min_y = min(y for x, y in clay)
    max_y = max(y for x, y in clay)
    water = full - clay
    return sum(min_y <= y <= max_y for x, y in water)


def main() -> None:
    example = read_clay('example')
    input = read_clay('input')

    assert count_all_water(example) == 57
    assert count_all_water(input) == 39649

    assert count_retained_water(example) == 29
    assert count_retained_water(input) == 28872


if __name__ == '__main__':
    main()
