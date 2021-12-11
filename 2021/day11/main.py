from copy import deepcopy
from typing import List


def read_octopuses(filename: str) -> List[List[int]]:
    with open(filename) as f:
        return [
            [int(c) for c in line.strip()]
            for line in f
        ]


def step(octopuses: List[List[int]]) -> int:
    q = []
    for i in range(len(octopuses)):
        for j in range(len(octopuses[i])):
            octopuses[i][j] += 1
            if octopuses[i][j] > 9:
                q.append((i, j))

    flashed = set()
    while q:
        i, j = q.pop()
        if (i, j) in flashed:
            continue
        flashed.add((i, j))
        for ni in (i - 1, i, i + 1):
            for nj in (j - 1, j, j + 1):
                if (ni, nj) == (i, j):
                    continue
                if not 0 <= ni < len(octopuses):
                    continue
                if not 0 <= nj < len(octopuses[ni]):
                    continue
                octopuses[ni][nj] += 1
                if octopuses[ni][nj] > 9:
                    q.append((ni, nj))
    for i, j in flashed:
        octopuses[i][j] = 0
    return len(flashed)


def count_flashes(octopuses: List[List[int]], steps: int, log: bool = False) -> int:
    octopuses = deepcopy(octopuses)
    flashes = 0
    if log:
        print('Before any steps:')
        for row in octopuses:
            print(''.join(str(d) for d in row))
        print()
    for step_number in range(steps):
        flashes += step(octopuses)
        if log:
            print(f'After step {step_number + 1}:')
            for row in octopuses:
                print(''.join(str(d) for d in row))
            print()
    return flashes


def first_all_flashes(octopuses: List[List[int]]) -> int:
    octopuses = deepcopy(octopuses)
    n_octopuses = sum(len(row) for row in octopuses)
    n = 1
    while step(octopuses) != n_octopuses:
        n += 1
    return n


example = read_octopuses('example')
input = read_octopuses('input')

assert count_flashes(example, 100) == 1656
assert count_flashes(input, 100) == 1546

assert first_all_flashes(example) == 195
assert first_all_flashes(input) == 471
