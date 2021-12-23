from heapq import heappop, heappush
from typing import Iterator, Tuple

State = Tuple[int, ...]


# 0 1   2   3   4   5 6
#     7   9  11  13
#     8  10  12  14
graph = {
    0: {(1, 1)},
    1: {(1, 0), (2, 2), (2, 7)},
    2: {(2, 1), (2, 3), (2, 7), (2, 9)},
    3: {(2, 2), (2, 4), (2, 9), (2, 11)},
    4: {(2, 3), (2, 5), (2, 11), (2, 13)},
    5: {(2, 4), (1, 6), (2, 13)},
    6: {(1, 5)},
    7: {(2, 1), (2, 2), (1, 8)},
    8: {(1, 7)},
    9: {(2, 2), (2, 3), (1, 10)},
    10: {(1, 9)},
    11: {(2, 3), (2, 4), (1, 12)},
    12: {(1, 11)},
    13: {(2, 4), (2, 5), (1, 14)},
    14: {(1, 13)},
}


def read_amphipods(filename: str) -> State:
    with open(filename) as f:
        grid = list(f)
        return (0,) * 7 + (
            '.ABCD'.index(grid[2][3]), '.ABCD'.index(grid[3][3]),
            '.ABCD'.index(grid[2][5]), '.ABCD'.index(grid[3][5]),
            '.ABCD'.index(grid[2][7]), '.ABCD'.index(grid[3][7]),
            '.ABCD'.index(grid[2][9]), '.ABCD'.index(grid[3][9]),
        )


def targets_from(state: State, pos: int) -> Iterator[Tuple[int, int]]:
    v = state[pos]
    if v == 0:
        return
    a, b = [(), (7, 8), (9, 10), (11, 12), (13, 14)][v]
    # already in final position
    if pos == b:
        return
    elif pos == a and state[b] == v:
        return
    # in hallway
    elif pos < 7:
        # move to final position if possible
        if state[a] != 0:
            return
        elif state[b] == 0:
            candidates = [b]
        elif state[b] == v:
            candidates = [a]
        else:
            return
    # wrong room
    else:
        # might move anywhere in hallway
        candidates = [0, 1, 2, 3, 4, 5, 6]
    q = [(0, pos)]
    seen = set()
    while q:
        moves, pos = heappop(q)
        if pos in seen:
            continue
        seen.add(pos)
        if pos in candidates:
            yield moves, pos
        for d, n in graph[pos]:
            if state[n] != 0:
                continue
            heappush(q, (moves + d, n))


def organize_amphipods(state: State) -> int:
    q = [(0, state)]
    seen = set()
    while q:
        cost, state = heappop(q)
        if state in seen:
            continue
        seen.add(state)
        if state[7:] == (1, 1, 2, 2, 3, 3, 4, 4):
            return cost
        for pos, v in enumerate(state):
            for (moves, target) in targets_from(state, pos):
                new_cost = cost + moves * [0, 1, 10, 100, 1000][v]
                tmp_state = list(state)
                tmp_state[pos], tmp_state[target] = tmp_state[target], tmp_state[pos]
                new_state = tuple(tmp_state)
                heappush(q, (new_cost, new_state))
    assert False


example = read_amphipods('example')
input = read_amphipods('input')

assert organize_amphipods(example) == 12521
assert organize_amphipods(input) == 15472
