from collections import deque
from hashlib import md5
from typing import Deque, Iterator, Tuple


def prng(seed: str, stretching: int = 0) -> Iterator[Tuple[int, str]]:
    index = 0
    while True:
        s = seed + str(index)
        for _ in range(stretching + 1):
            s = md5(s.encode()).hexdigest()
        yield index, s
        index += 1


def iter_keys(seed: str, stretching: int = 0) -> Iterator[int]:
    it = prng(seed, stretching)
    q: Deque[Tuple[int, str]] = deque()
    for _ in range(1000):
        q.append(next(it))
    while True:
        index, candidate = q.popleft()
        q.append(next(it))
        try:
            offset = min(
                candidate.index(digit * 3)
                for digit in '0123456789abcdef'
                if digit * 3 in candidate
            )
        except ValueError:
            continue
        digit = candidate[offset]
        if any(digit * 5 in other for _, other in q):
            yield index


def last_key(seed: str, stretching: int = 0) -> int:
    it = iter_keys(seed, stretching)
    for _ in range(64):
        key = next(it)
    return key


def main() -> None:
    assert next(iter_keys('abc')) == 39
    assert last_key('abc') == 22728
    assert last_key('yjdafjpo') == 25427

    assert next(iter_keys('abc', 2016)) == 10
    assert last_key('abc', 2016) == 22551
    assert last_key('yjdafjpo', 2016) == 22045


if __name__ == '__main__':
    main()
