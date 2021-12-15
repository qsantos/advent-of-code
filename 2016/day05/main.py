from hashlib import md5
from typing import Iterator, Tuple


def iter_digits(prefix: str) -> Iterator[Tuple[str, str]]:
    index = 0
    while True:
        digest = md5((prefix + str(index)).encode()).hexdigest()
        if digest[:5] == '00000':
            yield digest[5], digest[6]
        index += 1


def crack_password1(door_id: str) -> str:
    ret = []
    it = iter_digits(door_id)
    for _ in range(8):
        sixth, seventh = next(it)
        print(sixth)
        ret.append(sixth)
    return ''.join(ret)


def crack_password2(door_id: str) -> str:
    ret = ['X'] * 8
    it = iter_digits(door_id)
    n_missing = len(ret)
    while n_missing != 0:
        sixth, seventh = next(it)
        if sixth not in '01234567':
            continue
        pos = int(sixth)
        if ret[pos] != 'X':
            continue
        ret[pos] = seventh
        print(''.join(ret))
        n_missing -= 1
    return ''.join(ret)


def main() -> None:
    assert crack_password1('abc') == '18f47a30'
    assert crack_password1('reyedfim') == 'f97c354d'

    assert crack_password2('abc') == '05ace8e3'
    assert crack_password2('reyedfim') == '863dde27'


if __name__ == '__main__':
    main()
