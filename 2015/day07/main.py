from typing import Dict

command_of_wire = {}
with open('input') as f:
    for line in f:
        command, wire = line.strip().split(' -> ')
        command_of_wire[wire] = command


value_of_wire: Dict[str, int] = {}


def evaluate(wire: str) -> int:
    try:
        return int(wire)
    except ValueError:
        pass
    if wire in value_of_wire:
        return value_of_wire[wire]
    command = command_of_wire[wire]
    parts = command.split()
    if len(parts) == 1:
        imm, = parts
        value = evaluate(imm)
    elif len(parts) == 2:
        gate, imm = parts
        assert gate == 'NOT'
        value = 65535 & ~evaluate(imm)
    elif len(parts) == 3:
        imm1, gate, imm2 = parts
        a = evaluate(imm1)
        b = evaluate(imm2)
        if gate == 'AND':
            value = a & b
        elif gate == 'OR':
            value = a | b
        elif gate == 'LSHIFT':
            value = a << b
        elif gate == 'RSHIFT':
            value = a >> b
        else:
            assert False, gate
    else:
        assert False, command
    value_of_wire[wire] = value
    return value


assert evaluate('a') == 46065

value_of_wire = {'b': 46065}
assert evaluate('a') == 14134
