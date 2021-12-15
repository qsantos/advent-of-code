from collections import defaultdict
from typing import DefaultDict, List, Tuple

Claim = Tuple[int, int, int, int, int]


def read_claims(filename: str) -> List[Claim]:
    claims = []
    with open(filename) as f:
        for line in f:
            # line = '#1 @ 1,3: 4x4\n'
            identifier, _, corner, size = line.strip().split()
            x, y = corner[:-1].split(',')
            w, h = size.split('x')
            claims.append((int(identifier[1:]), int(x), int(y), int(w), int(h)))
    return claims


def count_overlaps(claims: List[Claim]) -> int:
    counts: DefaultDict[Tuple[int, int], int] = defaultdict(int)
    for _, x, y, w, h in claims:
        for rx in range(x, x + w):
            for ry in range(y, y + h):
                counts[rx, ry] += 1
    return sum(
        count > 1
        for count in counts.values()
    )


def overlap(a: Claim, b: Claim) -> bool:
    _, ax, ay, aw, ah = a
    _, bx, by, bw, bh = b
    if ax + aw <= bx or bx + bw <= ax:
        return False
    if ay + ah <= by or by + bh <= ay:
        return False
    return True


def not_overlapping(claims: List[Claim]) -> int:
    for a in claims:
        if not any(overlap(a, b) for b in claims if a != b):
            return a[0]
    assert False


example = read_claims('example')
input = read_claims('input')


assert count_overlaps(example) == 4
assert count_overlaps(input) == 111485

assert not_overlapping(example) == 3
assert not_overlapping(input) == 113
