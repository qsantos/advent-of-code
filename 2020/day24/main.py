from typing import Dict, List, Set, Tuple

Coord = Tuple[int, int]


def read_instructions(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def coord_of_instruction(instruction: str) -> Coord:
    offset = 0
    q = 0
    r = 0
    while offset < len(instruction):
        c = instruction[offset]
        offset += 1
        if c == 'e':
            q += 1
        elif c == 'w':
            q -= 1
        elif c == 'n':
            c = instruction[offset]
            offset += 1
            r -= 1
            if c == 'e':
                q += 1
            elif c == 'w':
                pass
            else:
                assert False
        elif c == 's':
            c = instruction[offset]
            offset += 1
            r += 1
            if c == 'e':
                pass
            elif c == 'w':
                q -= 1
            else:
                assert False
        else:
            assert False
    return q, r


def initial_state(instructions: List[str]) -> Set[Coord]:
    flipped: Set[Coord] = set()
    for instruction in instructions:
        coord = coord_of_instruction(instruction)
        if coord in flipped:
            flipped.remove(coord)
        else:
            flipped.add(coord)
    return flipped


def next_step(state: Set[Coord]) -> Set[Coord]:
    counts: Dict[Coord, int] = {}
    for q, r in state:
        for neighbor in [
            (q, r - 1),
            (q + 1, r - 1),
            (q - 1, r),
            (q + 1, r),
            (q - 1, r + 1),
            (q, r + 1)
        ]:
            try:
                counts[neighbor] += 1
            except KeyError:
                counts[neighbor] = 1
    return {
        tile
        for tile in state | set(counts)
        if tile in state and counts.get(tile, 0) == 1 or counts.get(tile, 0) == 2
    }


def run_steps(state: Set[Coord], n_steps: int) -> Set[Coord]:
    for _ in range(n_steps):
        state = next_step(state)
    return state


example = read_instructions('example')
input = read_instructions('input')

example_state = initial_state(example)
input_state = initial_state(input)

assert len(example_state) == 10
assert len(input_state) == 450

assert len(run_steps(example_state, 100)) == 2208
assert len(run_steps(input_state, 100)) == 4059
