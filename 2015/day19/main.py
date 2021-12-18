from typing import List, Tuple

Molecule = str
Replacements = List[Tuple[str, Molecule]]
Problem = Tuple[Replacements, Molecule]


def parse_molecule(s: str) -> Molecule:
    return s


def read_problem(filename: str) -> Problem:
    with open(filename) as f:
        replacements = []
        for line in f:
            if not line.strip():
                break
            left, right = line.strip().split(' => ')
            before = left
            after = right
            replacements.append((before, after))
        ref = parse_molecule(next(f).strip())
        return replacements, ref


def count_distinct_molecules_after_one_step(problem: Problem) -> int:
    replacements, start = problem
    return len({
        start[:i] + b + start[i + len(a):]
        for i, element in enumerate(start)
        for a, b in replacements
        if start[i:i + len(a)] == a
    })


def steps_to_target(problem: Problem) -> int:
    replacements, target = problem
    molecule = target
    steps = 0
    while molecule != 'e':
        for a, b in replacements:
            try:
                i = molecule.index(b)
            except ValueError:
                pass
            else:
                molecule = molecule[:i] + a + molecule[i + len(b):]
                break
        else:
            break
        steps += 1
    return steps


example1 = read_problem('example1')
example2 = read_problem('example2')
input = read_problem('input')

assert count_distinct_molecules_after_one_step(example1) == 4
assert count_distinct_molecules_after_one_step(example2) == 7
assert count_distinct_molecules_after_one_step(input) == 518

assert steps_to_target(example1) == 3
assert steps_to_target(example2) == 6
assert steps_to_target(input) == 200
