from typing import List


def read_polymer(filename: str) -> str:
    with open(filename) as f:
        return f.read().strip()


def reduce_polymer(polymer: str) -> int:
    units: List[str] = []
    for unit in polymer:
        if units and unit != units[-1] and unit.upper() == units[-1].upper():
            units.pop()
        else:
            units.append(unit)
    return len(''.join(units))


def improved_reduce_polymer(polymer: str) -> int:
    units = set(polymer.upper())
    return min(
        reduce_polymer(polymer.replace(unit, '').replace(unit.lower(), ''))
        for unit in units
    )


example = read_polymer('example')
input = read_polymer('input')

assert reduce_polymer(example) == 10
assert reduce_polymer(input) == 11042

assert improved_reduce_polymer(example) == 4
assert improved_reduce_polymer(input) == 6872
