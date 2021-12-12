def is_valid1(passphrase: str) -> bool:
    words = passphrase.split()
    return len(words) == len(set(words))


def is_valid2(passphrase: str) -> bool:
    words = [tuple(sorted(word)) for word in passphrase.split()]
    return len(words) == len(set(words))


def main() -> None:
    with open('input') as f:
        passphrases = [line.strip() for line in f]

    assert is_valid1('aa bb cc dd ee')
    assert not is_valid1('aa bb cc dd aa')
    assert is_valid1('aa bb cc dd aaa')
    assert sum(is_valid1(passphrase) for passphrase in passphrases) == 466

    assert is_valid2('abcde fghij')
    assert not is_valid2('abcde xyz ecdab')
    assert is_valid2('a ab abc abd abf abj')
    assert is_valid2('iiii oiii ooii oooi oooo')
    assert not is_valid2('oiii ioii iioi iiio')
    assert sum(is_valid2(passphrase) for passphrase in passphrases) == 251


if __name__ == '__main__':
    main()
