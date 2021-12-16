exhcnage_0_and_1 = str.maketrans('01', '10')


def dragon_step(s: str) -> str:
    return s + '0' + s[::-1].translate(exhcnage_0_and_1)


def checksum(s: str) -> str:
    while len(s) % 2 == 0:
        ret = []
        for i in range(0, len(s), 2):
            if s[i] == s[i + 1]:
                ret.append('1')
            else:
                ret.append('0')
        s = ''.join(ret)
    return s


def repeat_dragon_and_checksum(s: str, length: int) -> str:
    while len(s) < length:
        print(len(s) / length)
        s = dragon_step(s)
    return checksum(s[:length])


def main() -> None:
    assert dragon_step('1') == '100'
    assert dragon_step('0') == '001'
    assert dragon_step('0') == '001'
    assert dragon_step('11111') == '11111000000'
    assert dragon_step('111100001010') == '1111000010100101011110000'

    assert checksum('110010110100') == '100'
    assert repeat_dragon_and_checksum('10000', 20) == '01100'

    input = '10111100110001111'
    assert repeat_dragon_and_checksum(input, 272) == '11100110111101110'
    assert repeat_dragon_and_checksum(input, 35651584) == '10001101010000101'


if __name__ == '__main__':
    main()
