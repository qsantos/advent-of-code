from typing import List, Set, Tuple

Coord = Tuple[int, int, int, int]
Constellation = Set[Coord]


def parse_coord(s: str) -> Coord:
    x, y, z, w = [int(x) for x in s.split(',')]
    return x, y, z, w


def read_points(filename: str) -> List[Coord]:
    with open(filename) as f:
        return [
            parse_coord(line.strip())
            for line in f
        ]


def distance(a: Coord, b: Coord) -> int:
    return sum(abs(ax - bx) for ax, bx in zip(a, b))


def constellation_distance(a: Constellation, b: Constellation) -> int:
    return min(
        distance(ap, bp)
        for ap in a
        for bp in b
    )


def count_constellations(points: List[Coord]) -> int:
    constellation_of_root = {
        point: {point}
        for point in points
    }
    constellation_roots = set(points)
    while True:
        merged = set()
        next_constellation_roots = set()
        for ar in constellation_roots:
            a = constellation_of_root[ar]
            for br in constellation_roots:
                if br in merged or ar <= br:
                    continue
                b = constellation_of_root[br]
                if constellation_distance(a, b) <= 3:
                    b.update(a)
                    merged.add(ar)
                    break
            else:
                next_constellation_roots.add(ar)

        if next_constellation_roots == constellation_roots:
            break
        constellation_roots = next_constellation_roots
    return len(constellation_roots)


def main() -> None:
    example1 = read_points('example1')
    example2 = read_points('example2')
    example3 = read_points('example3')
    example4 = read_points('example4')
    input = read_points('input')

    assert count_constellations(example1) == 2
    assert count_constellations(example2) == 4
    assert count_constellations(example3) == 3
    assert count_constellations(example4) == 8
    assert count_constellations(input) == 422


if __name__ == '__main__':
    main()
