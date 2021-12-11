from dataclasses import dataclass
from typing import List


@dataclass
class TreeNode:
    children: List['TreeNode']
    metadata: List[int]


def read_tree(filename: str) -> TreeNode:
    with open(filename) as f:
        numbers = [int(x) for x in f.read().strip().split()]
    it = iter(numbers)

    def aux() -> TreeNode:
        n_child_nodes = next(it)
        n_metadata_entries = next(it)
        children = [
            aux()
            for _ in range(n_child_nodes)
        ]
        metadata = [
            next(it)
            for _ in range(n_metadata_entries)
        ]
        return TreeNode(children=children, metadata=metadata)
    return aux()


def metadata_sum(node: TreeNode) -> int:
    return sum(metadata_sum(child) for child in node.children) + sum(node.metadata)


def value(node: TreeNode) -> int:
    if node.children:
        return sum(
            value(node.children[index - 1])
            for index in node.metadata
            if 1 <= index <= len(node.children)
        )
    else:
        return sum(node.metadata)


example = read_tree('example')
input = read_tree('input')

assert metadata_sum(example) == 138
assert metadata_sum(input) == 42472

assert value(example) == 66
assert value(input) == 21810
