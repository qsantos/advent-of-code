import re


def puzzle1() -> None:
    n_valid_passwords = 0
    with open('input') as f:
        for line in f:
            m = re.match(r'(\d+)-(\d+) ([a-z]): ([a-z]+)$', line)
            assert m is not None
            lowest, highest, letter, password = m.groups()
            n_valid_passwords += int(lowest) <= password.count(letter) <= int(highest)
    print(n_valid_passwords)


def puzzle2() -> None:
    n_valid_passwords = 0
    with open('input') as f:
        for line in f:
            m = re.match(r'(\d+)-(\d+) ([a-z]): ([a-z]+)$', line)
            assert m is not None
            first, second, letter, password = m.groups()
            first_ok = int(first) <= len(password) and password[int(first) - 1] == letter
            second_ok = int(second) <= len(password) and password[int(second) - 1] == letter
            n_valid_passwords += first_ok != second_ok
    print(n_valid_passwords)


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
