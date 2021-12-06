from typing import List


def read_fishes(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(x) for x in f.read().strip().split(',')]


def fishes_after(fishes: List[int], days: int) -> int:
    fishes_per_day = [0] * 9
    for fish in fishes:
        fishes_per_day[fish] += 1
    for _ in range(days):
        prev = fishes_per_day.pop(0)
        fishes_per_day[6] += prev
        fishes_per_day.append(prev)
    return sum(fishes_per_day)


example = read_fishes('example')
input = read_fishes('input')

assert fishes_after(example, 80) == 5934
assert fishes_after(input, 80) == 389726

assert fishes_after(example, 256) == 26984457539
assert fishes_after(input, 256) == 1743335992042
