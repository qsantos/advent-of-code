from typing import Dict, List, NamedTuple, Optional, Tuple

Mem = Dict[str, int]


class Condition(NamedTuple):
    comparator: str
    left_operand: str
    right_operand: str


class Instruction(NamedTuple):
    destination: str
    opcode: str
    operand: int
    condition: Condition


def parse_instruction(s: str) -> Instruction:
    # b inc 5 if a > 1
    destination, opcode, operand, if_, cond_left, comparator, cond_right = s.split()
    assert if_ == 'if'
    condition = Condition(comparator=comparator, left_operand=cond_left, right_operand=cond_right)
    return Instruction(
        destination=destination,
        opcode=opcode,
        operand=int(operand),
        condition=condition
    )


def read_instructions(filename: str) -> List[Instruction]:
    with open(filename) as f:
        return [
            parse_instruction(line.strip())
            for line in f
        ]


def test_condition(mem: Mem, condition: Condition) -> bool:
    try:
        left = int(condition.left_operand)
    except ValueError:
        left = mem.get(condition.left_operand, 0)
    try:
        right = int(condition.right_operand)
    except ValueError:
        left = mem.get(condition.right_operand, 0)
    if condition.comparator == '<':
        return left < right
    elif condition.comparator == '>':
        return left > right
    elif condition.comparator == '<=':
        return left <= right
    elif condition.comparator == '>=':
        return left >= right
    elif condition.comparator == '==':
        return left == right
    elif condition.comparator == '!=':
        return left != right
    else:
        assert False, condition.comparator


def run_instructions(instructions: List[Instruction]) -> Tuple[int, Optional[int]]:
    mem: Mem = {}
    largest_ever = None
    for instruction in instructions:
        if not test_condition(mem, instruction.condition):
            continue
        value = mem.get(instruction.destination, 0)
        if instruction.opcode == 'inc':
            value += instruction.operand
        elif instruction.opcode == 'dec':
            value -= instruction.operand
        else:
            assert False, instruction.opcode
        if largest_ever is None or value > largest_ever:
            largest_ever = value
        mem[instruction.destination] = value
    return max(mem.values()), largest_ever


def main() -> None:
    example = read_instructions('example')
    input = read_instructions('input')

    assert run_instructions(example) == (1, 10)
    assert run_instructions(input) == (5966, 6347)


if __name__ == '__main__':
    main()
