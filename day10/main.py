from collections import defaultdict
from fractions import Fraction
from functools import cmp_to_key
from itertools import cycle
from textwrap import dedent
from typing import DefaultDict, Dict, Iterator, List, Optional, Tuple

Coord = Tuple[int, int]
Direction = Tuple[bool, Optional[Fraction]]


def load_map(s: str) -> List[str]:
    return dedent(s).strip().split('\n')


def group_asteroids(map: List[str], src: Coord) -> Dict[Direction, List[Coord]]:
    src_x, src_y = src
    assert map[src_y][src_x] == '#'
    visible: DefaultDict[Direction, List[Coord]] = defaultdict(list)
    for y in range(len(map)):
        for x in range(len(map[0])):
            if (y, x) == (src_y, src_x):
                continue
            if map[y][x] != '#':
                continue
            if src_x == x:
                d = src_y > y
                visible[d, None].append((x, y))
            else:
                d = src_x > x
                f = Fraction(src_y - y, src_x - x)
                visible[d, f].append((x, y))
    return dict(visible)


def count_visible(map: List[str], src: Coord) -> int:
    return len(group_asteroids(map, src))


def direction_cmp(a: Direction, b: Direction) -> int:
    if a == b:
        return 0

    ad, af = a
    bd, bf = b

    # welcome in hell
    if af is None:  # a vertical
        if ad:  # a in upper half
            return -1
        elif bf is None:  # a in lower half, b vertical in upper half
            return 1
        elif bd:  # a in lower half, b in left half
            return -1
        else:  # a in lower half, b in right half
            return 1
    elif bf is None:  # b vertical
        if bd:  # b in upper half
            return 1
        elif ad:  # b in lower half, a in left half
            return 1
        else:  # b in lower half, a in right half
            return -1
    elif ad:  # a in left half
        if bd:  # b in left half
            if af < bf:
                return -1
            else:
                return 1
        else:  # b in right half
            return 1
    else:  # a in right half
        if bd:  # b in left half
            return -1
        else:  # b in right half
            if af < bf:
                return -1
            else:
                return 1


def iter_asteroids(map: List[str], src: Coord) -> Iterator[Coord]:
    groups = group_asteroids(map, src)

    src_x, src_y = src

    def coord_key(coord: Coord) -> int:
        coord_x, coord_y = coord
        return abs(src_x - coord_x) + abs(src_y - coord_y)

    for direction in groups:
        groups[direction].sort(key=coord_key, reverse=True)

    directions = cycle(sorted(groups, key=cmp_to_key(direction_cmp)))
    for direction in directions:
        try:
            yield groups[direction].pop()
        except KeyError:
            continue

        if not groups[direction]:
            del groups[direction]

            if not groups:
                break


def find_best_position(map: List[str]) -> Coord:
    return max(
        (
            (x, y)
            for x in range(len(map[0]))
            for y in range(len(map))
            if map[y][x] == '#'
        ),
        key=lambda src: count_visible(map, src),
    )


def find_most_visible(map: List[str]) -> int:
    src = find_best_position(map)
    return count_visible(map, src)


def main() -> None:
    assert find_most_visible(load_map("""
    .#..#
    .....
    #####
    ....#
    ...##
    """)) == 8

    assert find_most_visible(load_map("""
    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####
    """)) == 33

    assert find_most_visible(load_map("""
    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.
    """)) == 35

    assert find_most_visible(load_map("""
    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..
    """)) == 41

    map = load_map("""
    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##
    """)
    src = find_best_position(map)
    assert count_visible(map, src) == 210
    asteroids = list(iter_asteroids(map, src))
    assert asteroids[0] == (11, 12)
    assert asteroids[1] == (12, 1)
    assert asteroids[2] == (12, 2)
    assert asteroids[9] == (12, 8)
    assert asteroids[19] == (16, 0)
    assert asteroids[49] == (16, 9)
    assert asteroids[99] == (10, 16)
    assert asteroids[198] == (9, 6)
    assert asteroids[199] == (8, 2)
    assert asteroids[200] == (10, 9)
    assert asteroids[298] == (11, 1)

    assert list(iter_asteroids(load_map("""
    .#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....#...###..
    ..#.#.....#....##
    """), (8, 3))) == [
        (8, 1),
        (9, 0),
        (9, 1),
        (10, 0),
        (9, 2),
        (11, 1),
        (12, 1),
        (11, 2),
        (15, 1),
        (12, 2),
        (13, 2),
        (14, 2),
        (15, 2),
        (12, 3),
        (16, 4),
        (15, 4),
        (10, 4),
        (4, 4),
        (2, 4),
        (2, 3),
        (0, 2),
        (1, 2),
        (0, 1),
        (1, 1),
        (5, 2),
        (1, 0),
        (5, 1),
        (6, 1),
        (6, 0),
        (7, 0),
        (8, 0),
        (10, 1),
        (14, 0),
        (16, 1),
        (13, 3),
        (14, 3),
    ]

    with open('input') as f:
        map = load_map(f.read())

    src = find_best_position(map)

    # puzzle 1
    assert count_visible(map, src) == 296

    asteroids = list(iter_asteroids(map, src))
    x, y = asteroids[199]
    assert 100 * x + y == 204


if __name__ == '__main__':
    main()
