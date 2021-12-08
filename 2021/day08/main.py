from itertools import permutations
from typing import Dict, List, Tuple

Puzzle = List[Tuple[List[str], List[str]]]
segments_of_digit = {
    0: 'abcefg',
    1: 'cf',
    2: 'acdeg',
    3: 'acdfg',
    4: 'bcdf',
    5: 'abdfg',
    6: 'abdefg',
    7: 'acf',
    8: 'abcdefg',
    9: 'abcdfg',
}
digit_of_segments = {
    segments: digit
    for digit, segments in segments_of_digit.items()
}


def read_puzzle(filename: str) -> Puzzle:
    ret = []
    with open(filename) as f:
        for line in f:
            a, b = line.strip().split(' | ')
            ret.append((
                a.split(),
                b.split(),
            ))
    return ret


def count_1478(puzzle: Puzzle) -> int:
    return sum(
        1 if len(segments) in (2, 4, 3, 7) else 0
        for _, b in puzzle
        for segments in b
    )


def map_segments(mapping: Dict[int, int], segments: str) -> str:
    return ''.join(sorted(segments.translate(mapping)))


def check_mapping(segments_list: List[str], mapping: Dict[int, int]) -> bool:
    digits = set()
    for segments in segments_list:
        segments = map_segments(mapping, segments)
        try:
            digit = digit_of_segments[segments]
        except KeyError:
            return False
        else:
            digits.add(digit)
    return len(digits) == 10


def solve_mapping(segments_list: List[str]) -> Dict[int, int]:
    for candidate in permutations('abcdefg'):
        mapping = dict((ord(a), ord(b)) for a, b in zip('abcdefg', candidate))
        if check_mapping(segments_list, mapping):
            return mapping
    assert False


def sum_outputs(puzzle: Puzzle) -> int:
    total = 0
    for a, b in puzzle:
        mapping = solve_mapping(a)
        number = 0
        for segments in b:
            segments = map_segments(mapping, segments)
            number *= 10
            number += digit_of_segments[segments]
        total += number
    return total


example1 = read_puzzle('example1')
example2 = read_puzzle('example2')
input = read_puzzle('input')

assert count_1478(example1) == 0
assert count_1478(example2) == 26
assert count_1478(input) == 349

assert sum_outputs(example1) == 5353
assert sum_outputs(example2) == 61229
assert sum_outputs(input) == 1070957
