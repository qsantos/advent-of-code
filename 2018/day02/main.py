from collections import Counter
from typing import List, Tuple


def read_box_ids(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def count_doubles_and_trebles(box_ids: List[str]) -> int:
    doubles = 0
    trebles = 0
    for box_id in box_ids:
        counts = set(Counter(box_id).values())
        if 2 in counts:
            doubles += 1
        if 3 in counts:
            trebles += 1
    return doubles * trebles


def find_close_box_ids(box_ids: List[str]) -> str:
    for a in box_ids:
        for b in box_ids:
            common = [ca for ca, cb in zip(a, b) if ca == cb]
            if len(a) - len(common) == 1:
                return ''.join(common)
    assert False


example1 = read_box_ids('example1')
example2 = read_box_ids('example2')
input = read_box_ids('input')

assert count_doubles_and_trebles(example1) == 12
assert count_doubles_and_trebles(input) == 9633

assert find_close_box_ids(example2) == 'fgij'
assert find_close_box_ids(input) == 'lujnogabetpmsydyfcovzixaw'
