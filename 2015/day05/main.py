from string import ascii_lowercase


def is_nice1(s: str) -> bool:
    if sum(s.count(c) for c in 'aeiou') < 3:
        return False
    if any(ss in s for ss in ('ab', 'cd', 'pq', 'xy')):
        return False
    return any(2 * c in s for c in ascii_lowercase)


def is_nice2(s: str) -> bool:
    for i in range(len(s) - 2):
        ss = s[i: i + 2]
        try:
            s.index(ss, i + 2)
        except ValueError:
            continue
        else:
            break
    else:
        return False

    for i in range(len(s) - 2):
        a, _, c = s[i], s[i + 1], s[i + 2]
        if a == c:
            break
    else:
        return False

    return True


def main() -> None:
    with open('input') as f:
        input = [line.strip() for line in f]

    assert is_nice1('ugknbfddgicrmopn')
    assert is_nice1('aaa')
    assert not is_nice1('jchzalrnumimnmhp')
    assert not is_nice1('haegwjzuvuyypxyu')
    assert not is_nice1('dvszwmarrgswjxmb')
    assert sum(is_nice1(string) for string in input) == 255

    assert is_nice2('qjhvhtzxzqqjkmpb')
    assert is_nice2('xxyxx')
    assert not is_nice2('uurcxstgmygtbstg')
    assert not is_nice2('ieodomkazucvgmuy')
    assert sum(is_nice2(string) for string in input) == 55


if __name__ == '__main__':
    main()
