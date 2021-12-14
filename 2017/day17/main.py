from typing import Dict, Iterable, Iterator


class CircularLinkedList:
    nexts: Dict[int, int]
    prevs: Dict[int, int]

    def __init__(self, values: Iterable[int]):
        it = iter(values)
        first = next(it)
        self.nexts = {first: first}
        self.prevs = {first: first}
        prev = first
        for cur in values:
            self.insert_after(prev, cur)
            prev = cur

    def __contains__(self, value: int) -> bool:
        return value in self.nexts

    def __iter__(self) -> Iterator[int]:
        return iter(self.nexts)

    def iter_after(self, start: int) -> Iterator[int]:
        cur = self.nexts[start]
        while cur != start:
            yield cur
            cur = self.nexts[cur]

    def pop_after(self, pos: int) -> int:
        ret = self.nexts[pos]
        n = self.nexts[ret]
        self.nexts[pos] = n
        self.prevs[n] = pos
        del self.nexts[ret]
        del self.prevs[ret]
        return ret

    def insert_after(self, pos: int, value: int) -> None:
        n = self.nexts[pos]
        self.nexts[pos] = value
        self.nexts[value] = n
        self.prevs[n] = value
        self.prevs[value] = pos


def puzzle1(steps: int) -> int:
    buffer = CircularLinkedList([0])
    cur = 0
    for v in range(1, 2018):
        for _ in range(steps):
            cur = buffer.nexts[cur]
        buffer.insert_after(cur, v)
        cur = v
    return buffer.nexts[2017]


def puzzle2(steps: int) -> int:
    index = 0
    after_zero = 0
    for v in range(1, 50_000_001):
        index = (index + steps) % v
        if index == 0:
            after_zero = v
        index += 1
    return after_zero


def main() -> None:
    assert puzzle1(3) == 638
    assert puzzle1(349) == 640
    assert puzzle2(349) == 47949463


if __name__ == '__main__':
    main()
