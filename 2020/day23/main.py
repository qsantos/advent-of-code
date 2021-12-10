from typing import Dict, Iterable, Iterator, List


class LinkedList:
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


def read_cups(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(c) for c in f.read().strip()]


def play_game1(cups: List[int], n_moves: int) -> str:
    ll = LinkedList(cups)
    cur = cups[0]
    for move in range(n_moves):
        a = ll.pop_after(cur)
        b = ll.pop_after(cur)
        c = ll.pop_after(cur)
        label = cur - 1
        while label not in ll:
            label -= 1
            if label <= 0:
                label = max(ll)
        ll.insert_after(label, a)
        ll.insert_after(a, b)
        ll.insert_after(b, c)
        cur = ll.nexts[cur]
    a = ll.nexts[1]
    b = ll.nexts[a]
    return ''.join(str(d) for d in ll.iter_after(1))


def play_game2(cups: List[int], n_moves: int) -> int:
    ll = LinkedList(cups + list(range(max(cups) + 1, 1_000_001)))
    cur = cups[0]
    for move in range(n_moves):
        a = ll.pop_after(cur)
        b = ll.pop_after(cur)
        c = ll.pop_after(cur)
        label = cur - 1
        while label not in ll:
            label -= 1
            if label <= 0:
                label = max(ll)
        ll.insert_after(label, a)
        ll.insert_after(a, b)
        ll.insert_after(b, c)
        cur = ll.nexts[cur]
    a = ll.nexts[1]
    b = ll.nexts[a]
    return a * b


example = read_cups('example')
input = read_cups('input')

assert play_game1(example, 10) == '92658374'
assert play_game1(example, 100) == '67384529'
assert play_game1(input, 100) == '75893264'

assert play_game2(example, 10_000_000) == 149245887792
assert play_game2(input, 10_000_000) == 38162588308
