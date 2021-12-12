from typing import Dict, Iterator, List, Optional, Set, Tuple

Coord = Tuple[int, int, int]
Nanobot = Tuple[Coord, int]
Cube = Tuple[Coord, int]
Parallelepiped = Tuple[Coord, Coord]


def read_nanobot(s: str) -> Nanobot:
    left, right = s.rsplit(', ', 1)
    x, y, z = [int(x) for x in left[len('pos=<'):-len('>')].split(',')]
    r = int(right[len('r='):])
    return (x, y, z), r


def read_nanobots(filename: str) -> List[Nanobot]:
    with open(filename) as f:
        return [
            read_nanobot(line.strip())
            for line in f
        ]


def distance(a: Coord, b: Coord) -> int:
    return sum(abs(ax - bx) for ax, bx in zip(a, b))


def n_nanobots_in_range_of_strongest(nanobots: List[Nanobot]) -> int:
    coord, r = max(nanobots, key=lambda nanobot: nanobot[1])
    return sum(
        distance(coord, other) <= r
        for other, _ in nanobots
    )


def global_cube(nanobots: List[Nanobot]) -> Cube:
    min_x = min(x for (x, y, z), _ in nanobots)
    min_y = min(y for (x, y, z), _ in nanobots)
    min_z = min(z for (x, y, z), _ in nanobots)
    max_x = max(x for (x, y, z), _ in nanobots)
    max_y = max(y for (x, y, z), _ in nanobots)
    max_z = max(z for (x, y, z), _ in nanobots)
    r = max(max_x - min_x, max_y - min_y, max_z - min_z) + 1
    return (min_x, min_y, min_z), r


def iter_subcubes(cube: Cube) -> Iterator[Cube]:
    (x, y, z), r = cube
    r = (r - 1) // 2 + 1
    yield (x + 0, y + 0, z + 0), r
    yield (x + 0, y + 0, z + r), r
    yield (x + 0, y + r, z + 0), r
    yield (x + 0, y + r, z + r), r
    yield (x + r, y + 0, z + 0), r
    yield (x + r, y + 0, z + r), r
    yield (x + r, y + r, z + 0), r
    yield (x + r, y + r, z + r), r


def cube_distance(cube: Cube, p: Coord) -> int:
    (cx, cy, cz), r = cube
    px, py, pz = p
    dx = max(0, cx - px, px - (cx + r - 1))
    dy = max(0, cy - py, py - (cy + r - 1))
    dz = max(0, cz - pz, pz - (cz + r - 1))
    return dx + dy + dz


def bots_in_range_of_cube(nanobots: List[Nanobot], cube: Cube) -> int:
    return sum(
        cube_distance(cube, p) <= r
        for p, r in nanobots
    )


def most_social_nanobot_bla(nanobots: List[Nanobot], threshold: int) -> Optional[Coord]:
    cubes = [global_cube(nanobots)]
    while cubes:
        cubes = [
            subcube
            for cube in cubes
            for subcube in iter_subcubes(cube)
            if bots_in_range_of_cube(nanobots, subcube) >= threshold
        ]
        if not cubes:
            return None
        _, r = cubes[0]
        if r == 1:
            return min(coord for coord, _ in cubes)
    return None


def most_social_nanobot(nanobots: List[Nanobot]) -> int:
    threshold = len(nanobots)
    for threshold in reversed(range(len(nanobots) + 1)):
        ret = most_social_nanobot_bla(nanobots, threshold)
        if ret is not None:
            return sum(ret)
    assert False


def main() -> None:
    example1 = read_nanobots('example1')
    example2 = read_nanobots('example2')
    input = read_nanobots('input')

    assert n_nanobots_in_range_of_strongest(example1) == 7
    assert n_nanobots_in_range_of_strongest(input) == 652

    assert most_social_nanobot(example2) == 36
    assert most_social_nanobot(input) == 164960498


if __name__ == '__main__':
    main()
