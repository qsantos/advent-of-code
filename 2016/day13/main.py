def is_wall(designer_number: int, x: int, y: int) -> bool:
    bits = f'{x * x + 3 * x + 2 * x * y + y + y * y  + designer_number:b}'
    return bits.count('1') % 2 == 1


def go_to(designer_number: int, x: int, y: int) -> int:
    q = [(1, 1)]
    target = (x, y)
    seen = set()
    steps = 0
    while q:
        next_q = []
        for state in q:
            if state in seen:
                continue
            seen.add(state)
            if state == target:
                return steps
            x, y = state
            for nx, ny in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]:
                if nx >= 0 and ny >= 0 and not is_wall(designer_number, nx, ny):
                    next_q.append((nx, ny))
        q = next_q
        steps += 1
    assert False


def explore(designer_number: int, max_steps: int) -> int:
    q = [(1, 1)]
    seen = set()
    for _ in range(max_steps + 1):
        next_q = []
        for state in q:
            if state in seen:
                continue
            seen.add(state)
            x, y = state
            for nx, ny in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]:
                if nx >= 0 and ny >= 0 and not is_wall(designer_number, nx, ny):
                    next_q.append((nx, ny))
        q = next_q
    return len(seen)


def main() -> None:
    assert go_to(10, 7, 4) == 11
    assert go_to(1350, 31, 39) == 92

    assert explore(1350, 50) == 124


if __name__ == '__main__':
    main()
