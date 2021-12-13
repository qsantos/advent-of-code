def puzzle1(a: int, b: int) -> int:
    count = 0
    for _ in range(40_000_000):
        a = (a * 16807) % 2147483647
        b = (b * 48271) % 2147483647
        if a % 65536 == b % 65536:
            count += 1
    return count


def puzzle2(a: int, b: int) -> int:
    count = 0
    for i in range(5_000_000):
        while True:
            a = (a * 16807) % 2147483647
            if a % 4 == 0:
                break
        while True:
            b = (b * 48271) % 2147483647
            if b % 8 == 0:
                break
        if a & 65535 == b % 65536:
            count += 1
    return count


def main() -> None:
    example = (65, 8921)
    input = (883, 879)

    assert puzzle1(*example) == 588
    assert puzzle1(*input) == 609

    assert puzzle2(*example) == 309
    assert puzzle2(*input) == 253


if __name__ == '__main__':
    main()
