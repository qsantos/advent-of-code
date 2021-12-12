from typing import List


def read_offsets(filename: str) -> List[int]:
    with open(filename) as f:
        return [
            int(line.strip())
            for line in f
        ]


def steps1(offsets: List[int]) -> int:
    offsets = list(offsets)
    ip = 0
    steps = 0
    while ip < len(offsets):
        offset = offsets[ip]
        offsets[ip] += 1
        ip += offset
        steps += 1
    return steps


def steps2(offsets: List[int]) -> int:
    offsets = list(offsets)
    ip = 0
    steps = 0
    while ip < len(offsets):
        offset = offsets[ip]
        if offset >= 3:
            offsets[ip] -= 1
        else:
            offsets[ip] += 1
        ip += offset
        steps += 1
    return steps


def main() -> None:
    example = read_offsets('example')
    input = read_offsets('input')

    assert steps1(example) == 5
    assert steps1(input) == 342669

    assert steps2(example) == 10
    assert steps2(input) == 25136209


if __name__ == '__main__':
    main()
