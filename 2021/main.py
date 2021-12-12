from typing import Dict, Set

Graph = Dict[str, Set[str]]


def read_graph(filename: str) -> Graph:
    graph: Graph = {}
    with open(filename) as f:
        for line in f:
            a, b = line.strip().split('-')
            if a not in graph:
                graph[a] = set()
            if b not in graph:
                graph[b] = set()
            graph[a].add(b)
            graph[b].add(a)
    return graph


def visit_small_once(graph: Graph) -> int:
    def aux(node: str) -> int:
        if node == 'end':
            return 1
        total = 0
        for neighbor in graph[node]:
            if neighbor == 'start':
                continue
            if neighbor not in seen:
                if neighbor == neighbor.lower():
                    seen.add(neighbor)
                    total += aux(neighbor)
                    seen.remove(neighbor)
                else:
                    total += aux(neighbor)
        return total
    seen: Set[str] = set()
    return aux('start')


def visit_small_twice(graph: Graph) -> int:
    def aux(node: str, visited_twice: bool) -> int:
        if node == 'end':
            return 1
        total = 0
        for neighbor in graph[node]:
            if neighbor == 'start':
                continue
            if neighbor == neighbor.lower():
                if neighbor not in seen:
                    seen.add(neighbor)
                    total += aux(neighbor, visited_twice)
                    seen.remove(neighbor)
                elif not visited_twice:
                    total += aux(neighbor, True)
            else:
                total += aux(neighbor, visited_twice)
        return total
    seen: Set[str] = set()
    return aux('start', False)


def main() -> None:
    example1 = read_graph('example1')
    example2 = read_graph('example2')
    example3 = read_graph('example3')
    input = read_graph('input')

    assert visit_small_once(example1) == 10
    assert visit_small_once(example2) == 19
    assert visit_small_once(example3) == 226
    assert visit_small_once(input) == 5920

    assert visit_small_twice(example1) == 36
    assert visit_small_twice(example2) == 103
    assert visit_small_twice(example3) == 3509
    assert visit_small_twice(input) == 155477


if __name__ == '__main__':
    main()
