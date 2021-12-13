from typing import List


def sparse_hash(list_size: int, lengths: List[int], n_rounds: int) -> List[int]:
    numbers = list(range(list_size))
    current_position = 0
    skip_size = 0
    for _ in range(n_rounds):
        for length in lengths:
            for i in range(length // 2):
                a = (current_position + i) % len(numbers)
                b = (current_position + length - 1 - i) % len(numbers)
                numbers[a], numbers[b] = numbers[b], numbers[a]
            current_position += length + skip_size
            skip_size += 1
    return numbers


def single_round(s: str, list_size: int = 256) -> int:
    lengths = [int(x) for x in s.split(',')]
    numbers = sparse_hash(list_size, lengths, 1)
    a, b, *_ = numbers
    return a * b


def dense_hash(numbers: List[int]) -> List[int]:
    ret = [0] * 16
    for i in range(16):
        for j in range(16):
            ret[i] ^= numbers[16 * i + j]
    return ret


def to_hexa(numbers: List[int]) -> str:
    return bytes(numbers).hex()


def knot_hash(s: str, list_size: int = 256) -> str:
    lengths = [ord(c) for c in s] + [17, 31, 73, 47, 23]
    numbers = sparse_hash(list_size, lengths, 64)
    return to_hexa(dense_hash(numbers))


def main() -> None:
    example = '3,4,1,5'
    input = '225,171,131,2,35,5,0,13,1,246,54,97,255,98,254,110'

    assert single_round(example, 5) == 12
    assert single_round(input) == 23874

    assert knot_hash(input) == 'e1a65bfb5a5ce396025fab5528c25a87'


if __name__ == '__main__':
    main()
