from string import ascii_lowercase


def is_valid_password(s: str) -> bool:
    if not any(ascii_lowercase[i: i + 3] in s for i in range(len(ascii_lowercase) - 2)):
        return False
    if set(s) & set('iol'):
        return False
    if sum(c * 2 in s for c in ascii_lowercase) < 2:
        return False
    return True


def next_string(s: str) -> str:
    i = len(s) - 1
    while s[i] == 'z':
        i -= 1
    return s[:i] + ascii_lowercase[ascii_lowercase.index(s[i]) + 1] + 'a' * (len(s) - i - 1)


def next_password(s: str) -> str:
    s = next_string(s)
    while not is_valid_password(s):
        s = next_string(s)
    return s


assert not is_valid_password('hijklmmn')
assert not is_valid_password('abbceffg')
assert not is_valid_password('abbcegjk')

assert next_string('a') == 'b'
assert next_string('az') == 'ba'
assert next_string('abczz') == 'abdaa'

assert next_password('abcdefgh') == 'abcdffaa'
assert next_password('ghijklmn') == 'ghjaabcc'

assert next_password('hxbxwxba') == 'hxbxxyzz'
assert next_password('hxbxxyzz') == 'hxcaabcc'
