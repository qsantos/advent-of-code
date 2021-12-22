import re
from typing import List, Tuple


def crt(remainders: List[int], modulos: List[int]) -> int:
    assert len(remainders) == len(modulos)
    N = 1
    for modulo in modulos:
        N *= modulo
    x = 0
    for ai, ni in zip(remainders, modulos):
        Ni = N // ni
        Mi = pow(Ni, -1, ni)
        x += ai * Ni * Mi
        x %= N
    return x


pattern = re.compile(r'^Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).$')


def read_discs(filename: str) -> List[Tuple[int, int]]:
    discs = []
    with open(filename) as f:
        for line in f:
            m = pattern.match(line.strip())
            assert m
            positions, start = m.groups()
            discs.append((int(positions), int(start)))
    return discs


def when_to_press_the_button(discs: List[Tuple[int, int]]) -> int:
    # we want to t such that:
    # all((start + i + 1 + t) % n_pos == 0 for i, (n_pos, start) in enumerate(discs))
    # so we want t == - (start + i + 1) mod n_pos for each i, (n_pos, start) in enumerate(discs)
    remainders = []
    modulos = []
    for i, (n_pos, start) in enumerate(discs):
        remainders.append(- (start + i + 1))
        modulos.append(n_pos)
    t = crt(remainders, modulos)
    assert all((start + i + 1 + t) % n_pos == 0 for i, (n_pos, start) in enumerate(discs))
    return t


def main() -> None:
    example = read_discs('example')
    input = read_discs('input')

    # puzzle 2
    assert when_to_press_the_button(example) == 5
    assert when_to_press_the_button(input) == 148737

    # puzzle 1
    input.append((11, 0))
    assert when_to_press_the_button(input) == 2353212


if __name__ == '__main__':
    main()
