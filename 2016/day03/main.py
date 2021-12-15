from typing import List, Tuple

Triangle = Tuple[int, int, int]


def read_triangles(filename: str) -> List[Triangle]:
    triangles = []
    with open(filename) as f:
        for line in f:
            a, b, c = (int(x) for x in line.strip().split())
            triangles.append((a, b, c))
    return triangles


def count_possible_triangles1(triangles: List[Triangle]) -> int:
    return sum(
        a + b > c
        for a, b, c in triangles
        for a, b, c in [sorted([a, b, c])]
    )


def count_possible_triangles2(triangles: List[Triangle]) -> int:
    a, b, c = list(zip(*triangles))
    numbers = a + b + c
    count = 0
    for offset in range(0, len(numbers), 3):
        a, b, c = sorted(numbers[offset:offset + 3])
        if a + b > c:
            count += 1
    return count


def main() -> None:
    input = read_triangles('input')
    assert count_possible_triangles1(input) == 982
    assert count_possible_triangles2(input) == 1826


if __name__ == '__main__':
    main()
