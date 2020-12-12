from typing import List


def puzzle1(instructions: List[str]) -> None:
    de, dn = 1, 0
    e, n = 0, 0
    for instruction in instructions:
        code = instruction[0]
        value = int(instruction[1:])
        assert value >= 0
        if code == 'E':
            e += value
        elif code == 'W':
            e -= value
        elif code == 'N':
            n += value
        elif code == 'S':
            n -= value
        elif code in ('L', 'R'):
            assert value % 90 == 0
            if code == 'R':
                value = 360 - value
            while value:
                de, dn = -dn, de
                value -= 90
        elif code == 'F':
            n += dn * value
            e += de * value
    print(abs(n) + abs(e))


def puzzle2(instructions: List[str]) -> None:
    se, sn = 0, 0
    we, wn = 10, 1
    for instruction in instructions:
        code = instruction[0]
        value = int(instruction[1:])
        assert value >= 0
        if code == 'E':
            we += value
        elif code == 'W':
            we -= value
        elif code == 'N':
            wn += value
        elif code == 'S':
            wn -= value
        elif code in ('L', 'R'):
            assert value % 90 == 0
            if code == 'R':
                value = 360 - value
            while value:
                we, wn = -wn, we
                value -= 90
        elif code == 'F':
            sn += wn * value
            se += we * value
    print(abs(sn) + abs(se))


def main() -> None:
    with open('input') as f:
        instructions = f.readlines()

    puzzle1(instructions)
    puzzle2(instructions)


if __name__ == '__main__':
    main()
