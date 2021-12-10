from itertools import cycle
from typing import List


def read_drifts(filename: str) -> List[int]:
    with open(filename) as f:
        return [
            int(line.strip())
            for line in f
        ]


def first_duplicate(drifts: List[int]) -> int:
    seen = set()
    cur = 0
    for drift in cycle(drifts):
        if cur in seen:
            return cur
        seen.add(cur)
        cur += drift
    assert False


example = read_drifts('example')
input = read_drifts('input')

assert sum(example) == 3
assert sum(input) == 490

assert first_duplicate(example) == 2
assert first_duplicate(input) == 70357
