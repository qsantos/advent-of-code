from collections import Counter


def is_valid(x: int) -> bool:
    digits = list(str(x))
    if sorted(digits) != digits:
        return False

    c = Counter(digits)
    return 2 in c.values()


def main() -> None:
    with open('input') as f:
        data = f.read()
    a, b = (int(x) for x in data.strip().split('-'))

    assert is_valid(112233)
    assert not is_valid(123444)
    assert is_valid(111122)

    c = sum(is_valid(i) for i in range(a, b))
    print(c)


if __name__ == '__main__':
    main()
