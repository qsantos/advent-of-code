from collections import deque
from heapq import heappop, heappush
from string import ascii_lowercase, ascii_uppercase
from typing import Deque, FrozenSet, Iterator, List, Set, Tuple

Coord = Tuple[int, int]
Maze = List[List[str]]


def read_maze(filename: str) -> Maze:
    with open(filename) as f:
        return [
            list(row)
            for row in f.read().strip().split('\n')
        ]


def collect_keys1(maze: Maze) -> int:
    h, w = len(maze), len(maze[0])

    # locate entrance
    for i in range(h):
        for j in range(w):
            if maze[i][j] == '@':
                break
        else:
            continue
        break

    all_keys = frozenset({
        maze[i][j]
        for i in range(h)
        for j in range(w)
        if maze[i][j] in ascii_lowercase
    })

    State = Tuple[int, int, FrozenSet[str]]
    q: Deque[Tuple[int, State]] = deque()
    q.append((0, (i, j, frozenset())))
    visited: Set[State] = set()
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        i, j, keys = state
        if keys == all_keys:
            return steps

        for a, b in (i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j):
            if not 0 <= i < h or not 0 <= j < w:
                continue

            c = maze[a][b]
            if c == '#':
                continue
            if c in ascii_uppercase and c.lower() not in keys:
                continue

            if c in ascii_lowercase:
                new_keys = keys | {c}
            else:
                new_keys = keys

            q.append((steps + 1, (a, b, new_keys)))

    return -1


def find_new_keys(maze: Maze, robot: Coord, keys: FrozenSet[str]) -> Iterator[Tuple[int, Coord]]:
    h, w = len(maze), len(maze[0])

    visited: Set[Coord] = set()
    q: Deque[Tuple[int, Coord]] = deque()
    q.append((0, robot))
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        i, j = state
        for a, b in (i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j):
            if not 0 <= i < h or not 0 <= j < w:
                continue

            c = maze[a][b]
            if c == '#':
                continue
            if c in ascii_uppercase and c.lower() not in keys:
                continue

            if c in ascii_lowercase and c not in keys:
                yield steps + 1, (a, b)
            else:
                q.append((steps + 1, (a, b)))


def collect_keys2(maze: Maze) -> int:
    h, w = len(maze), len(maze[0])

    all_keys = frozenset({
        maze[i][j]
        for i in range(h)
        for j in range(w)
        if maze[i][j] in ascii_lowercase
    })

    # locate entrance
    for i in range(h):
        for j in range(w):
            if maze[i][j] == '@':
                break
        else:
            continue
        break

    # change entrances
    assert list('...') == maze[i - 1][j - 1:j + 2]
    assert list('.@.') == maze[i][j - 1:j + 2]
    assert list('...') == maze[i + 1][j - 1:j + 2]
    maze[i - 1][j - 1:j + 2] = list('@#@')
    maze[i][j - 1:j + 2] = list('###')
    maze[i + 1][j - 1:j + 2] = list('@#@')

    State = Tuple[FrozenSet[str], Tuple[Coord, ...]]
    initial_state: State = (frozenset(), (
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ))

    q: List[Tuple[int, State]] = []
    heappush(q, (0, initial_state))
    visited: Set[State] = set()
    while q:
        steps, state = heappop(q)
        if state in visited:
            continue
        visited.add(state)

        keys, coords = state
        if keys == all_keys:
            return steps

        for r in range(4):
            for add_steps, (i, j) in find_new_keys(maze, coords[r], keys):
                new_steps = steps + add_steps
                new_state = (
                    keys | {maze[i][j]},
                    tuple(list(coords)[:r] + [(i, j)] + list(coords)[r + 1:]),
                )
                heappush(q, (new_steps, new_state))

    return -1


def main() -> None:
    challenge = read_maze('input')

    # puzzle 1
    assert collect_keys1(read_maze('example1')) == 8
    assert collect_keys1(read_maze('example2')) == 86
    assert collect_keys1(read_maze('example3')) == 132
    assert collect_keys1(read_maze('example4')) == 136
    assert collect_keys1(read_maze('example5')) == 81
    assert collect_keys1(challenge) == 6162

    # puzzle 2
    assert collect_keys2(read_maze('example6')) == 8
    assert collect_keys2(read_maze('example7')) == 24
    assert collect_keys2(read_maze('example8')) == 32
    assert collect_keys2(read_maze('example9')) == 72
    assert collect_keys2(challenge) == 1556


if __name__ == '__main__':
    main()
