from collections import deque
from typing import List, Set

preamble_size = 25


def is_sum_of_two(values: Set[int], target: int) -> bool:
    return any(target - value in values for value in values)


def find_invalid(values: List[int]) -> int:
    preamble = deque(values[:preamble_size], maxlen=preamble_size)
    rest = values[preamble_size:]
    for value in rest:
        if not is_sum_of_two(set(preamble), value):
            return value
        preamble.append(value)
    assert False


def find_span(values: List[int], target: int) -> List[int]:
    for i in range(len(values)):
        s = 0
        for j in range(i + 1, len(values)):
            s += values[j]
            if s == target:
                return values[i:j]
    assert False


def main() -> None:
    with open('input') as f:
        values = [int(line) for line in f]

    # puzzle 1
    invalid = find_invalid(values)
    print(invalid)

    # puzzle 2
    span = find_span(values, invalid)
    print(min(span) + max(span))


if __name__ == '__main__':
    main()
