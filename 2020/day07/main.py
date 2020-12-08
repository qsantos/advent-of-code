import re
from collections import defaultdict
from typing import DefaultDict, Dict


def main() -> None:
    # parse graph
    parents = defaultdict(set)
    children: DefaultDict[str, Dict[str, int]] = defaultdict(dict)
    with open('input') as f:
        for line in f:
            m = re.fullmatch(r'(.*?) bags contain (.*)\.\n', line)
            assert m
            bag, subbags = m.groups()
            for subbag in subbags.split(', '):
                if subbag == 'no other bags':
                    continue
                m = re.fullmatch(r'([0-9]+) (.*?) bags?', subbag)
                assert m
                str_count, child = m.groups()
                parents[child].add(bag)
                children[bag][child] = int(str_count)

    # puzzle 1
    ascendants = {'shiny gold'}
    while True:
        new_ascendants = ascendants | {parent for bag in ascendants for parent in parents[bag]}
        if new_ascendants == ascendants:
            break
        ascendants = new_ascendants
    print(len(ascendants) - 1)

    # puzzle 2
    total = 0
    level = {'shiny gold': 1}
    while level:
        sublevel: DefaultDict[str, int] = defaultdict(int)
        for bag, count in level.items():
            for child, mult in children[bag].items():
                sublevel[child] += count * mult
        level = sublevel
        total += sum(level.values())
    print(total)


if __name__ == '__main__':
    main()
