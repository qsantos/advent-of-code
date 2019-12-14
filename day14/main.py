from collections import defaultdict
from typing import DefaultDict, Dict, Set, Tuple

Chemical = str
Reactions = Dict[Chemical, Tuple[int, Dict[Chemical, int]]]


def read_reactions(filename: str) -> Reactions:
    r = {}
    with open(filename) as f:
        for line in f:
            sources, product = line.strip().split(' => ')
            requirements = {}
            for requirement in sources.split(', '):
                r_amount, r_chemical = requirement.split(' ')
                requirements[r_chemical] = int(r_amount)
            p_amount, p_chemical = product.split(' ')
            r[p_chemical] = (int(p_amount), requirements)
    return r


def needed_ore(reactions: Reactions, fuel: int) -> int:
    children: DefaultDict[Chemical, Set[Chemical]] = defaultdict(set)
    for p_chemical in reactions:
        _, requirements = reactions[p_chemical]
        for r_chemical in requirements:
            children[r_chemical].add(p_chemical)
    children['FUEL']

    needs: DefaultDict[Chemical, int] = defaultdict(int)
    needs['FUEL'] = fuel
    while children:
        try:
            p_chemical = next(c for c in children if not children[c])
        except StopIteration:
            raise Exception('There is a cycle!')
        del children[p_chemical]

        if p_chemical not in needs:
            continue
        needed = needs[p_chemical]

        if p_chemical == 'ORE':
            return needed

        p_amount, requirements = reactions[p_chemical]
        q = (needed - 1) // p_amount + 1
        for r_chemical in requirements:
            needs[r_chemical] += requirements[r_chemical] * q
            children[r_chemical].remove(p_chemical)

    assert False


def brute_force(reactions: Reactions, ore: int) -> int:
    n = 10**6
    fuel = n * ore // needed_ore(reactions, n)
    while needed_ore(reactions, fuel) <= ore:
        fuel += 1
    return fuel - 1


def main() -> None:
    assert needed_ore(read_reactions('example1'), 1) == 31
    assert needed_ore(read_reactions('example2'), 1) == 165

    example3 = read_reactions('example3')
    assert needed_ore(example3, 1) == 13312
    assert brute_force(example3, 10**12) == 82892753

    example4 = read_reactions('example4')
    assert needed_ore(example4, 1) == 180697
    assert brute_force(example4, 10**12) == 5586022

    example5 = read_reactions('example5')
    assert needed_ore(example5, 1) == 2210736
    assert brute_force(example5, 10**12) == 460664

    challenge = read_reactions('input')

    # puzzle 1
    assert needed_ore(challenge, 1) == 892207

    # puzzle 2
    assert brute_force(challenge, 10**12) == 1935265


if __name__ == '__main__':
    main()
