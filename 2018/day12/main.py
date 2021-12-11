from typing import Set, Tuple


def read_state_and_rules(filename: str) -> Tuple[Set[int], Set[str]]:
    with open(filename) as f:
        line = next(f)
        # line = 'initial state: #..#.#..##......###...###\n'
        state = {
            i
            for i, c in enumerate(line.strip()[len('initial state: '):])
            if c == '#'
        }
        next(f)  # skip empty line
        rules = set()
        for line in f:
            before, after = line.strip().split(' => ')
            if after == '#':
                rules.add(before)
        return state, rules


def print_state(state: Set[int], start: int) -> None:
    print(''.join(
        '#' if i in state else '.'
        for i in range(start, max(state) + 1)
    ))


def next_step(state: Set[int], rules: Set[str]) -> Set[int]:
    return {
        pot
        for pot in range(min(state) - 2, max(state) + 2)
        if ''.join('#' if i in state else '.' for i in range(pot - 2, pot + 3)) in rules
    }


def count_after_n_steps(state: Set[int], rules: Set[str], n_steps: int) -> int:
    for step in range(n_steps):
        old_state = state
        state = next_step(state, rules)
        if state == {i + 1 for i in old_state}:
            break
    remaining_steps = n_steps - step - 1
    return sum(state) + remaining_steps * len(state)


example = read_state_and_rules('example')
input = read_state_and_rules('input')

assert count_after_n_steps(*example, 20) == 325
assert count_after_n_steps(*input, 20) == 3337

assert count_after_n_steps(*input, 50_000_000_000) == 4300000000349
