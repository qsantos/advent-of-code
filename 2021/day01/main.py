from typing import List


def count_increases(values: List[int]) -> int:
    ret = 0
    it = iter(values)
    last = next(it)
    for cur in it:
        if cur > last:
            ret += 1
        last = cur
    return ret


def count_window_increases(values: List[int]) -> int:
    ret = 0
    cur_window = sum(values[:3])
    last_window = cur_window
    for i in range(3, len(values)):
        last_window = cur_window
        cur_window += values[i] - values[i - 3]
        if cur_window > last_window:
            ret += 1
    return ret


example = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263,
]

data = []
with open('input') as f:
    for line in f:
        data.append(int(line.strip()))


assert count_increases(example) == 7
assert count_increases(data) == 1527

assert count_window_increases(example) == 5
assert count_window_increases(data) == 1575
