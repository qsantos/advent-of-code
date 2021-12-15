from typing import Callable, List, Tuple, TypeVar

State = List[str]
T = TypeVar('T')


def read_state(filename: str) -> State:
    with open(filename) as f:
        return f.read().strip().split()


def step(state: State) -> State:
    h, w = len(state), len(state[0])
    ret = []
    for i in range(h):
        row = []
        for j in range(w):
            c = sum(
                1
                for a, b in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                if 0 <= a < h and 0 <= b < w
                if state[a][b] == '#'
            )
            if (state[i][j] == '#' and c == 1) or (state[i][j] == '.' and c in (1, 2)):
                bug = '#'
            else:
                bug = '.'
            row.append(bug)
        ret.append(''.join(row))
    return ret


def brent(x0: T, step: Callable[[T], T]) -> Tuple[int, int]:
    power = lam = 1
    tortoise = x0
    hare = x0
    hare = step(hare)

    while tortoise != hare:
        if power == lam:
            tortoise = hare
            power *= 2
            lam = 0
        hare = step(hare)
        lam += 1

    tortoise = x0
    hare = x0

    for i in range(lam):
        hare = step(hare)

    mu = 0
    while tortoise != hare:
        tortoise = step(tortoise)
        hare = step(hare)
        mu += 1

    return lam, mu


def count_bugs(state: State) -> int:
    return sum(row.count('#') for row in state)


def biodiversity_rating(state: State) -> int:
    return sum(
        2**i if c == '#' else 0
        for i, c in enumerate(''.join(state))
    )


def first_repetition(state: State) -> int:
    lam, mu = brent(state, step)
    for _ in range(mu):
        state = step(state)
    return biodiversity_rating(state)


def main() -> None:
    example1 = read_state('example1')
    state = example1
    for count in [8, 16, 12, 13, 10]:
        assert count_bugs(state) == count
        state = step(state)

    assert first_repetition(example1) == 2129920

    challenge = read_state('input')
    print(first_repetition(challenge))


if __name__ == '__main__':
    main()
