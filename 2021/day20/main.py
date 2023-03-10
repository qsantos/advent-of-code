from typing import Set, Tuple

Coord = Tuple[int, int]
Problem = Tuple[str, Set[Coord]]


def read_problem(filename: str) -> Problem:
    with open(filename) as f:
        algorithm = next(f).strip()
        next(f)
        pixels = {
            (x, y)
            for y, line in enumerate(f)
            for x, c in enumerate(line.strip())
            if c == '#'
        }
        return algorithm, pixels


def print_pixels(pixels: Set[Coord]) -> None:
    min_x = min(x for x, y in pixels)
    min_y = min(y for x, y in pixels)
    max_x = max(x for x, y in pixels)
    max_y = max(y for x, y in pixels)
    for y in range(min_y, max_y + 1):
        print(''.join(
            '#' if (x, y) in pixels else '.'
            for x in range(min_x, max_x + 1)
        ))


def do_steps(problem: Problem, n_steps: int) -> Set[Coord]:
    algorithm, pixels = problem
    outside = '0'
    for _ in range(n_steps):
        min_x = min(x for x, y in pixels)
        min_y = min(y for x, y in pixels)
        max_x = max(x for x, y in pixels)
        max_y = max(y for x, y in pixels)
        pixels = {
            (x, y)
            for x in range(min_x - 2, max_x + 3)
            for y in range(min_y - 2, max_y + 3)
            if algorithm[int(''.join(
                ('1' if (sx, sy) in pixels else '0')
                if min_x <= sx <= max_x and min_y <= sy <= max_y else outside
                for sy in range(y - 1, y + 2)
                for sx in range(x - 1, x + 2)
            ), 2)] == '#'
        }
        outside = '1' if algorithm[int(''.join(outside * 9), 2)] == '#' else '0'
    return pixels


def main() -> None:
    example = read_problem('example')
    input = read_problem('input')

    assert len(do_steps(example, 2)) == 35
    assert len(do_steps(input, 2)) == 4964
    assert len(do_steps(input, 50)) == 13202


if __name__ == '__main__':
    main()
