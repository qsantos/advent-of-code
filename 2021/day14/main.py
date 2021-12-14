from typing import Dict, Tuple

Problem = Tuple[str, Dict[str, str]]
Pairs = Dict[str, int]


def read_template_rules(filename: str) -> Problem:
    with open(filename) as f:
        polymer = next(f).strip()
        assert next(f).strip() == ''
        rules = {}
        for line in f:
            left, right = line.strip().split(' -> ')
            assert left not in rules
            rules[left] = right
    return polymer, rules


def pairs_of_template(template: str) -> Pairs:
    pairs: Pairs = {}
    it = iter(template)
    prev = next(it)
    for cur in it:
        pair = prev + cur
        pairs[pair] = pairs.get(pair, 0) + 1
        prev = cur
    return pairs


def transform(rules: Dict[str, str], pairs: Pairs) -> Pairs:
    next_pairs: Pairs = {}
    for pair, count in pairs.items():
        if pair in rules:
            a, c = pair[0], pair[1]
            b = rules[pair]
            next_pairs[a + b] = next_pairs.get(a + b, 0) + count
            next_pairs[b + c] = next_pairs.get(b + c, 0) + count
        else:
            next_pairs[pair] = next_pairs.get(pair, 0) + count
    return next_pairs


def repeat_transform(problem: Problem, n_steps: int) -> int:
    polymer, rules = problem
    pairs = pairs_of_template(polymer)
    for step in range(n_steps):
        pairs = transform(rules, pairs)
    counts: Dict[str, int] = {}
    for pair, count in pairs.items():
        element = pair[0]
        counts[element] = counts.get(element, 0) + count
    counts[polymer[-1]] = counts.get(polymer[-1], 0) + 1
    return max(counts.values()) - min(counts.values())


def main() -> None:
    example = read_template_rules('example')
    input = read_template_rules('input')

    assert repeat_transform(example, 10) == 1588
    assert repeat_transform(input, 10) == 2891

    assert repeat_transform(example, 40) == 2188189693529
    assert repeat_transform(input, 40) == 4607749009683


if __name__ == '__main__':
    main()
