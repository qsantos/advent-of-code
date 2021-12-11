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


def play(n_players: int, last_marble: int) -> int:
    circle = CircularLinkedList([0, 2, 1])
    cur = 2
    player_scores = {player: 0 for player in range(n_players)}
    for marble in range(3, last_marble + 1):
        if marble % 23 == 0:
            for _ in range(8):
                cur = circle.prevs[cur]
            player = marble % n_players
            player_scores[player] += marble + circle.pop_after(cur)
            cur = circle.nexts[cur]
        else:
            cur = circle.nexts[cur]
            circle.insert_after(cur, marble)
            cur = marble
    return max(player_scores.values())


# players, points of last marble, expected high score
examples = [
    (9, 25, 32),
    (10, 1618, 8317),
    (13, 7999, 146373),
    (17, 1104, 2764),
    (21, 6111, 54718),
    (30, 5807, 37305),
]
input = (452, 70784, 384892)

for n_players, last_marble, high_score in examples + [input]:
    assert play(n_players, last_marble) == high_score

n_players, last_marble, _ = input
assert play(n_players, last_marble * 100) == 3169872331
