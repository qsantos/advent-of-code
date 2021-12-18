from typing import List


def read_containers(filename: str) -> List[int]:
    with open(filename) as f:
        return [
            int(line.strip())
            for line in f
        ]


def count_combinations_to(containers: List[int], target: int) -> int:
    def aux(i: int, cur: int) -> int:
        if cur == target:
            return 1
        elif cur > target:
            return 0
        elif i == len(containers):
            return 0
        container = containers[i]
        return aux(i + 1, cur) + aux(i + 1, cur + container)
    return aux(0, 0)


def count_min_combinations_to(containers: List[int], target: int) -> int:
    def aux(i: int, cur: int, rem: int) -> int:
        if cur == target:
            return 1
        elif cur > target:
            return 0
        elif rem == 0:
            return 0
        elif i == len(containers):
            return 0
        container = containers[i]
        return aux(i + 1, cur, rem) + aux(i + 1, cur + container, rem - 1)
    for n_containers in range(len(containers) + 1):
        r = aux(0, 0, n_containers)
        if r > 0:
            return r
    return 0


example = read_containers('example')
input = read_containers('input')

assert count_combinations_to(example, 25) == 4
assert count_combinations_to(input, 150) == 1304

assert count_min_combinations_to(example, 25) == 3
assert count_min_combinations_to(input, 150) == 18
