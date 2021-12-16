def puzzle1(n_elves: int) -> int:
    def aux(n: int) -> int:
        if n == 1:
            return 0
        q, r = divmod(n, 2)
        if r == 0:
            return 2 * aux(q)
        else:
            return 2 + 2 * aux(q)
    return aux(n_elves) + 1


def puzzle2(n_elves: int) -> int:
    winner = 0
    n = 1
    for n in range(2, n_elves + 1):
        if winner >= n // 2 - 1:
            winner += 2
        else:
            winner += 1
        winner %= n
    return 1 + winner


def main() -> None:
    assert puzzle1(5) == 3
    assert puzzle1(3012210) == 1830117

    print(puzzle2(5))
    print(puzzle2(3012210))


if __name__ == '__main__':
    main()
