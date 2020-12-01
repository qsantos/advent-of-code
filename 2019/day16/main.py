from itertools import accumulate
from typing import Iterator, List


def parse(s: str) -> List[int]:
    return [int(x) for x in s]


def step(signal: List[int]) -> Iterator[int]:
    n = len(signal)
    sums = list(accumulate([0] + signal))

    # for i in range(1, len(signal) + 1):
    #     pattern = [0] * (i - 1) + [1] * i + [0] * i + [-1] * i + [0]
    #     yield abs(sum(a * b for a, b in zip(signal, cycle(pattern)))) % 10
    for i in range(n):
        digit = 0
        j = 0
        while True:
            # i + 1 - 1 zeroes
            j += i

            # i + 1 ones
            if j >= n:
                break
            digit -= sums[j]
            j = min(n, j + i + 1)
            digit += sums[j]

            # i + 1 zeroes
            j += i + 1

            # i + 1 minus ones
            if j >= n:
                break
            digit += sums[j]
            j = min(n, j + i + 1)
            digit -= sums[j]

            # 1 zero
            j += 1

        yield abs(digit) % 10


def steps(signal: List[int], steps: int) -> List[int]:
    for _ in range(steps):
        signal = list(step(signal))
    return signal


def extract_message(s: str) -> int:
    signal = parse(s * 10000)
    offset = int(s[:7])
    signal = steps(signal, 100)
    return int(''.join(str(x) for x in signal[offset:offset + 8]))


def main() -> None:
    with open('input') as f:
        challenge = f.read().strip()

    # puzzle 1
    assert steps(parse('12345678'), 4) == parse('01029498')
    assert steps(parse('80871224585914546619083218645595'), 100)[:8] == parse('24176176')
    assert steps(parse('19617804207202209144916044189917'), 100)[:8] == parse('73745418')
    assert steps(parse('69317163492948606335995924319873'), 100)[:8] == parse('52432133')
    assert steps(parse(challenge), 100)[:8] == parse('23135243')

    # puzzle 2
    assert extract_message('03036732577212944063491565474664') == 84462026
    assert extract_message('02935109699940807407585447034323') == 78725270
    assert extract_message('03081770884921959731165446850517') == 53553731
    assert extract_message(challenge) == 21130597


if __name__ == '__main__':
    main()
