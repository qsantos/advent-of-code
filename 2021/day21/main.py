from itertools import product
from collections import Counter, defaultdict
from typing import DefaultDict, Tuple


def run_game(positions: Tuple[int, int]) -> int:
    pos1, pos2 = positions
    score1, score2 = 0, 0
    die = 1
    rolls = 0
    while True:
        for _ in range(3):
            pos1 = 1 + (pos1 + die - 1) % 10
            die = 1 + (die + 1 - 1) % 100
            rolls += 1
        score1 += pos1
        if score1 >= 1000:
            break
        for _ in range(3):
            pos2 = 1 + (pos2 + die - 1) % 10
            die = 1 + (die + 1 - 1) % 100
            rolls += 1
        score2 += pos2
        if score2 >= 1000:
            break
    return min(score1, score2) * rolls


def run_game2(positions: Tuple[int, int]) -> int:
    pos1, pos2 = positions
    counts: DefaultDict[Tuple[int, int, int, int], int] = defaultdict(int)
    counts[pos1, pos2, 0, 0] = 1
    still_playing = True
    offsets = Counter(a + b + c for a, b, c in product((1, 2, 3), repeat=3)).most_common()
    while still_playing:
        new_counts: DefaultDict[Tuple[int, int, int, int], int] = defaultdict(int)
        still_playing = False
        for state, count in counts.items():
            pos1, pos2, score1, score2 = state
            if score1 >= 21 or score2 >= 21:
                new_counts[state] += count
                continue
            still_playing = True
            for offset1, subcount1 in offsets:
                npos1 = 1 + (pos1 + offset1 - 1) % 10
                nscore1 = score1 + npos1
                if nscore1 >= 21:
                    new_counts[npos1, pos2, nscore1, score2] += count * subcount1
                    continue
                for offset2, subcount2 in offsets:
                    npos2 = 1 + (pos2 + offset2 - 1) % 10
                    nscore2 = score2 + npos2
                    new_counts[npos1, npos2, nscore1, nscore2] += count * subcount1 * subcount2
        counts = new_counts
    wins1 = sum(counts for (pos1, pos2, score1, score2), counts in counts.items() if score1 >= 21)
    wins2 = sum(counts for (pos1, pos2, score1, score2), counts in counts.items() if score2 >= 21)
    return max(wins1, wins2)


example = (4, 8)
input = (5, 10)

assert run_game(example) == 739785
assert run_game(input) == 711480
assert run_game2(example) == 444356092776315
assert run_game2(input) == 265845890886828
