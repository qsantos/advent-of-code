from collections import defaultdict, deque
from string import ascii_uppercase
from typing import DefaultDict, Deque, Dict, List, Set, Tuple

Coord = Tuple[int, int]
Maze = List[str]
Portals = Dict[str, Set[Coord]]


def read_maze(filename: str) -> Maze:
    with open(filename) as f:
        return [row for row in f.read().split('\n') if row]


def reverse_portals(portals: Portals) -> Dict[Coord, str]:
    return {
        v: label
        for label in portals
        for v in portals[label]
    }


def find_portals(maze: Maze) -> Portals:
    h, w = len(maze), len(maze[0])

    portals: DefaultDict[str, Set[Coord]] = defaultdict(set)

    for i in range(h):
        for j in range(w):
            if maze[i][j] != '.':
                continue

            for di, dj in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
                ni, nj = i + di, j + dj
                if not 0 <= ni < h or not 0 <= nj < w:
                    continue
                n = maze[ni][nj]
                if n not in ascii_uppercase:
                    continue

                mi, mj = ni + di, nj + dj
                assert 0 <= mi < h and 0 <= mj < w
                m = maze[i + 2 * di][j + 2 * dj]
                assert m in ascii_uppercase

                if di < 0 or dj < 0:
                    label = m + n
                else:
                    label = n + m

                portals[label].add((i, j))

                break

    return dict(portals)


def solve_maze1(maze: Maze) -> int:
    h, w = len(maze), len(maze[0])

    portals = find_portals(maze)
    assert 'AA' in portals
    assert 'ZZ' in portals

    rev_portals = reverse_portals(portals)

    start, = portals['AA']

    q: Deque[Tuple[int, Coord]] = deque()
    q.append((0, start))
    visited: Set[Coord] = set()
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        i, j = state

        if (i, j) in portals['ZZ']:
            return steps

        try:
            label = rev_portals[i, j]
        except KeyError:
            pass
        else:
            for n in portals[label]:
                if n != (i, j):
                    q.append((steps + 1, n))

        for di, dj in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
            ni, nj = i + di, j + dj

            if not 0 <= ni < h or not 0 <= nj < w:
                continue
            if maze[ni][nj] != '.':
                continue

            q.append((steps + 1, (ni, nj)))

    return -1


def solve_maze2(maze: Maze) -> int:
    h, w = len(maze), len(maze[0])

    portals = find_portals(maze)
    assert 'AA' in portals
    assert 'ZZ' in portals

    rev_portals = reverse_portals(portals)

    start, = portals['AA']

    State = Tuple[int, Coord]
    q: Deque[Tuple[int, State]] = deque()
    q.append((0, (0, start)))
    visited: Set[State] = set()
    while q:
        steps, state = q.popleft()
        if state in visited:
            continue
        visited.add(state)

        depth, (i, j) = state

        if depth == 0 and (i, j) in portals['ZZ']:
            return steps

        if depth > 400:
            return -1

        is_at_outer_border = (i == 2 or i == h - 3 or j == 2 or j == w - 3)

        if depth != 0 or not is_at_outer_border:
            try:
                label = rev_portals[i, j]
            except KeyError:
                pass
            else:
                for n in portals[label]:
                    if n == (i, j):
                        continue

                    if is_at_outer_border:
                        new_depth = depth - 1
                    else:
                        new_depth = depth + 1
                    q.append((steps + 1, (new_depth, n)))

        for di, dj in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
            ni, nj = i + di, j + dj

            if not 0 <= ni < h or not 0 <= nj < w:
                continue
            if maze[ni][nj] != '.':
                continue

            q.append((steps + 1, (depth, (ni, nj))))

    return -1


def main() -> None:
    assert solve_maze1(read_maze('example1')) == 23
    assert solve_maze1(read_maze('example2')) == 58
    assert solve_maze1(read_maze('input')) == 568

    assert solve_maze2(read_maze('example1')) == 26
    assert solve_maze2(read_maze('example2')) == -1
    assert solve_maze2(read_maze('example3')) == 396
    assert solve_maze2(read_maze('input')) == 6546


if __name__ == '__main__':
    main()
