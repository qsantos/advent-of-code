from typing import List


def read_instructions(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def print_screen(screen: List[List[str]]) -> None:
    for row in screen:
        print(''.join(row))


def follow_instructions(width: int, height: int, instructions: List[str]) -> int:
    screen = [['.'] * width for _ in range(height)]
    for instruction in instructions:
        if instruction.startswith('rect '):
            left, right = instruction[len('rect '):].split('x')
            w, h = int(left), int(right)
            for x in range(w):
                for y in range(h):
                    screen[y][x] = '#'
        elif instruction.startswith('rotate row y='):
            left, right = instruction[len('rotate row y='):].split(' by ')
            y, shift = int(left), int(right)
            screen[y] = screen[y][-shift:] + screen[y][:-shift]
        elif instruction.startswith('rotate column x='):
            left, right = instruction[len('rotate column x='):].split(' by ')
            x, shift = int(left), int(right)
            column = [screen[y][x] for y in range(height)]
            column = column[-shift:] + column[:-shift]
            for y in range(height):
                screen[y][x] = column[y]
        else:
            assert False, instruction
        print_screen(screen)
        print()
    return sum(row.count('#') for row in screen)


def main() -> None:
    input = read_instructions('input')

    example = [
        'rect 3x2',
        'rotate column x=1 by 1',
        'rotate row y=0 by 4',
        'rotate column x=1 by 1',
    ]
    follow_instructions(7, 3, example)
    assert follow_instructions(50, 6, input) == 115


if __name__ == '__main__':
    main()
