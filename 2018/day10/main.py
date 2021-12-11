import re
from typing import List, Tuple

State = Tuple[int, int, int, int]


def read_states(filename: str) -> List[State]:
    pattern = re.compile(r'^position=<\s*(-?\d+),\s+(-?\d+)> velocity=<\s*(-?\d+),\s+(-?\d+)>\n$')
    states = []
    with open(filename) as f:
        for line in f:
            m = pattern.match(line)
            assert m is not None
            x, y, vx, vy = [int(g) for g in m.groups()]
            states.append((x, y, vx, vy))
    return states


def print_states(states: List[State]) -> None:
    points = {
        (x, y)
        for x, y, _, _ in states
    }
    min_x = min(x for x, y in points)
    min_y = min(y for x, y in points)
    max_x = max(x for x, y in points)
    max_y = max(y for x, y in points)
    if max_x - min_x > 200:
        return
    if max_y - min_y > 200:
        return
    print()
    for y in range(min_y, max_y + 1):
        print(''.join(
            '#' if (x, y) in points else '.'
            for x in range(min_x, max_x + 1)
        ))


def step(states: List[State]) -> None:
    for i, (x, y, vx, vy) in enumerate(states):
        states[i] = (x + vx, y + vy, vx, vy)


example = read_states('example')
# print_states(example)
# for _ in range(10):
#     print()
#     step(example)
#     print_states(example)

input = read_states('input')
print_states(input)
time = 0
while True:
    time += 1
    step(input)
    print(time)
    print_states(input)
