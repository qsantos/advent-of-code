from typing import Dict, List, Set, Tuple


Instruction = Tuple[int, int, int, int]
State = List[int]
Transition = Tuple[State, Instruction, State]
Problem = Tuple[List[Transition], List[Instruction]]


def read_before_after(s: str) -> State:
    return [int(v) for v in s[len('Before: ['):-len(']')].split(',')]


def read_instruction(s: str) -> Instruction:
    a, b, c, d = [int(x) for x in s.split()]
    return a, b, c, d


def read_transitions_and_program(filename: str) -> Problem:
    transitions = []
    with open(filename) as f:
        while True:
            before = next(f).strip()
            if not before:
                break
            instruction = next(f).strip()
            after = next(f).strip()
            assert not next(f).strip()  # skip empty line
            transitions.append((
                read_before_after(before),
                read_instruction(instruction),
                read_before_after(after),
            ))
        assert not next(f).strip()  # skip empty line
        program = [
            read_instruction(line.strip())
            for line in f
        ]

    return transitions, program


opcode_names = [
    'addr',
    'addri',
    'mulr',
    'muli',
    'banr',
    'bani',
    'borr',
    'bori',
    'setr',
    'seti',
    'gtir',
    'gtri',
    'gtrr',
    'eqir',
    'eqri',
    'eqrr',
]


def run_opcode_by_name(mem: List[int], name: str, a: int, b: int, c: int) -> None:
    if name == 'addr':
        mem[c] = mem[a] + mem[b]
    elif name == 'addri':
        mem[c] = mem[a] + b
    elif name == 'mulr':
        mem[c] = mem[a] * mem[b]
    elif name == 'muli':
        mem[c] = mem[a] * b
    elif name == 'banr':
        mem[c] = mem[a] & mem[b]
    elif name == 'bani':
        mem[c] = mem[a] & b
    elif name == 'borr':
        mem[c] = mem[a] | mem[b]
    elif name == 'bori':
        mem[c] = mem[a] | b
    elif name == 'setr':
        mem[c] = mem[a]
    elif name == 'seti':
        mem[c] = a
    elif name == 'gtir':
        mem[c] = 1 if a > mem[b] else 0
    elif name == 'gtri':
        mem[c] = 1 if mem[a] > b else 0
    elif name == 'gtrr':
        mem[c] = 1 if mem[a] > mem[b] else 0
    elif name == 'eqir':
        mem[c] = 1 if a == mem[b] else 0
    elif name == 'eqri':
        mem[c] = 1 if mem[a] == b else 0
    elif name == 'eqrr':
        mem[c] = 1 if mem[a] == mem[b] else 0
    else:
        assert False


def name_candidates_of_transition(transition: Transition) -> Set[str]:
    before, instruction, after = transition
    opcode, a, b, c = instruction
    names: Set[str] = set()
    for name in opcode_names:
        mem = list(before)
        run_opcode_by_name(mem, name, a, b, c)
        if mem == after:
            names.add(name)
    return names


def count_potential_chameleons(problem: Problem) -> int:
    transitions, _ = problem
    return sum(
        1
        for transition in transitions
        if len(name_candidates_of_transition(transition)) >= 3
    )


def identify_opcodes(transitions: List[Transition]) -> Dict[int, str]:
    name_candidates_of_opcode = {
        opcode: set(opcode_names)
        for opcode in range(16)
    }
    for transition in transitions:
        _, (opcode, _, _, _), _ = transition
        name_candidates_of_opcode[opcode] &= name_candidates_of_transition(transition)
    ret = {}
    while name_candidates_of_opcode:
        for opcode, names in name_candidates_of_opcode.items():
            if len(names) != 1:
                continue
            name, = names
            del name_candidates_of_opcode[opcode]
            ret[opcode] = name
            for other_opcode, other_names in name_candidates_of_opcode.items():
                if other_opcode != opcode and name in other_names:
                    other_names.remove(name)
            break
    return ret


def run_program(name_of_opcode: Dict[int, str], program: List[Instruction]) -> int:
    mem = [0] * 4
    for opcode, a, b, c in program:
        name = name_of_opcode[opcode]
        run_opcode_by_name(mem, name, a, b, c)
    return mem[0]


def run_problem(problem: Problem) -> int:
    transitions, program = problem
    name_of_opcode = identify_opcodes(transitions)
    return run_program(name_of_opcode, program)


def main() -> None:
    example = read_transitions_and_program('example')
    input = read_transitions_and_program('input')

    assert count_potential_chameleons(example) == 1
    assert count_potential_chameleons(input) == 544

    assert run_problem(input) == 600


if __name__ == '__main__':
    main()
