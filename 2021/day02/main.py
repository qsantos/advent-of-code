from typing import List, Tuple


def follow_steps(steps: List[str]) -> Tuple[int, int]:
    h, d = 0, 0
    for step in steps:
        instr, param = step.split()
        if instr == 'forward':
            h += int(param)
        elif instr == 'down':
            d += int(param)
        elif instr == 'up':
            d -= int(param)
        else:
            assert False
    return h, d


def follow_steps2(steps: List[str]) -> Tuple[int, int]:
    h, d, aim = 0, 0, 0
    for step in steps:
        instr, param = step.split()
        if instr == 'forward':
            h += int(param)
            d += aim * int(param)
        elif instr == 'down':
            aim += int(param)
        elif instr == 'up':
            aim -= int(param)
        else:
            assert False
    return h, d


example = [
    'forward 5',
    'down 5',
    'forward 8',
    'up 3',
    'down 8',
    'forward 2',
]

with open('input') as f:
    data = [
        line.strip()
        for line in f
    ]

assert follow_steps(example) == (15, 10)
assert follow_steps(data) == (1944, 1049)

assert follow_steps2(example) == (15, 60)
assert follow_steps2(data) == (1944, 954969)
