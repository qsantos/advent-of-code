from typing import Iterator, Set

def read_input() -> Iterator[int]:
    with open('input') as f:
        for line in f:
            yield int(line)

def puzzle1() -> None:
    seen: Set[int] = set()
    target = 2020
    for a in read_input():
        b = target - a
        if b in seen:
            print(a * b)
            return
        seen.add(a)
    assert False


def puzzle2() -> None:
    seen: Set[int] = set()
    target = 2020
    for a in read_input():
        for b in seen:
            c = target - a - b
            if c in seen:
                print(a * b * c)
                return
        seen.add(a)
    assert False


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
