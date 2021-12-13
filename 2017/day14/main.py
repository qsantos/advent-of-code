from typing import List, Set, Tuple

Coord = Tuple[int, int]
Grid = Set[Coord]


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


def dense_hash(numbers: List[int]) -> List[int]:
    ret = [0] * 16
    for i in range(16):
        for j in range(16):
            ret[i] ^= numbers[16 * i + j]
    return ret


def knot_hash(s: str, list_size: int = 256) -> bytes:
    lengths = [ord(c) for c in s] + [17, 31, 73, 47, 23]
    numbers = sparse_hash(list_size, lengths, 64)
    return bytes(dense_hash(numbers))


def grid_of_key(key: str) -> Grid:
    return {
        (i, 8 * j + k)
        for i in range(128)
        for j, byte in enumerate(knot_hash(f'{key}-{i}'))
        for k, bit in enumerate(f'{byte:08b}')
        if bit == '1'
    }


def count_squares(key: str) -> int:
    grid = grid_of_key(key)
    return len(grid)


def count_regions(key: str) -> int:
    grid = grid_of_key(key)
    seen = set()
    count = 0
    for start in grid:
        if start in seen:
            continue
        count += 1
        q = [start]
        while q:
            state = q.pop()
            if state in seen:
                continue
            seen.add(state)
            i, j = state
            for n in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
                if n in grid:
                    q.append(n)
    assert len(seen) == len(grid)
    return count


def main() -> None:
    example = 'flqrgnkx'
    input = 'wenycdww'

    assert count_squares(example) == 8108
    assert count_squares(input) == 8226

    assert count_regions(example) == 1242
    assert count_regions(input) == 1128


if __name__ == '__main__':
    main()

