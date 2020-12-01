from typing import Dict, Optional, Set

Node = str
Tree = Dict[Node, Node]
ROOT = 'COM'


def read_orbits(filename: str) -> Tree:
    parents: Tree = {}
    with open(filename) as f:
        for line in f:
            parent, child = line.strip().split(')')
            assert child not in parents
            parents[child] = parent
    return parents


def depth(parents: Tree, node: Node) -> int:
    d = 0
    while node != ROOT:
        node = parents[node]
        d += 1
    return d


def tree_weight(parents: Tree) -> int:
    return sum(depth(parents, node) for node in parents)


def distance(parents: Tree, a: Node, b: Node) -> int:
    a = parents[a]
    b = parents[b]
    da = depth(parents, a)
    db = depth(parents, b)
    r = abs(da - db)
    for _ in range(da, db):
        b = parents[b]
    for _ in range(db, da):
        a = parents[a]
    while a != b:
        a = parents[a]
        b = parents[b]
        r += 2
    return r


def main() -> None:
    parents = read_orbits('example1')
    assert tree_weight(parents) == 42

    parents = read_orbits('example2')
    assert distance(parents, 'YOU', 'SAN') == 4
    assert distance(parents, 'YOU', 'YOU') == 0
    assert distance(parents, 'YOU', 'L') == 0

    parents = read_orbits('input')
    print(sum(depth(parents, node) for node in parents))
    print(distance(parents, 'YOU', 'SAN'))


if __name__ == '__main__':
    main()
