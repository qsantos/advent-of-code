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


def puzzle1(buses: List[Tuple[int, int]], earliest: int) -> None:
    offset, bus = min(buses, key=lambda ob: ob[1] - (earliest % ob[1]))
    wait = bus - (earliest % bus)
    print(bus * wait)


def puzzle2(buses: List[Tuple[int, int]]) -> None:
    remainders = [-offset for offset, bus in buses]
    modulos = [bus for offset, bus in buses]
    print(crt(remainders, modulos))


def main() -> None:
    with open('input') as f:
        earliest = int(next(f))
        buses = [
            (offset, int(bus))
            for offset, bus in enumerate(next(f).strip().split(','))
            if bus != 'x'
        ]

    puzzle1(buses, earliest)
    puzzle2(buses)


if __name__ == '__main__':
    main()
