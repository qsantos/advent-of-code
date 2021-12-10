from collections import deque
from typing import Dict, List, Tuple, Set

Coord = Tuple[int, int]


def read_coords(filename: str) -> List[Coord]:
    coords = []
    with open(filename) as f:
        for line in f:
            x, y = line.strip().split(',')
            coords.append((int(x), int(y)))
    return coords


def biggest_area(coords: List[Coord]) -> int:
    min_x = min(x for x, y in coords)
    min_y = min(y for x, y in coords)
    max_x = max(x for x, y in coords)
    max_y = max(y for x, y in coords)

    infinites = set()
    q = deque(
        (0, i, coord)
        for i, coord in enumerate(coords)
    )
    areas: Dict[int, Set[Coord]] = {
        i: set()
        for i in range(len(coords))
    }
    seen: Dict[Tuple[int, int], Tuple[int, int]] = {}
    collisions = set()
    while q:
        d, area, (x, y) = q.popleft()
        if not (min_x <= x <= max_x and min_y <= y <= max_y):
            infinites.add(area)
            continue
        if (x, y) in collisions:
            continue
        if (x, y) in seen:
            other_area, other_d = seen[x, y]
            if other_area != area and other_d == d:
                collisions.add((x, y))
                areas[other_area] -= {(x, y)}
            continue
        seen[x, y] = area, d
        areas[area].add((x, y))

        for nx, ny in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]:
            q.append((d + 1, area, (nx, ny)))

    return max(
        len(areas[area])
        for area in areas
        if area not in infinites
    )


def area_close_to_point(coord: Coord, stop_distance: int) -> Dict[Coord, int]:
    ox, oy = coord
    return {
        (x, y): abs(ox - x) + abs(oy - y)
        for x in range(ox - stop_distance + 1, ox + stop_distance)
        for y in range(oy - (stop_distance - abs(ox - x)) + 1, oy + (stop_distance - abs(ox - x)))
    }


def area_close_to_all_points(coords: List[Coord], stop_distance: int) -> int:
    min_x = min(x for x, y in coords) - stop_distance // len(coords)
    min_y = min(y for x, y in coords) - stop_distance // len(coords)
    max_x = max(x for x, y in coords) + stop_distance // len(coords)
    max_y = max(y for x, y in coords) + stop_distance // len(coords)
    return sum(
        sum(abs(x - ox) + abs(y - oy) for ox, oy in coords) < stop_distance
        for x in range(min_x, max_x + 1)
        for y in range(min_y, max_y + 1)
    )


example = read_coords('example')
input = read_coords('input')

assert biggest_area(example) == 17
assert biggest_area(input) == 3620

assert area_close_to_all_points(example, 32) == 16
assert area_close_to_all_points(input, 10_000) == 39930
