from typing import List


def read_ips(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def supports_tls(ip: str) -> bool:
    has_abba = False
    prev2 = ''
    prev1 = ''
    prev0 = ''
    in_square_brackets = False
    for c in ip:
        if c == '[':
            assert not in_square_brackets
            in_square_brackets = True
        elif c == ']':
            assert in_square_brackets
            in_square_brackets = False
        elif c == prev2 and prev0 == prev1 and c != prev0:
            if in_square_brackets:
                return False
            else:
                has_abba = True
        prev0, prev1, prev2 = c, prev0, prev1
    return has_abba


def supports_ssl(ip: str) -> bool:
    abas = set()
    babs = set()
    prev1 = ''
    prev0 = ''
    in_square_brackets = False
    for c in ip:
        if c == '[':
            assert not in_square_brackets
            in_square_brackets = True
        elif c == ']':
            assert in_square_brackets
            in_square_brackets = False
        elif c == prev1:
            if in_square_brackets:
                bab = c + prev0 + c
                aba = prev0 + c + prev0
                if aba in abas:
                    return True
                babs.add(bab)
            else:
                aba = c + prev0 + c
                bab = prev0 + c + prev0
                if bab in babs:
                    return True
                abas.add(aba)
        prev0, prev1 = c, prev0
    return False


def main() -> None:
    input = read_ips('input')

    assert supports_tls('abba[mnop]qrst')
    assert not supports_tls('abcd[bddb]xyyx')
    assert not supports_tls('aaaa[qwer]tyui')
    assert supports_tls('ioxxoj[asdfgh]zxcvbn')
    assert sum(supports_tls(ip) for ip in input) == 110

    assert supports_ssl('aba[bab]xyz')
    assert not supports_ssl('xyx[xyx]xyx')
    assert supports_ssl('aaa[kek]eke')
    assert supports_ssl('zazbz[bzb]cdb')
    assert sum(supports_ssl(ip) for ip in input) == 242


if __name__ == '__main__':
    main()
