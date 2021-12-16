from typing import List, Tuple

Ducts = List[str]


def read_ducts(filename: str) -> Ducts:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def find_start(ducts: Ducts) -> Tuple[int, int]:
    for i, row in enumerate(ducts):
        for j, c in enumerate(row):
            if c == '0':
                return i, j
    assert False


def find_numbers(ducts: Ducts) -> int:
    ret = 0
    for row in ducts:
        for c in row:
            if c in '123456789':
                ret += 1 << (int(c) - 1)
    return ret


def shortest_route(ducts: Ducts, *, return_to_origin: bool = False) -> int:
    start = find_start(ducts)
    all_numbers = find_numbers(ducts)
    rows = len(ducts)
    cols = len(ducts[0])
    assert all(len(row) == cols for row in ducts)
    q = [(start, 0)]
    seen = set()
    steps = 0
    while True:
        next_q = []
        for state in q:
            if state in seen:
                continue
            seen.add(state)
            (i, j), numbers = state
            c = ducts[i][j]
            if c in '123456789':
                numbers |= 1 << (int(c) - 1)
            if numbers == all_numbers and ((i, j) == start or not return_to_origin):
                return steps
            for ni, nj in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
                if 0 <= ni < rows and 0 <= nj < cols and ducts[i][j] != '#':
                    next_q.append(((ni, nj), numbers))
        q = next_q
        steps += 1
    return -1


def main() -> None:
    example = read_ducts('example')
    input = read_ducts('input')

    assert shortest_route(example) == 14
    assert shortest_route(input) == 500

    assert shortest_route(input, return_to_origin=True) == 748


if __name__ == '__main__':
    main()
