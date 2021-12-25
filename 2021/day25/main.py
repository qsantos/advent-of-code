from typing import Set, Tuple

Coord = Tuple[int, int]
SeaCucumbers = Set[Coord]
Problem = Tuple[int, int, SeaCucumbers, SeaCucumbers]


def read_sea_cucumbers(filename: str) -> Problem:
    with open(filename) as f:
        east = set()
        south = set()
        for i, line in enumerate(f):
            for j, c in enumerate(line.strip()):
                if c == '>':
                    east.add((i, j))
                elif c == 'v':
                    south.add((i, j))
        return i + 1, j + 1, east, south


def print_problem(problem: Problem) -> None:
    rows, cols, east, south = problem
    for i in range(rows):
        print(''.join(
            '>' if (i, j) in east else
            'v' if (i, j) in south else
            '.'
            for j in range(cols)
        ))


def count_steps(problem: Problem) -> int:
    rows, cols, east, south = problem
    steps = 0
    moved = True
    while moved:
        # print(f'After {steps} steps')
        # print_problem((rows, cols, east, south))
        # print()
        moved = False
        new_east = set()
        for pos in east:
            i, j = pos
            new_pos = (i, (j + 1) % cols)
            if new_pos in east or new_pos in south:
                new_east.add(pos)
            else:
                moved = True
                new_east.add(new_pos)
        east = new_east
        new_south = set()
        for pos in south:
            i, j = pos
            new_pos = ((i + 1) % rows, j)
            if new_pos in east or new_pos in south:
                new_south.add(pos)
            else:
                moved = True
                new_south.add(new_pos)
        south = new_south
        steps += 1
    return steps


example = read_sea_cucumbers('example')
input = read_sea_cucumbers('input')

assert count_steps(example) == 58
assert count_steps(input) == 367
