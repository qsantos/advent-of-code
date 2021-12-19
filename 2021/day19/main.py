from collections import Counter
from typing import Dict, Iterable, Iterator, List, Optional, Set, Tuple

Orientation = Tuple[int, int, int, int, int, int]
Vec3 = Tuple[int, int, int]
Scan = Set[Vec3]

# include reflections
orientations = [
    (0, 1, 2, +1, +1, +1),
    (0, 1, 2, +1, +1, -1),
    (0, 1, 2, +1, -1, +1),
    (0, 1, 2, +1, -1, -1),
    (0, 1, 2, -1, +1, +1),
    (0, 1, 2, -1, +1, -1),
    (0, 1, 2, -1, -1, +1),
    (0, 1, 2, -1, -1, -1),

    (0, 2, 1, +1, +1, +1),
    (0, 2, 1, +1, +1, -1),
    (0, 2, 1, +1, -1, +1),
    (0, 2, 1, +1, -1, -1),
    (0, 2, 1, -1, +1, +1),
    (0, 2, 1, -1, +1, -1),
    (0, 2, 1, -1, -1, +1),
    (0, 2, 1, -1, -1, -1),

    (1, 0, 2, +1, +1, +1),
    (1, 0, 2, +1, +1, -1),
    (1, 0, 2, +1, -1, +1),
    (1, 0, 2, +1, -1, -1),
    (1, 0, 2, -1, +1, +1),
    (1, 0, 2, -1, +1, -1),
    (1, 0, 2, -1, -1, +1),
    (1, 0, 2, -1, -1, -1),

    (1, 2, 0, +1, +1, +1),
    (1, 2, 0, +1, +1, -1),
    (1, 2, 0, +1, -1, +1),
    (1, 2, 0, +1, -1, -1),
    (1, 2, 0, -1, +1, +1),
    (1, 2, 0, -1, +1, -1),
    (1, 2, 0, -1, -1, +1),
    (1, 2, 0, -1, -1, -1),

    (2, 0, 1, +1, +1, +1),
    (2, 0, 1, +1, +1, -1),
    (2, 0, 1, +1, -1, +1),
    (2, 0, 1, +1, -1, -1),
    (2, 0, 1, -1, +1, +1),
    (2, 0, 1, -1, +1, -1),
    (2, 0, 1, -1, -1, +1),
    (2, 0, 1, -1, -1, -1),

    (2, 1, 0, +1, +1, +1),
    (2, 1, 0, +1, +1, -1),
    (2, 1, 0, +1, -1, +1),
    (2, 1, 0, +1, -1, -1),
    (2, 1, 0, -1, +1, +1),
    (2, 1, 0, -1, +1, -1),
    (2, 1, 0, -1, -1, +1),
    (2, 1, 0, -1, -1, -1),
]


def read_scans(filename: str) -> List[Scan]:
    with open(filename) as f:
        scans = []
        for line in f:  # '-- scanner %d --\n'
            scan = set()
            for line in f:
                if not line.strip():
                    break
                x, y, z = (int(part) for part in line.strip().split(','))
                scan.add((x, y, z))
            scans.append(scan)
        return scans


def orientate_point(point: Vec3, orientation: Orientation) -> Vec3:
    xm, ym, zm, xs, ys, zs = orientation
    return point[xm] * xs, point[ym] * ys, point[zm] * zs


def orientate_points(points: Iterable[Vec3], orientation: Orientation) -> Set[Vec3]:
    return {
        orientate_point(point, orientation)
        for point in points
    }


def distance(a: Vec3, b: Vec3) -> int:
    return sum(abs(ax - bx) for ax, bx in zip(a, b))


def counter_intersect_count(a: Counter, b: Counter) -> int:
    return sum(min(a[v], b[v]) for v in a if v in b)


def scans_overlap(a: Scan, b: Scan) -> bool:
    a_distances = Counter(
        distance(p, q)
        for p in a
        for q in a
        if p != q
    )
    b_distances = Counter(
        distance(p, q)
        for p in b
        for q in b
        if p != q
    )
    return counter_intersect_count(a_distances, b_distances) >= 132


def find_same_point_candidates(a: Scan, b: Scan) -> Optional[Tuple[Vec3, Vec3]]:
    for p in a:
        pc = Counter(
            distance(p, o)
            for o in a
            if p != o
        )
        for q in b:
            qc = Counter(
                distance(q, o)
                for o in b
                if q != o
            )
            if counter_intersect_count(pc, qc) >= 11:
                return p, q
    return None


def point_add(a: Vec3, b: Vec3) -> Vec3:
    ax, ay, az = a
    bx, by, bz = b
    return ax + bx, ay + by, az + bz


def point_sub(a: Vec3, b: Vec3) -> Vec3:
    ax, ay, az = a
    bx, by, bz = b
    return ax - bx, ay - by, az - bz


def scan_relative_to(ref: Scan, scan: Scan) -> Optional[Tuple[Orientation, Vec3, Scan]]:
    ref_set = set(ref)
    same_point = find_same_point_candidates(ref, scan)
    if same_point is None:
        return None
    ref0, scan0 = same_point
    for orientation in orientations:
        oriented_scan0 = orientate_point(scan0, orientation)
        oriented_diff = point_sub(ref0, oriented_scan0)
        assert point_add(oriented_scan0, oriented_diff) == ref0
        scan_relative_to_ref = {
            point_add(orientate_point(point, orientation), oriented_diff)
            for point in scan
        }
        if len(scan_relative_to_ref & ref_set) >= 12:
            return orientation, oriented_diff, scan_relative_to_ref
    return None


def assemble_beacons(scans: List[Scan]) -> Tuple[Set[Vec3], Set[Vec3]]:
    scanner_neighbors = {
        i: {(0, 0, 0)}
        for i in range(len(scans))
    }
    while True:
        print([len(x) for x in scans])
        for i in range(len(scans)):
            a = scans[i]
            if not a:
                continue
            for j in range(i + 1, len(scans)):
                b = scans[j]
                if not b:
                    continue
                x = scan_relative_to(a, b)
                if x is None:
                    continue
                orientation, diff, b_relative_to_a = x
                a.update(b_relative_to_a)
                for neighbor in scanner_neighbors[j]:
                    oriented_neighbor = point_add(orientate_point(neighbor, orientation), diff)
                    scanner_neighbors[i].add(oriented_neighbor)
                b.clear()
                scanner_neighbors[j].clear()
                break
            else:
                continue
            break
        else:
            break
    return scanner_neighbors[0], scans[0]


def solve_puzzle(scans: List[Scan]) -> Tuple[int, int]:
    scanners, beacons = assemble_beacons(scans)
    max_d = max(
        sum(abs(ax - bx) for ax, bx in zip(a, b))
        for a in scanners
        for b in scanners
    )
    return len(beacons), max_d


def main() -> None:
    example = read_scans('example')
    input = read_scans('input')

    assert solve_puzzle(example) == (79, 3621)
    assert solve_puzzle(input) == (405, 12306)


if __name__ == '__main__':
    main()
