from fractions import Fraction
from textwrap import dedent
from typing import List, Optional, Set, Tuple


def load_map(s: str) -> List[str]:
    return dedent(s).strip().split('\n')


def count_visible(map: List[str], src: Tuple[int, int]) -> int:
    src_x, src_y = src
    assert map[src_y][src_x] == '#'
    visible: Set[Tuple[bool, Optional[Fraction]]] = set()
    for y in range(len(map)):
        for x in range(len(map[0])):
            if (y, x) == (src_y, src_x):
                continue
            if map[y][x] != '#':
                continue
            if src_x == x:
                d = src_y > y
                visible.add((d, None))
            else:
                d = src_x > x
                f = Fraction(src_y - y, src_x - x)
                visible.add((d, f))
    return len(visible)


def find_best_position(map: List[str]) -> Tuple[int, int]:
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

    assert find_most_visible(load_map("""
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
    """)) == 210

    with open('input') as f:
        map = load_map(f.read())

    src = find_best_position(map)

    # puzzle 1
    assert count_visible(map, src) == 296


if __name__ == '__main__':
    main()
