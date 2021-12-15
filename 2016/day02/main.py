from typing import Dict, List


# 1 2 3
# 4 5 6
# 7 8 9
keypad1 = {
    #     UDLR
    '1': '1412',
    '2': '2513',
    '3': '3623',
    '4': '1745',
    '5': '2846',
    '6': '3956',
    '7': '4778',
    '8': '5879',
    '9': '6989',
}

#     1
#   2 3 4
# 5 6 7 8 9
#   A B C
#     D
keypad2 = {
    #     UDLR
    '1': '1311',
    '2': '1623',
    '3': '1724',
    '4': '4834',
    '5': '5556',
    '6': '2A57',
    '7': '3B68',
    '8': '4C79',
    '9': '9989',
    'A': '6AAB',
    'B': '7DAC',
    'C': '8CBC',
    'D': 'BDDD',
}


def read_instructions(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def bathroom_code(instructions: List[str], keypad: Dict[str, str]) -> str:
    digits = []
    digit = '5'
    for moves in instructions:
        for move in moves:
            digit = keypad[digit]['UDLR'.index(move)]
        digits.append(digit)
    return ''.join(digits)


def main() -> None:
    example = read_instructions('example')
    input = read_instructions('input')

    assert bathroom_code(example, keypad1) == '1985'
    assert bathroom_code(input, keypad1) == '18843'

    assert bathroom_code(example, keypad2) == '5DB3'
    assert bathroom_code(input, keypad2) == '67BB9'


if __name__ == '__main__':
    main()
