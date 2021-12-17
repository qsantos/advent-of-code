from heapq import heapify, heappop, heappush
from typing import Dict, Set, Tuple

Graph = Dict[str, Set[Tuple[int, str]]]


def read_graph(filename: str) -> Graph:
    graph: Graph = {}
    with open(filename) as f:
        for line in f:
            a, _, b, _, distance = line.strip().split()
            if a not in graph:
                graph[a] = set()
            if b not in graph:
                graph[b] = set()
            graph[a].add((int(distance), b))
            graph[b].add((int(distance), a))
    return graph


def shortest_hamiltonian_path1(graph: Graph) -> int:
    def aux(cur: str) -> int:
        to_visit.remove(cur)
        ret = min((
            aux(neighbor) + distance
            for distance, neighbor in graph[cur]
            if neighbor in to_visit
        ), default=0)
        to_visit.add(cur)
        return ret
    to_visit = set(graph)
    return min(aux(start) for start in graph)


def shortest_hamiltonian_path2(graph: Graph) -> int:
    to_visit = set(graph)
    q = [(0, start, {start}) for start in graph]
    seen = set()
    heapify(q)
    while q:
        d, cur, visited = heappop(q)
        state = (cur, tuple(sorted(visited)))
        if state in seen:
            continue
        seen.add(state)
        if visited == to_visit:
            return d
        for distance, neighbor in graph[cur]:
            if neighbor not in visited:
                heappush(q, (d + distance, neighbor, visited | {neighbor}))
    assert False


def furthest_from(graph: Graph, node: str) -> Tuple[int, str]:
    def aux(cur: str) -> Tuple[int, str]:
        to_visit.remove(cur)
        candidates = [(0, cur)]
        for distance, neighbor in graph[cur]:
            if neighbor in to_visit:
                d, other = aux(neighbor)
                candidates.append((d + distance, other))
        to_visit.add(cur)
        return max(candidates)
    to_visit = set(graph)
    return aux(node)


def longest_route(graph: Graph) -> int:
    start = next(iter(graph))
    _, start = furthest_from(graph, start)
    d, _ = furthest_from(graph, start)
    return d


example = read_graph('example')
input = read_graph('input')

assert shortest_hamiltonian_path1(example) == 605
assert shortest_hamiltonian_path1(input) == 141

assert shortest_hamiltonian_path2(example) == 605
assert shortest_hamiltonian_path2(input) == 141

assert longest_route(example) == 982
assert longest_route(input) == 736
