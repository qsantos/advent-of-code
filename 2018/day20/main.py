from typing import Dict, Set, Tuple

Coord = Tuple[int, int]
Graph = Dict[Coord, Set[Coord]]


def read_regex(filename: str) -> str:
    with open(filename) as f:
        return f.read().strip()


def find_graph(regex: str) -> Graph:
    def move(coord: Coord, dir: str) -> Coord:
        x, y = coord
        if dir == 'N':
            n = x, y - 1
        elif dir == 'S':
            n = x, y + 1
        elif dir == 'W':
            n = x - 1, y
        elif dir == 'E':
            n = x + 1, y
        else:
            assert False
        if coord not in graph:
            graph[coord] = set()
        if n not in graph:
            graph[n] = set()
        graph[coord].add(n)
        graph[n].add(coord)
        return n

    def aux(starts: Set[Coord]) -> Set[Coord]:
        finals: Set[Coord] = set()
        cur = starts
        for c in it:
            if c == '^':
                continue
            elif c == '(':
                cur = aux(cur)
            elif c == '|':
                finals |= cur
                cur = starts
            elif c == ')' or c == '$':
                return finals | cur
            else:
                cur = {move(coord, c) for coord in cur}
        assert False
    graph: Graph = {}
    it = iter(regex)
    aux({(0, 0)})
    return graph


def furthest_room(regex: str) -> int:
    graph = find_graph(regex)
    q = [(0, 0)]
    seen = set()
    d = 0
    ret = 0
    while q:
        next_q = []
        for node in q:
            if node in seen:
                continue
            seen.add(node)
            ret = d
            for neighbor in graph[node]:
                next_q.append(neighbor)
        q = next_q
        d += 1
    return ret


def far_rooms(regex: str, min_doors: int) -> int:
    graph = find_graph(regex)
    q = [(0, 0)]
    seen = set()
    d = 0
    ret = 0
    while q:
        next_q = []
        for node in q:
            if node in seen:
                continue
            seen.add(node)
            if d >= 1000:
                ret += 1
            for neighbor in graph[node]:
                next_q.append(neighbor)
        q = next_q
        d += 1
    return ret


def main() -> None:
    example1 = read_regex('example1')
    example2 = read_regex('example2')
    example3 = read_regex('example3')
    input = read_regex('input')

    assert furthest_room(example1) == 3
    assert furthest_room(example2) == 10
    assert furthest_room(example3) == 18
    assert furthest_room(input) == 3465

    assert far_rooms(input, 1_000) == 7956


if __name__ == '__main__':
    main()
