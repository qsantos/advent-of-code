from typing import Dict, Iterable, List, Tuple

Instruction = Tuple[str, str, str]


def read_instructions(filename: str) -> List[Instruction]:
    instructions = []
    with open(filename) as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) == 2:
                opcode, a = parts
                b = ''
            else:
                opcode, a, b = parts
            instructions.append((opcode, a, b))
    return instructions


def run_instructions(instructions: List[Instruction], inputs: Iterable[int]) -> int:
    inputs = iter(inputs)
    mem: Dict[str, int] = {}
    ip = 0
    while 0 <= ip < len(instructions):
        opcode, a, b = instructions[ip]
        # print(opcode, a, b, mem)
        try:
            vb = int(b)
        except ValueError:
            vb = mem.get(b, 0)
        if opcode == 'inp':
            mem[a] = next(inputs)
        elif opcode == 'add':
            mem[a] = mem.get(a, 0) + vb
        elif opcode == 'mul':
            mem[a] = mem.get(a, 0) * vb
        elif opcode == 'div':
            mem[a] = mem.get(a, 0) // vb
        elif opcode == 'mod':
            mem[a] = mem.get(a, 0) % vb
        elif opcode == 'eql':
            mem[a] = 1 if mem.get(a, 0) == vb else 0
        else:
            assert False, opcode
        ip += 1
    # print(mem['z'])
    return mem['z']


def check_model_number(inputs: Iterable[int]) -> bool:
    inputs = iter(inputs)
    z = 0

    w = next(inputs)
    # w cannot be 12
    # if w != z % 26 + 12:
    #     z = 26 * z + w + 1
    z = w + 1
    # z >= 2

    w = next(inputs)
    # w cannot be 13 or greater
    # if w != z % 26 + 13:
    #     z = 26 * z + w + 9
    z = 26 * z + w + 9
    # z = 26 * (inputs[0] + 1) + (inputs[1] + 9)
    # z >= 2 * 26

    w = next(inputs)
    # w cannot be 12 or greater
    # if w != z % 26 + 12:
    #     z = 26 * z + w + 11
    z = 26 * z + w + 11
    # z = 26 * 26 * (inputs[0] + 1) + 26 * (inputs[1] + 9) + (inputs[2] + 11)
    # z >= 2 * 26 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    # r = inputs[2] + 11
    # z = 26 * (inputs[0] + 1) + (inputs[1] + 9)
    if w != r - 13:
        z = z * 26 + w + 6
    # z >= 2 * 26

    w = next(inputs)
    # w cannot be 11 or greater
    # if w != z % 26 + 11:
    #     z = z * 26 + w + 6
    z = z * 26 + w + 6
    # z >= 2 * 26 * 26

    w = next(inputs)
    # w cannot be 15 or greater
    # if w != z % 26 + 15:
    #     z = 26 * z + w + 13
    z = 26 * z + w + 13
    # z >= 2 * 26 * 26 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    if w != r - 14:
        z = z * 26 + w + 13
    # z >= 2 * 26 * 26

    w = next(inputs)
    # w cannot be 12 or greater
    # if w != z % 26 + 12:
    #     z = 26 * z + w + 5
    z = 26 * z + w + 5
    # z >= 2 * 26 * 26 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    if w != r - 8:
        z = z * 26 + w + 7
    # z >= 2 * 26 * 26

    w = next(inputs)
    # w cannot be 14 or greater
    # if w != z % 26 + 14:
    #     z = 26 * z + w + 2
    z = 26 * z + w + 2
    # z >= 2 * 26 * 26 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    if w != r - 9:
        z = z * 26 + w + 10
    # z >= 2 * 26 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    if w != r - 11:
        z = z * 26 + w + 14
    # z >= 2 * 26

    w = next(inputs)
    z, r = divmod(z, 26)
    if w != r - 6:
        z = z * 26 + w + 7
    # z >= 2

    # no optional if block must be run

    # from below: z must be lower than 26

    w = next(inputs)
    z, r = divmod(z, 26)
    # from below: cannot add anything to z
    if w != r - 5:
        z = z * 26 + w + 1

    # from below: z must be equal to zero

    return z == 0


def check_consistent_rules(instructions: List[Instruction]) -> None:
    # check_model_number comes down to:
    # all((
    #     number[3] == number[2] - 2,
    #     number[6] == number[5] - 1,
    #     number[8] == number[7]  - 3,
    #     number[10] == number[9] - 7,
    #     number[11] == number[4] - 5,
    #     number[12] == number[1] + 3,
    #     number[13] == number[0] - 4,
    # ))
    number = [0] * 14
    for n0 in range(5, 10):
        number[0] = n0
        number[13] = n0 - 4
        for n1 in range(1, 7):
            number[1] = n1
            number[12] = n1 + 3
            for n4 in range(6, 10):
                number[4] = n4
                number[11] = n4 - 5
                for n9 in range(8, 10):
                    number[9] = n9
                    number[10] = n9 - 7
                    for n7 in range(4, 10):
                        number[7] = n7
                        number[8] = n7 - 3
                        for n5 in range(2, 10):
                            number[5] = n5
                            number[6] = n5 - 1
                            for n2 in range(3, 10):
                                number[2] = n2
                                number[3] = n2 - 2
                                assert 0 not in number
                                assert run_instructions(instructions, number) == 0
                                assert check_model_number(number)


def biggest_model_number() -> int:
    number = [0] * 14
    number[0] = 9
    number[13] = number[0] - 4
    number[1] = 6
    number[12] = number[1] + 3
    number[2] = 9
    number[3] = number[2] - 2
    number[4] = 9
    number[11] = number[4] - 5
    number[5] = 9
    number[6] = number[5] - 1
    number[7] = 9
    number[8] = number[7] - 3
    number[9] = 9
    number[10] = number[9] - 7
    return int(''.join(str(d) for d in number))


def smallest_model_number() -> int:
    number = [0] * 14
    number[0] = 5
    number[13] = number[0] - 4
    number[1] = 1
    number[12] = number[1] + 3
    number[2] = 3
    number[3] = number[2] - 2
    number[4] = 6
    number[11] = number[4] - 5
    number[5] = 2
    number[6] = number[5] - 1
    number[7] = 4
    number[8] = number[7] - 3
    number[9] = 8
    number[10] = number[9] - 7
    return int(''.join(str(d) for d in number))


def main() -> None:
    input = read_instructions('input')
    check_consistent_rules(input)
    assert biggest_model_number() == 96979989692495
    assert smallest_model_number() == 51316214181141


if __name__ == '__main__':
    main()
