from bisect import bisect_left
from typing import List, Tuple, TypeVar

T = TypeVar('T')


def sorted_contains(haystack: List[T], needle: T) -> bool:
    i = bisect_left(haystack, needle)
    return i < len(haystack) and haystack[i] == needle


def test(values: List[int], target: int) -> Tuple[int, int, int]:
    values.sort()
    a, b = 0, len(values) - 1
    while a < b:
        if values[a] + values[b] > target:
            b -= 1
        elif sorted_contains(values, target - (values[a] + values[b])):
            return values[a], values[b], target - (values[a] + values[b])
        else:
            a += 1
    assert False


def main() -> None:
    values = [1, 3, 4, 5]
    target = 8
    print(test(values, target))


if __name__ == '__main__':
    main()
