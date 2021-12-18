import re
from itertools import permutations
from typing import Dict, Tuple

Graph = Dict[str, Dict[str, int]]

pattern = re.compile(
    r'^(\S+) would (lose|gain) (\d+) happiness units? by sitting next to (\S+)\.$',
)


def read_graph(filename: str) -> Graph:
    graph: Graph = {}
    with open(filename) as f:
        for line in f:
            m = pattern.match(line.strip())
            assert m is not None, line.strip()
            a, lose_gain, p, b = m.groups()
            if a not in graph:
                graph[a] = {}
            points = int(p)
            graph[a][b] = points if lose_gain == 'gain' else -points
    return graph


def total_change_in_happiness(graph: Graph, seating: Tuple[str, ...]) -> int:
    it = iter(seating)
    prev = next(it)
    total = 0
    for cur in it:
        total += graph[prev][cur] + graph[cur][prev]
        prev = cur
    cur = seating[0]
    total += graph[prev][cur] + graph[cur][prev]
    return total


def optimal_seating_arrangement(graph: Graph) -> int:
    it = iter(graph)
    start = next(it)
    others = tuple(it)
    return max(
        total_change_in_happiness(graph, (start, ) + seating)
        for seating in permutations(others)
    )


example = read_graph('example')
input = read_graph('input')

assert total_change_in_happiness(example, ('Alice', 'Bob', 'Carol', 'David')) == 330

assert optimal_seating_arrangement(example) == 330
assert optimal_seating_arrangement(input) == 733

for node in input:
    input[node]['me'] = 0
input['me'] = {node: 0 for node in input}
assert optimal_seating_arrangement(input) == 725
