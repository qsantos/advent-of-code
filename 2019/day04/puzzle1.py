def is_valid(x: int) -> bool:
    digits = list(str(x))
    if sorted(digits) != digits:
        return False
    if len(set(digits)) == 6:
        return False
    return True


def main() -> None:
    with open('input') as f:
        data = f.read()
    a, b = (int(x) for x in data.strip().split('-'))

    assert is_valid(111111)
    assert not is_valid(223450)
    assert not is_valid(123789)

    c = sum(is_valid(i) for i in range(a, b))
    print(c)


if __name__ == '__main__':
    main()
