def length_of_string(s: str) -> int:
    it = iter(s)
    assert next(it) == '"'
    length = 0
    for c in it:
        if c == '\\':
            c = next(it)
            if c == 'x':
                assert next(it) in '0123456789abcdefABCDEF'
                assert next(it) in '0123456789abcdefABCDEF'
        elif c == '"':
            break
        length += 1
    return length


def encoding_delta(s: str) -> int:
    return 2 + s.count('"') + s.count('\\')


with open('input') as f:
    input = [line.strip() for line in f]


assert length_of_string(r'""') == 0
assert length_of_string(r'"abc"') == 3
assert length_of_string(r'"aaa\"aaa"') == 7
assert length_of_string(r'"\x27"') == 1

assert sum(len(string) - length_of_string(string) for string in input) == 1342

assert encoding_delta(r'""') == 4
assert encoding_delta(r'"abc"') == 4
assert encoding_delta(r'"aaa\"aaa"') == 6
assert encoding_delta(r'"\x27"') == 5

assert sum(encoding_delta(string) for string in input) == 2074
