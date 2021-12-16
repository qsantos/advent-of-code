from hashlib import md5
from typing import Tuple


def open_doors_of_room(salt: str, path: str) -> Tuple[bool, bool, bool, bool]:
    digest = md5((salt + path).encode()).hexdigest()
    u, d, l, r = (c in 'bcdef' for c in digest[:4])
    return u, d, l, r


def find_path(salt: str, *, shortest: bool = True) -> str:
    q = [('', 0, 0)]
    steps = 0
    seen = set()
    longest_path = ''
    while q:
        next_q = []
        for state in q:
            if state in seen:
                continue
            seen.add(state)
            path, x, y = state
            if (x, y) == (3, 3):
                if shortest:
                    return path
                else:
                    longest_path = max(path, longest_path, key=len)
                    continue
            u, d, l, r = open_doors_of_room(salt, path)
            if u and y > 0:
                next_q.append((path + 'U', x, y - 1))
            if d and y < 3:
                next_q.append((path + 'D', x, y + 1))
            if l and x > 0:
                next_q.append((path + 'L', x - 1, y))
            if r and x < 3:
                next_q.append((path + 'R', x + 1, y))
        q = next_q
        steps += 1
    return longest_path


def main() -> None:
    input = 'hhhxzeay'

    assert find_path('hijkl') == ''
    assert find_path('ihgpwlah') == 'DDRRRD'
    assert find_path('kglvqrro') == 'DDUDRLRRUDRD'
    assert find_path('ulqzkmiv') == 'DRURDRUDDLLDLUURRDULRLDUUDDDRR'
    assert find_path(input) == 'DDRUDLRRRD'

    assert len(find_path(input, shortest=False)) == 398


if __name__ == '__main__':
    main()
