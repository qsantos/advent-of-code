from typing import List, Tuple

Grid = List[str]


def read_grid(filename: str) -> Grid:
    with open(filename) as f:
        return f.read().split('\n')


def read_letters(grid: Grid) -> Tuple[str, int]:
    letters = []
    i, j = 0, grid[0].index('|')
    di, dj = 1, 0
    steps = 0
    while True:
        c = grid[i][j]
        if c == '+':
            for ndi, ndj in [(-dj, di), (dj, -di)]:
                ni, nj = i + ndi, j + ndj
                if not len(grid) > ni >= 0 <= nj < len(grid[ni]):
                    continue
                nc = grid[ni][nj]
                if nc != ' ':
                    di, dj = ndi, ndj
                    break
            else:
                assert False
        elif c == ' ':
            break
        elif c == '-' or c == '|':
            pass
        else:
            letters.append(c)
        i += di
        j += dj
        steps += 1
    print(steps)
    return ''.join(letters), steps


def main() -> None:
    example = read_grid('example')
    input = read_grid('input')

    assert read_letters(example) == ('ABCDEF', 38)
    assert read_letters(input) == ('GEPYAWTMLK', 17628)


if __name__ == '__main__':
    main()
