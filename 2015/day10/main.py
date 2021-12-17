from typing import Iterator, Tuple


def count_cons(s: str) -> Iterator[Tuple[int, str]]:
    cur = ''
    count = 0
    for c in s:
        if c == cur:
            count += 1
        else:
            if count:
                yield count, cur
            cur = c
            count = 1
    if count:
        yield count, cur


def next_sequence(s: str) -> str:
    return ''.join(str(count) + c for count, c in count_cons(s))


input = '3113322113'

assert next_sequence('1') == '11'

assert next_sequence('1') == '11'
assert next_sequence('11') == '21'
assert next_sequence('21') == '1211'
assert next_sequence('1211') == '111221'
assert next_sequence('111221') == '312211'

s = input
for _ in range(40):
    s = next_sequence(s)
assert len(s) == 329356

s = input
for _ in range(50):
    s = next_sequence(s)
assert len(s) == 4666278
