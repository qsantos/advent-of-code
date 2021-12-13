from typing import Dict, Set

Graph = Dict[int, Set[int]]


def read_pipes(filename: str) -> Graph:
    graph: Graph = {}
    with open(filename) as f:
        for line in f:
            left, right = line.split(' <-> ')
            node = int(left)
            if node not in graph:
                graph[node] = set()
            for part in right.split(', '):
                neighbor = int(part)
                graph[node].add(neighbor)
                if neighbor not in graph:
                    graph[neighbor] = {node}
    return graph


def component_size(graph: Graph, start: int) -> int:
    q = [start]
    seen = set()
    while q:
        node = q.pop()
        if node in seen:
            continue
        seen.add(node)
        for neighbor in graph[node]:
            q.append(neighbor)
    return len(seen)


def count_components(graph: Graph) -> int:
    seen = set()
    components = 0
    for start in graph:
        if start in seen:
            continue
        components += 1
        q = [start]
        while q:
            node = q.pop()
            if node in seen:
                continue
            seen.add(node)
            for neighbor in graph[node]:
                q.append(neighbor)
    return components


def main() -> None:
    example = read_pipes('example')
    input = read_pipes('input')

    assert component_size(example, 0) == 6
    assert component_size(input, 0) == 239

    assert count_components(example) == 2
    assert count_components(input) == 215


if __name__ == '__main__':
    main()
