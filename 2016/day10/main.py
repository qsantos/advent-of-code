import re
from collections import defaultdict
from typing import Dict, Set, Tuple

Assignments = Dict[str, Set[int]]
Rules = Dict[str, Tuple[str, str]]
Instructions = Tuple[Assignments, Rules]

pattern1 = re.compile(r'^value (\d+) goes to (\S+ \d+)$')
pattern2 = re.compile(r'^(bot \d+) gives low to (\S+ \d+) and high to (\S+ \d+)$')


def read_instructions(filename: str) -> Instructions:
    assignments: Assignments = {}
    rules = {}
    with open(filename) as f:
        for line in f:
            line = line.strip()
            if m := pattern1.match(line):
                value, bot = m.groups()
                if bot not in assignments:
                    assignments[bot] = set()
                assignments[bot].add(int(value))
            elif m := pattern2.match(line):
                src, dst_low, dst_high = m.groups()
                rules[src] = (dst_low, dst_high)
            else:
                assert False, line
    return assignments, rules


def run_instructions(instructions: Instructions) -> None:
    assignments, rules = instructions
    assignments = defaultdict(set, assignments)
    while True:
        for src, items in assignments.items():
            if not src.startswith('bot '):
                continue
            if len(items) < 2:
                continue
            dst_low, dst_high = rules[src]
            assert src in assignments
            chips = assignments[src]
            assert len(chips) == 2
            low, high = min(chips), max(chips)
            if (low, high) == (17, 61):
                print(src)
            assignments[dst_low].add(low)
            assignments[dst_high].add(high)
            assignments[src].remove(low)
            assignments[src].remove(high)
            break
        else:
            break
    print(assignments['output 0'], assignments['output 1'], assignments['output 2'])


def main() -> None:
    example = read_instructions('example')
    input = read_instructions('input')

    run_instructions(example)
    run_instructions(input)


if __name__ == '__main__':
    main()
