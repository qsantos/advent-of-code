import re
from typing import List


def read_operations(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


pattern_swap_position = re.compile(r'^swap position (\d+) with position (\d+)$')
pattern_swap_letters = re.compile(r'^swap letter (\S) with letter (\S+)$')
pattern_rotate = re.compile(r'^rotate (left|right) (\d+) steps?$')
pattern_rotate_based = re.compile(r'^rotate based on position of letter (\S)$')
pattern_reverse = re.compile(r'^reverse positions (\d+) through (\d+)$')
pattern_move = re.compile(r'^move position (\d+) to position (\d+)$')


def scramble(operations: List[str], s: str) -> str:
    characters = list(s)
    for operation in operations:
        if m := pattern_swap_position.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            characters[x], characters[y] = characters[y], characters[x]
        elif m := pattern_swap_letters.match(operation):
            x, y = characters.index(m.group(1)), characters.index(m.group(2))
            characters[x], characters[y] = characters[y], characters[x]
        elif m := pattern_rotate.match(operation):
            direction = m.group(1)
            x = int(m.group(2))
            x %= len(characters)
            if direction == 'left':
                characters = characters[x:] + characters[:x]
            else:
                characters = characters[-x:] + characters[:-x]
        elif m := pattern_rotate_based.match(operation):
            letter = m.group(1)
            x = characters.index(letter)
            if x >= 4:
                x += 1
            x += 1
            x %= len(characters)
            characters = characters[-x:] + characters[:-x]
        elif m := pattern_reverse.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            characters[x:y + 1] = characters[x:y + 1][::-1]
        elif m := pattern_move.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            letter = characters.pop(x)
            characters.insert(y, letter)
        else:
            assert False, operation
    return ''.join(characters)


def unscramble(operations: List[str], s: str) -> str:
    characters = list(s)
    for operation in reversed(operations):
        if m := pattern_swap_position.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            characters[x], characters[y] = characters[y], characters[x]
        elif m := pattern_swap_letters.match(operation):
            x, y = characters.index(m.group(1)), characters.index(m.group(2))
            characters[x], characters[y] = characters[y], characters[x]
        elif m := pattern_rotate.match(operation):
            direction = m.group(1)
            x = int(m.group(2))
            x %= len(characters)
            if direction == 'left':
                characters = characters[-x:] + characters[:-x]
            else:
                characters = characters[x:] + characters[:x]
        elif m := pattern_rotate_based.match(operation):
            # yeah, I know
            letter = m.group(1)
            for offset in range(len(characters)):
                before = characters[offset:] + characters[:offset]
                x = before.index(letter)
                if x >= 4:
                    x += 1
                x += 1
                x %= len(characters)
                after = before[-x:] + before[:-x]
                if after == characters:
                    characters = before
                    break
            else:
                assert False
        elif m := pattern_reverse.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            characters[x:y + 1] = characters[x:y + 1][::-1]
        elif m := pattern_move.match(operation):
            x, y = int(m.group(1)), int(m.group(2))
            letter = characters.pop(y)
            characters.insert(x, letter)
        else:
            assert False, operation
    return ''.join(characters)


def main() -> None:
    example = read_operations('example')
    input = read_operations('input')

    assert scramble(example, 'abcde') == 'decab'
    assert scramble(input, 'abcdefgh') == 'baecdfgh'

    assert unscramble(example, 'decab') == 'abcde'
    assert unscramble(input, 'baecdfgh') == 'abcdefgh'
    assert unscramble(input, 'fbgdceah') == 'cegdahbf'


if __name__ == '__main__':
    main()
