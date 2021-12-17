# NOTE: solving this problems relies on two unspoken assumpetions that reduce
#       it to a 15-puzzle (look for assumption #1 and #2)
import re
from typing import Dict, Tuple

Coord = Tuple[int, int]
Nodes = Dict[Coord, Tuple[int, int]]

pattern_df_line = re.compile(r'^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+\d+T\s+\d+%$')


def read_nodes(filename: str) -> Nodes:
    nodes = {}
    with open(filename) as f:
        next(f)  # 'root@ebhq-gridcenter# df -h\n'
        next(f)  # 'Filesystem              Size  Used  Avail  Use%\n'
        for line in f:
            m = pattern_df_line.match(line.strip())
            assert m is not None, line.strip()
            x, y, size, used = m.groups()
            nodes[int(x), int(y)] = (int(size), int(used))
    return nodes


def count_viable_pairs(nodes: Nodes) -> int:
    return sum(
        a != b and 0 < used_a <= size_b - used_b
        for a, (size_a, used_a) in nodes.items()
        for b, (size_b, used_b) in nodes.items()
    )


def use_initial_target_and_source(nodes: Nodes, t: Coord, s: Coord) -> int:
    steps = 0
    nodes = dict(nodes)
    size_t, used_t = nodes[t]
    size_s, used_s = nodes[s]

    steps += 1
    nodes[t] = (size_t, used_t + used_s)
    nodes[s] = (size_s, 0)

    movable_threshold = 100

    largest_used_movable = max(
        used
        for size, used in nodes.values()
        if used <= movable_threshold
    )
    smallest_size_movable = max(
        size
        for size, used in nodes.values()
        if used <= movable_threshold
    )
    # puzzle assumption #2
    assert largest_used_movable < smallest_size_movable

    movable_nodes = {
        node
        for node, (size, used) in nodes.items()
        if used <= movable_threshold
    }

    goal = (max(x for x, y in nodes), 0)
    start = (goal, s)
    q = [start]
    seen = set()
    while q:
        next_q = []
        for state in q:
            if state in seen:
                continue
            seen.add(state)
            goal, empty = state
            if goal == (0, 0):
                return steps
            gx, gy = goal
            ex, ey = empty
            if abs(gx - ex) + abs(gy - ey) == 1:
                next_q.append((empty, goal))
            for n in [(ex - 1, ey), (ex, ey - 1), (ex, ey + 1), (ex + 1, ey)]:
                if n != goal and n in movable_nodes:
                    next_q.append((goal, n))
        q = next_q
        steps += 1
    assert False


def move_data(nodes: Nodes) -> int:
    # find the single node that can receive data
    targetable_nodes = set()
    for b, (size_b, used_b) in nodes.items():
        for a, (size_a, used_a) in nodes.items():
            if a != b and 0 < used_a <= size_b - used_b:
                targetable_nodes.add(b)
                break
    # puzzle assumption #1
    assert len(targetable_nodes) == 1
    t, = targetable_nodes

    # find first nodes that can be emptied
    source_nodes = []
    size_t, used_t = nodes[t]
    tx, ty = t
    for s in [(tx - 1, ty), (tx, ty - 1), (tx, ty + 1), (tx + 1, ty)]:
        if s not in nodes:
            continue
        size_s, used_s = nodes[s]
        if used_s <= size_t - used_t:
            source_nodes.append(s)
    assert source_nodes

    return min(
        use_initial_target_and_source(nodes, t, s)
        for s in source_nodes
    )


def main() -> None:
    example = read_nodes('example')
    input = read_nodes('input')

    assert count_viable_pairs(input) == 901
    assert move_data(example) == 7
    assert move_data(input) == 238


if __name__ == '__main__':
    main()
