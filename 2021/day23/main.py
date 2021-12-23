from heapq import heappop, heappush
from typing import Iterator, List, Tuple

State = List[int]

# 0 1   2   3   4   5 6
#     7   8   9  10
#    11  12  13  14
#    15  16  17  18
#    19  20  21  22
graph = {
    # hallway
    0: {(1, 1)},
    1: {(1, 0), (2, 2), (2, 7)},
    2: {(2, 1), (2, 3), (2, 7), (2, 8)},
    3: {(2, 2), (2, 4), (2, 8), (2, 9)},
    4: {(2, 3), (2, 5), (2, 9), (2, 10)},
    5: {(2, 4), (1, 6), (2, 10)},
    6: {(1, 5)},
    # first room layer
    7: {(2, 1), (2, 2), (1, 11)},
    8: {(2, 2), (2, 3), (1, 12)},
    9: {(2, 3), (2, 4), (1, 13)},
    10: {(2, 4), (2, 5), (1, 14)},
    # second room layer
    11: {(1, 7), (1, 15)},
    12: {(1, 8), (1, 16)},
    13: {(1, 9), (1, 17)},
    14: {(1, 10), (1, 18)},
    # third room layer
    15: {(1, 11), (1, 19)},
    16: {(1, 12), (1, 20)},
    17: {(1, 13), (1, 21)},
    18: {(1, 14), (1, 22)},
    # fourth room layer
    19: {(1, 15)},
    20: {(1, 16)},
    21: {(1, 17)},
    22: {(1, 18)},
}

for a, n in graph.items():
    for c, b in n:
        assert (c, a) in graph[b]


def read_amphipods(filename: str, *, unfold: bool = False) -> State:
    with open(filename) as f:
        grid = list(f)
        if unfold:
            grid.insert(3, '  #D#C#B#A#\n')
            grid.insert(4, '  #D#B#A#C#\n')
        return [0] * 7 + [
            '.ABCD'.index(row[j])
            for row in grid[2:-1]
            for j in (3, 5, 7, 9)
        ]


def targets_from(state: State, pos: int) -> Iterator[Tuple[int, int]]:
    v = state[pos]
    if v == 0:
        return
    room = [(7, 11, 15, 19), (8, 12, 16, 20), (9, 13, 17, 21), (10, 14, 18, 22)][v - 1]
    # already in final position
    for i in reversed(room):
        if i >= len(state):
            continue
        elif pos == i:
            return
        elif state[i] != v:
            break
    # in hallway
    if pos < 7:
        # move to final position if possible
        for i in reversed(room):
            if i >= len(state):
                continue
            elif state[i] == v:
                pass
            elif state[i] == 0:
                break
            else:
                # room occupied by wrong amphipod
                return
        else:
            assert False
        candidates = [i]
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
            if n >= len(state):
                continue
            if state[n] != 0:
                continue
            heappush(q, (moves + d, n))


def organize_amphipods(state: State) -> int:
    q = [(0, state)]
    seen = set()
    while q:
        cost, state = heappop(q)
        fstate = tuple(state)
        if fstate in seen:
            continue
        seen.add(fstate)
        if state[7:] == [1, 2, 3, 4] * ((len(state) - 7) // 4):
            return cost
        for pos, v in enumerate(state):
            for (moves, target) in targets_from(state, pos):
                new_cost = cost + moves * [0, 1, 10, 100, 1000][v]
                new_state = list(state)
                new_state[pos], new_state[target] = new_state[target], new_state[pos]
                heappush(q, (new_cost, new_state))
    assert False


example = read_amphipods('example')
input = read_amphipods('input')

assert organize_amphipods(example) == 12521
assert organize_amphipods(input) == 15472

example = read_amphipods('example', unfold=True)
input = read_amphipods('input', unfold=True)

assert organize_amphipods(example) == 44169
assert organize_amphipods(input) == 46182
