import json
from typing import Any, List, Tuple

Number = Any


def read_numbers(filename: str) -> List[Number]:
    with open(filename) as f:
        return [
            json.loads(line.strip())
            for line in f
        ]


def just_add(a: Number, b: Number) -> Number:
    return [a, b]


def increase_left(n: Number, v: int) -> Number:
    if isinstance(n, int):
        return n + v
    a, b = n
    return [increase_left(a, v), b]


def increase_right(n: Number, v: int) -> Number:
    if isinstance(n, int):
        return n + v
    a, b = n
    return [a, increase_right(b, v)]


def explode(n: Number) -> Tuple[bool, Number]:
    def aux(n: Number, depth: int, exploded: bool, left: int, right: int) \
            -> Tuple[bool, Number, int, int]:
        a, b = n
        if not exploded and isinstance(a, int) and isinstance(b, int) and depth >= 4:
            return True, 0, a, b
        # visit children
        if not exploded and isinstance(a, list):
            exploded, a, left, right = aux(a, depth + 1, exploded, left, 0)
            if exploded and right > 0:
                b = increase_left(b, right)
                right = 0
        if not exploded and isinstance(b, list):
            exploded, b, left, right = aux(b, depth + 1, exploded, 0, right)
            if exploded and left > 0:
                a = increase_right(a, left)
                left = 0
        return exploded, [a, b], left, right
    exploded, n, left, right = aux(n, 0, False, 0, 0)
    return exploded, n


def split(n: Number) -> Tuple[bool, Number]:
    if isinstance(n, int):
        if n >= 10:
            h = n // 2
            return True, [h, n - h]
        return False, n
    a, b = n
    is_split, a = split(a)
    if not is_split:
        is_split, b = split(b)
    return is_split, [a, b]


def reduce_number(number: Number) -> Number:
    while True:
        exploded, number = explode(number)
        if exploded:
            continue
        is_split, number = split(number)
        if is_split:
            continue
        break
    return number


def add(a: Number, b: Number) -> Number:
    return reduce_number(just_add(a, b))


def sum_numbers(numbers: List[Number]) -> Number:
    it = iter(numbers)
    ret = next(it)
    for cur in it:
        ret = add(ret, cur)
    return ret


def magnitude(n: Number) -> int:
    if isinstance(n, int):
        return n
    a, b = n
    return 3 * magnitude(a) + 2 * magnitude(b)


def largest_magnitude_of_sum_of_two(numbers: List[Number]) -> int:
    return max(
        magnitude(add(a, b))
        for a in numbers
        for b in numbers
    )


example1 = read_numbers('example1')
example2 = read_numbers('example2')
example3 = read_numbers('example3')
example4 = read_numbers('example4')
example5 = read_numbers('example5')
input = read_numbers('input')

assert explode([[[[[9, 8], 1], 2], 3], 4]) == (True, [[[[0, 9], 2], 3], 4])
assert explode([7, [6, [5, [4, [3, 2]]]]]) == (True, [7, [6, [5, [7, 0]]]])
assert explode([[6, [5, [4, [3, 2]]]], 1]) == (True, [[6, [5, [7, 0]]], 3])
assert explode([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]) \
    == (True, [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]])
assert explode([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]) \
    == (True, [[3, [2, [8, 0]]], [9, [5, [7, 0]]]])


assert just_add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) \
    == [[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]
assert explode([[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]) \
    == (True, [[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]])
assert explode([[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]]) \
    == (True, [[[[0, 7], 4], [15, [0, 13]]], [1, 1]])
assert split([[[[0, 7], 4], [15, [0, 13]]], [1, 1]]) \
    == (True, [[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]])
assert split([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]) \
    == (True, [[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]])
assert explode([[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]) \
    == (True, [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]])

assert add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) \
    == [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]

assert sum_numbers(example1) == [[[[1, 1], [2, 2]], [3, 3]], [4, 4]]
assert sum_numbers(example2) == [[[[3, 0], [5, 3]], [4, 4]], [5, 5]]
assert sum_numbers(example3) == [[[[5, 0], [7, 4]], [5, 5]], [6, 6]]
assert sum_numbers(example4) == [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]

assert magnitude([9, 1]) == 29
assert magnitude([1, 9]) == 21
assert magnitude([[9, 1], [1, 9]]) == 129
assert magnitude([[1, 2], [[3, 4], 5]]) == 143
assert magnitude([[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]) == 1384
assert magnitude([[[[1, 1], [2, 2]], [3, 3]], [4, 4]]) == 445
assert magnitude([[[[3, 0], [5, 3]], [4, 4]], [5, 5]]) == 791
assert magnitude([[[[5, 0], [7, 4]], [5, 5]], [6, 6]]) == 1137
assert magnitude([[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]) == 3488

assert magnitude(sum_numbers(example5)) == 4140
assert magnitude(sum_numbers(input)) == 4202

assert largest_magnitude_of_sum_of_two(example5) == 3993
assert largest_magnitude_of_sum_of_two(input) == 4779
