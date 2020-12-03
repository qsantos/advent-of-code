def count_slope(dx: int, dy: int) -> int:
    with open('input') as f:
        grid = f.read().strip().split('\n')
    x, y = 0, 0
    c = 0
    while y < len(grid):
        c += grid[y][x] == '#'
        x = (x + dx) % len(grid[0])
        y += dy
    return c


def puzzle1() -> None:
    print(count_slope(3, 1))


def puzzle2() -> None:
    r = 1
    r *= count_slope(1, 1)
    r *= count_slope(3, 1)
    r *= count_slope(5, 1)
    r *= count_slope(7, 1)
    r *= count_slope(1, 2)
    print(r)


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
