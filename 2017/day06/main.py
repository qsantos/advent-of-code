from typing import Tuple

Banks = Tuple[int, ...]


def read_banks(filename: str) -> Banks:
    with open(filename) as f:
        return tuple(int(x) for x in f.read().strip().split())


def redistribution_cycle(banks: Banks) -> Tuple[int, int]:
    seen = {}
    t = sum(banks)
    steps = 0
    while banks not in seen:
        seen[banks] = steps

        n = len(banks)
        i = max(range(n), key=lambda i: (banks[i], -i))
        q, r = divmod(banks[i], n)
        banks = tuple(
            (banks[j] if j != i else 0)
            + q
            + (1 if 0 < (j - i) % n <= r else 0)
            for j in range(n)
        )
        assert sum(banks) == t
        steps += 1
    length = steps - seen[banks]
    return steps - length, length


def redistribution_first_repeat(banks: Banks) -> int:
    first, length = redistribution_cycle(banks)
    return first + length


def redistribution_length(banks: Banks) -> int:
    first, length = redistribution_cycle(banks)
    return length


def main() -> None:
    example = read_banks('example')
    input = read_banks('input')

    assert redistribution_first_repeat(example) == 5
    assert redistribution_first_repeat(input) == 11137

    assert redistribution_length(example) == 4
    assert redistribution_length(input) == 1037


if __name__ == '__main__':
    main()
