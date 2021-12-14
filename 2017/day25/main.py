from typing import Dict, NamedTuple, TextIO, Tuple


class Transition(NamedTuple):
    input_value: int
    output_value: int
    direction: str
    next_state: str


class TuringMachine(NamedTuple):
    initial_state: str
    steps_before_diagnostic: int
    transitions: Dict[str, Tuple[Transition, Transition]]


def read_transition(f: TextIO) -> Transition:
    # input value
    line = next(f).strip()
    assert line.startswith('If the current value is '), line
    assert line.endswith(':'), line
    input_value = int(line[len('If the current value is '):-len(':')])

    # output value
    line = next(f).strip()
    assert line.startswith('- Write the value '), line
    assert line.endswith('.'), line
    output_value = int(line[len('- Write the value '):-len('.')])

    # direction
    line = next(f).strip()
    if line == '- Move one slot to the left.':
        direction = 'left'
    elif line == '- Move one slot to the right.':
        direction = 'right'
    else:
        assert False, line

    # next state
    line = next(f).strip()
    assert line.startswith('- Continue with state '), line
    assert line.endswith('.'), line
    next_state = line[len('- Continue with state '):-len('.')]

    return Transition(
        input_value=input_value,
        output_value=output_value,
        direction=direction,
        next_state=next_state,
    )


def read_transitions(f: TextIO) -> Tuple[str, Tuple[Transition, Transition]]:
    line = next(f).strip()
    assert line.startswith('In state '), line
    assert line.endswith(':'), line
    from_state = line[len('In state '):-len(':')]

    part0 = read_transition(f)
    part1 = read_transition(f)
    assert part0[0] == 0
    assert part1[0] == 1
    return from_state, (part0, part1)


def read_tm(filename: str) -> TuringMachine:
    with open(filename) as f:
        # state
        line = next(f).strip()
        assert line.startswith('Begin in state '), line
        assert line.endswith('.'), line
        initial_state = line[len('Begin in state '):-len('.')]

        # steps
        line = next(f).strip()
        assert line.startswith('Perform a diagnostic checksum after '), line
        assert line.endswith(' steps.'), line
        steps = int(line[len('Perform a diagnostic checksum after '):-len(' steps.')])

        transitions = dict(
            read_transitions(f)
            for line in f
        )

    return TuringMachine(
        initial_state=initial_state,
        steps_before_diagnostic=steps,
        transitions=transitions,
    )


def diagnostic_checksum(tm: TuringMachine) -> int:
    state = tm.initial_state
    tape: Dict[int, int] = {}
    cursor = 0
    for _ in range(tm.steps_before_diagnostic):
        input_value = tape.get(cursor, 0)
        transition = tm.transitions[state][input_value]
        assert transition.input_value == input_value
        tape[cursor] = transition.output_value
        if transition.direction == 'left':
            cursor -= 1
        else:
            cursor += 1
        state = transition.next_state
    return sum(tape.values())


def main() -> None:
    example = read_tm('example')
    input = read_tm('input')

    assert diagnostic_checksum(example) == 3
    assert diagnostic_checksum(input) == 2725


if __name__ == '__main__':
    main()
