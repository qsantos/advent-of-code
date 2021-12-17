from typing import List


def count_lit1(instructions: List[str]) -> int:
    lights = [[0] * 1000 for _ in range(1000)]
    for instruction in instructions:
        verb, a, through, b = instruction.rsplit(' ', 3)
        ax, ay = (int(part) for part in a.split(','))
        bx, by = (int(part) for part in b.split(','))
        for x in range(ax, bx + 1):
            for y in range(ay, by + 1):
                if verb == 'turn on':
                    lights[y][x] = 1
                elif verb == 'turn off':
                    lights[y][x] = 0
                elif verb == 'toggle':
                    lights[y][x] = 1 - lights[y][x]
                else:
                    assert False, verb
    return sum(sum(row) for row in lights)


def count_lit2(instructions: List[str]) -> int:
    lights = [[0] * 1000 for _ in range(1000)]
    for instruction in instructions:
        verb, a, through, b = instruction.rsplit(' ', 3)
        ax, ay = (int(part) for part in a.split(','))
        bx, by = (int(part) for part in b.split(','))
        for x in range(ax, bx + 1):
            for y in range(ay, by + 1):
                if verb == 'turn on':
                    lights[y][x] += 1
                elif verb == 'turn off':
                    lights[y][x] = max(0, lights[y][x] - 1)
                elif verb == 'toggle':
                    lights[y][x] += 2
                else:
                    assert False, verb
    return sum(sum(row) for row in lights)


def main() -> None:
    with open('input') as f:
        input = [line.strip() for line in f]

    assert count_lit1(input) == 400410
    assert count_lit2(input) == 15343601


if __name__ == '__main__':
    main()
