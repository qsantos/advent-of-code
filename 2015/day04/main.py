from hashlib import md5


def starts_with_zeros(prefix: str, n_zeroes: int) -> int:
    suffix = 0
    while True:
        digest = md5((prefix + str(suffix)).encode()).hexdigest()
        if digest.startswith('0' * n_zeroes):
            return suffix
        suffix += 1


def main() -> None:
    input = 'iwrupvqb'

    assert starts_with_zeros('abcdef', 5) == 609043
    assert starts_with_zeros('pqrstuv', 5) == 1048970
    assert starts_with_zeros(input, 5) == 346386

    assert starts_with_zeros(input, 6) == 9958218


if __name__ == '__main__':
    main()
