from typing import List


def read_moves(filename: str) -> List[str]:
    with open(filename) as f:
        return f.read().split(', ')


def distance_of_moves(moves: List[str]) -> int:
    x, y = 0, 0
    dx, dy = 0, -1
    for move in moves:
        r = move[0]
        d = int(move[1:])
        if r == 'L':
            dx, dy = dy, -dx
        else:
            dx, dy = -dy, dx
        x += d * dx
        y += d * dy
    return abs(x) + abs(y)


def distance_of_first_seen_twice(moves: List[str]) -> int:
    x, y = 0, 0
    dx, dy = 0, -1
    seen = set()
    for move in moves:
        r = move[0]
        d = int(move[1:])
        if r == 'L':
            dx, dy = dy, -dx
        else:
            dx, dy = -dy, dx
        for _ in range(d):
            if (x, y) in seen:
                return abs(x) + abs(y)
            seen.add((x, y))
            x += dx
            y += dy
    assert False


def main() -> None:
    input = read_moves('input')

    assert distance_of_moves('R2, L3'.split(', ')) == 5
    assert distance_of_moves('R2, R2, R2'.split(', ')) == 2
    assert distance_of_moves('R5, L5, R5, R3'.split(', ')) == 12
    assert distance_of_moves(input) == 230

    assert distance_of_first_seen_twice('R8, R4, R4, R8'.split(', ')) == 4
    assert distance_of_first_seen_twice(input) == 154


if __name__ == '__main__':
    main()
