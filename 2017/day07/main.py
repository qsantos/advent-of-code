from typing import Dict, Set, Tuple

Nodes = Dict[str, Tuple[int, Set[str]]]
Parents = Dict[str, str]
Tree = Tuple[Nodes, Parents]


def read_tree(filename: str) -> Tree:
    nodes = {}
    parents = {}
    with open(filename) as f:
        for line in f:
            line = line.strip()
            if ' -> ' in line:
                left, right = line.split(' -> ')
                children = set(right.split(', '))
            else:
                left = line
                children = set()
            name, number = left[:-len(')')].split(' (')
            for child in children:
                parents[child] = name
            nodes[name] = (int(number), children)
    return nodes, parents


def get_root(tree: Tree) -> str:
    nodes, parents = tree
    node = next(iter(parents))
    while node in parents:
        node = parents[node]
    return node


def get_total_weights(tree: Tree) -> Dict[str, int]:
    def aux(node: str) -> int:
        weight, children = nodes[node]
        total = weight + sum(aux(child) for child in children)
        total_weights[node] = total
        return total

    total_weights: Dict[str, int] = {}
    nodes, parents = tree
    root = get_root(tree)
    aux(root)
    return total_weights


def fixed_weight(tree: Tree) -> int:
    total_weights = get_total_weights(tree)
    nodes, parents = tree

    def aux(node: str, expected_weight: int) -> int:
        total_weight = total_weights[node]
        assert total_weight != expected_weight
        weight, children = nodes[node]
        weights = [
            total_weights[child]
            for child in children
        ]
        weight_set = set(weights)
        if len(weight_set) == 1:
            return expected_weight - sum(weights)
        else:
            a, b = weight_set
            if weights.count(a) == 1:
                correct_weight, incorrect_weight = b, a
            else:
                correct_weight, incorrect_weight = a, b
            incorrect_child = next(
                child
                for child in children
                if total_weights[child] == incorrect_weight
            )
            return aux(incorrect_child, correct_weight)

    root = get_root(tree)
    return aux(root, 0)


def main() -> None:
    example = read_tree('example')
    input = read_tree('input')

    assert get_root(example) == 'tknk'
    assert get_root(input) == 'dgoocsw'

    assert fixed_weight(example) == 60
    assert fixed_weight(input) == 1275


if __name__ == '__main__':
    main()
