from typing import Callable, Dict, List, NamedTuple


class Aunt(NamedTuple):
    name: str
    properties: Dict[str, int]


def read_aunts(filename: str) -> List[Aunt]:
    with open(filename) as f:
        aunts = []
        for line in f:
            aunt, properties = line.strip().split(': ', 1)
            d = {}
            for property in properties.split(', '):
                name, value = property.split(': ')
                d[name] = int(value)
            aunts.append(Aunt(name=aunt, properties=d))
    return aunts


aunt_sue = Aunt('Aunt Sue', properties={
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1,
})
aunts = read_aunts('input')


def puzzle1(aunt: Aunt) -> bool:
    return all(
        aunt_sue.properties[p] == aunt.properties[p]
        for p in aunt.properties
    )


def puzzle2(aunt: Aunt) -> bool:
    for p in aunt.properties:
        if p in ('cats', 'trees'):
            if not aunt.properties[p] > aunt_sue.properties[p]:
                return False
        elif p in ('pomeranians', 'goldfish'):
            if not aunt.properties[p] < aunt_sue.properties[p]:
                return False
        else:
            if not aunt_sue.properties[p] == aunt.properties[p]:
                return False
    return True


def find_aunt_sue(predicate: Callable[[Aunt], bool]) -> str:
    candidates = [
        aunt
        for aunt in aunts
        if predicate(aunt)
    ]
    assert len(candidates) == 1, candidates
    return candidates[0].name


assert find_aunt_sue(puzzle1) == 'Sue 103'
assert find_aunt_sue(puzzle2) == 'Sue 405'
