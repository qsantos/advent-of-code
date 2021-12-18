from typing import Tuple

Lights = Tuple[str, ...]


def read_lights(filename: str) -> Lights:
    with open(filename) as f:
        return tuple(
            line.strip()
            for line in f
        )


def do_step(lights: Lights) -> Lights:
    rows = len(lights)
    cols = len(lights[0])
    assert all(len(row) == cols for row in lights)
    new_lights = []
    for i, row in enumerate(lights):
        new_row = []
        for j, s in enumerate(row):
            neighbors = sum(
                lights[ni][nj] == '#'
                for ni in (i - 1, i, i + 1)
                if 0 <= ni < rows
                for nj in (j - 1, j, j + 1)
                if 0 <= nj < cols
                if (ni, nj) != (i, j)
            )
            if s == '#' and neighbors in (2, 3) or s == '.' and neighbors == 3:
                new_s = '#'
            else:
                new_s = '.'
            new_row.append(new_s)
        new_lights.append(''.join(new_row))
    return tuple(new_lights)


def lit_corners(lights: Lights) -> Lights:
    new_lights = list(list(row) for row in lights)
    new_lights[+0][+0] = '#'
    new_lights[+0][-1] = '#'
    new_lights[-1][+0] = '#'
    new_lights[-1][-1] = '#'
    return tuple(''.join(row) for row in new_lights)


def do_steps(lights: Lights, n_steps: int, *, keep_corners_lit: bool = False) -> int:
    for _ in range(n_steps):
        if keep_corners_lit:
            lights = lit_corners(lights)
        lights = do_step(lights)
    if keep_corners_lit:
        lights = lit_corners(lights)
    return sum(row.count('#') for row in lights)


example = read_lights('example')
input = read_lights('input')

assert do_steps(example, 4) == 4
assert do_steps(input, 100) == 821

assert do_steps(example, 5, keep_corners_lit=True) == 17
assert do_steps(input, 100, keep_corners_lit=True) == 886
