from typing import Tuple


Present = Tuple[int, int, int]


def parse_present(s: str) -> Present:
    l, w, h = s.split('x')
    return int(l), int(w), int(h)


def wrapping_paper(present: Present) -> int:
    l, w, h = present
    sides = [l * w, w * h, h * l]
    return 2 * sum(sides) + min(sides)


def ribbon(present: Present) -> int:
    l, w, h = present
    a, b, c = sorted((l, w, h))
    return 2 * (a + b) + l * w * h


def main() -> None:
    with open('input') as f:
        input = [parse_present(line.strip()) for line in f]

    assert wrapping_paper(parse_present('2x3x4')) == 58
    assert wrapping_paper(parse_present('1x1x10')) == 43
    assert sum(wrapping_paper(present) for present in input) == 1586300

    assert ribbon(parse_present('2x3x4')) == 34
    assert ribbon(parse_present('1x1x10')) == 14
    assert sum(ribbon(present) for present in input) == 3737498


if __name__ == '__main__':
    main()
