from typing import List, Set, Tuple

Coord = Tuple[int, int]
Fold = Tuple[str, int]


def read_folds_and_points(filename: str) -> Tuple[List[Fold], Set[Coord]]:
    folds = []
    points = set()
    with open(filename) as f:
        for line in f:
            if not line.strip():
                break
            x, y = (int(x) for x in line.strip().split(','))
            points.add((x, y))
        for line in f:
            axis, value = line[len('fold along '):].split('=')
            folds.append((axis, int(value)))
    return folds, points


def print_points(points: Set[Coord]) -> None:
    min_x = min(x for x, y in points)
    min_y = min(y for x, y in points)
    max_x = max(x for x, y in points)
    max_y = max(y for x, y in points)
    for y in range(min_y, max_y + 1):
        print(''.join(
            '#' if (x, y) in points else '.'
            for x in range(min_x, max_x + 1)
        ))
    pass


def fold_point(fold: Fold, point: Coord) -> Coord:
    axis, value = fold
    x, y = point
    if axis == 'x':
        if x > value:
            return 2 * value - x, y
    else:
        if y > value:
            return x, 2 * value - y
    return point


def fold(folds: List[Fold], points: Set[Coord]) -> Set[Coord]:
    for fold in folds:
        points = {
            fold_point(fold, point)
            for point in points
        }
    return points


def puzzle1(folds: List[Fold], points: Set[Coord]) -> int:
    return len(fold(folds[:1], points))


def puzzle2(folds: List[Fold], points: Set[Coord]) -> None:
    points = fold(folds, points)
    print_points(points)


def main() -> None:
    example = read_folds_and_points('example')
    input = read_folds_and_points('input')

    assert puzzle1(*example) == 17
    assert puzzle1(*input) == 781

    puzzle2(*input)


if __name__ == '__main__':
    main()
