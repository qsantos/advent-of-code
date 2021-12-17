from itertools import combinations
from typing import List, Set, Tuple

Things = List[List[int]]
State = Tuple[int, Tuple[Tuple[int, ...], ...]]
Front = List[State]


def read_initial_state(filename: str) -> Things:
    floors = []
    microchips = {}
    generators = {}
    with open(filename) as f:
        for floor, line in enumerate(f):
            contents = line.strip()[line.index(' contains ') + len(' contains '):-len('.')]
            if contents == 'nothing relevant':
                # no things
                things = []
            elif ', ' in contents:
                # three things or more
                things = contents.split(', ')
                assert things[-1].startswith('and ')
                things[-1] = things[-1][len('and '):]
            elif ' and ' in contents:
                # two things
                things = contents.split(' and ')
            else:
                # one thing
                things = [contents]
            for thing in things:
                if thing.endswith('-compatible microchip'):
                    kind = thing[len('a '):-len('-compatible microchip')]
                    microchips[kind] = floor
                elif thing.endswith(' generator'):
                    kind = thing[len('a '):-len(' generator')]
                    generators[kind] = floor
                else:
                    assert False, thing
            floors.append(tuple(things))
    kinds = set(generators)
    assert set(microchips) == kinds
    ret = []
    for i, kind in enumerate(kinds):
        ret.append([microchips[kind], generators[kind]])
    return ret


def all_on_floor(things: Things, floor: int) -> bool:
    return all(
        microchip == floor and generator == floor
        for microchip, generator in things
    )


def is_valid(things: Things) -> bool:
    irradiated_floors = set()
    for microchip, generator in things:
        irradiated_floors.add(generator)
    for microchip, generator in things:
        if microchip != generator and microchip in irradiated_floors:
            return False
    return True


def freeze(lift: int, things: Things) -> State:
    return (lift, tuple(sorted(tuple(thing) for thing in things)))


def expand_front(n_floors: int, q: Front, seen: Set[State]) -> Front:
    next_q = []
    for lift, things in q:
        candidates = []
        for i, (microchip, generator) in enumerate(things):
            if microchip == lift:
                candidates.append((i, 0))
            if generator == lift:
                candidates.append((i, 1))
        for dh in (-1, 1):
            if not 0 <= lift + dh < n_floors:
                continue
            for n_things in (1, 2):
                for luggage in combinations(candidates, r=n_things):
                    new_lift = lift + dh
                    new_things = [list(thing) for thing in things]
                    for kind, is_generator in luggage:
                        new_things[kind][is_generator] += dh
                    if not is_valid(new_things):
                        continue
                    new_state = freeze(new_lift, new_things)
                    if new_state in seen:
                        continue
                    seen.add(new_state)
                    next_q.append(new_state)
    return next_q


def solve(things: Things) -> int:
    n_floors = 4
    forward_start = freeze(0, things)
    backward_start = freeze(3, [[3, 3] for _ in things])
    forward_front = [forward_start]
    backward_front = [backward_start]
    forward_seen: Set[State] = {forward_start}
    backward_seen: Set[State] = {backward_start}
    forward_steps = 0
    backward_steps = 0
    while forward_front or backward_front:
        forward_front = expand_front(n_floors, forward_front, forward_seen)
        forward_steps += 1
        if forward_seen & backward_seen:
            return forward_steps + backward_steps
        backward_front = expand_front(n_floors, backward_front, backward_seen)
        backward_steps += 1
        if forward_seen & backward_seen:
            return forward_steps + backward_steps
    return -1


def main() -> None:
    example = read_initial_state('example')
    input = read_initial_state('input')

    assert solve(example) == 11
    assert solve(input) == 33

    # puzzle 2
    input.append([0, 0])
    input.append([0, 0])

    assert solve(input) == 57


if __name__ == '__main__':
    main()
