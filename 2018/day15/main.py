from copy import deepcopy
from typing import Dict, Iterator, List, Optional, Set, Tuple

Coord = Tuple[int, int]
Units = Dict[Coord, int]
State = Tuple[Set[Coord], Units, Units]


def read_map(filename: str) -> State:
    walls = set()
    elves = {}
    goblins = {}
    with open(filename) as f:
        for i, line in enumerate(f):
            for j, c in enumerate(line.strip()):
                if c == '#':
                    walls.add((i, j))
                elif c == '.':
                    pass
                elif c == 'E':
                    elves[i, j] = 200
                elif c == 'G':
                    goblins[i, j] = 200
                else:
                    assert False, c
    return walls, elves, goblins


def print_state(state: State) -> None:
    walls, elves, goblins = state
    max_i = max(i for i, j in walls)
    max_j = max(j for i, j in walls)
    for i in range(max_i + 1):
        row = []
        healths = []
        for j in range(max_j + 1):
            if (i, j) in walls:
                row.append('#')
            elif (i, j) in elves:
                row.append('E')
                healths.append(f'E({elves[i, j]})')
            elif (i, j) in goblins:
                row.append('G')
                healths.append(f'G({goblins[i, j]})')
            else:
                row.append('.')
        print(''.join(row) + '   ' + ', '.join(healths))


def neighbors(coord: Coord) -> Iterator[Coord]:
    i, j = coord
    for ni, nj in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
        yield ni, nj


def closest_target(blocking: Set[Coord], subject: Coord, targets: Set[Coord]) -> Optional[Coord]:
    q = [subject]
    seen = set()
    closest_targets: Set[Coord] = set()
    while q and not closest_targets:
        next_q: List[Coord] = []
        for coord in q:
            if coord in seen:
                continue
            seen.add(coord)
            if coord in targets:
                closest_targets.add(coord)
                continue
            for neighbor in neighbors(coord):
                if neighbor not in blocking:
                    next_q.append(neighbor)
        q = next_q
    if closest_targets:
        return min(closest_targets)
    else:
        return None


def step_towards(blocking: Set[Coord], subject: Coord, targets: Set[Coord]) -> Coord:
    target = closest_target(blocking, subject, targets)
    if target is None:
        return subject

    q = [target]
    seen = set()
    candidates: Set[Coord] = set()
    while q and not candidates:
        next_q: List[Coord] = []
        for coord in q:
            if coord in seen:
                continue
            seen.add(coord)
            for neighbor in neighbors(coord):
                if neighbor == subject:
                    candidates.add(coord)
                elif neighbor not in blocking:
                    next_q.append(neighbor)
        q = next_q
    return min(candidates)


def do_round(state: State, elven_attack_power: int) -> bool:
    walls, elves, goblins = state
    units = list(elves) + list(goblins)
    units.sort()
    unit_index = 0
    while unit_index < len(units):
        unit = units[unit_index]

        if unit in elves:
            us, them = elves, goblins
            attack_power = elven_attack_power
        else:
            us, them = goblins, elves
            attack_power = 3

        if not them:
            return False

        health = us.pop(unit)

        in_range = {
            neighbor
            for neighbor in neighbors(unit)
            if neighbor in them
        }

        # move
        if not in_range:
            blocking = walls | set(elves) | set(goblins)
            target_neighbor_open_spaces = {
                neighbor
                for target in them
                for neighbor in neighbors(target)
                if neighbor not in blocking
            }
            if target_neighbor_open_spaces:
                unit = step_towards(blocking, unit, target_neighbor_open_spaces)

            in_range = {
                neighbor
                for neighbor in neighbors(unit)
                if neighbor in them
            }

        # attack
        if in_range:
            min_health = min(them[target] for target in in_range)
            min_health_targets = {target for target in in_range if them[target] == min_health}
            target = min(min_health_targets)
            them[target] -= attack_power
            if them[target] <= 0:
                del them[target]
                died_index = units.index(target)
                if died_index > unit_index:
                    units.pop(died_index)

        us[unit] = health
        unit_index += 1
    return True


def battle_outcome(state: State, attack_power: int) -> int:
    state = deepcopy(state)
    rounds = 0
    # print('Initially')
    # print_state(state)
    while do_round(state, attack_power):
        rounds += 1
        # print()
        # print(f'After {rounds} rounds')
        # print_state(state)
    walls, elves, goblins = state
    total_health = sum(elves.values()) + sum(goblins.values())
    return rounds * total_health


def win_without_a_single_death(state: State, attack_power: int) -> bool:
    state = deepcopy(state)
    walls, elves, goblins = state
    n_elves = len(elves)
    while do_round(state, attack_power):
        if len(elves) != n_elves:
            return False
    return len(elves) == n_elves


def lowest_integer_attack_power(state: State) -> int:
    # attack_power = 4
    # while not win_without_a_single_death(state, attack_power):
    #     attack_power *= 2
    # upper = attack_power
    # lower = upper // 2
    # while lower < upper - 1:
    #     middle = (upper + lower) // 2
    #     if win_without_a_single_death(state, middle):
    #         upper = middle
    #     else:
    #         lower = middle
    # assert not win_without_a_single_death(state, upper - 1)
    # assert win_without_a_single_death(state, upper)
    # return upper
    attack_power = 4
    while not win_without_a_single_death(state, attack_power):
        attack_power += 1
    return attack_power


def near_death_outcome(state: State) -> int:
    return battle_outcome(state, lowest_integer_attack_power(state))


def main() -> None:
    example1 = read_map('example1')
    example2 = read_map('example2')
    example3 = read_map('example3')
    example4 = read_map('example4')
    example5 = read_map('example5')
    example6 = read_map('example6')
    input = read_map('input')

    assert battle_outcome(example1, 3) == 27730
    assert battle_outcome(example2, 3) == 36334
    assert battle_outcome(example3, 3) == 39514
    assert battle_outcome(example4, 3) == 27755
    assert battle_outcome(example5, 3) == 28944
    assert battle_outcome(example6, 3) == 18740
    assert battle_outcome(input, 3) == 269430

    assert lowest_integer_attack_power(example1) == 15
    assert lowest_integer_attack_power(example2) == 4
    assert lowest_integer_attack_power(example3) == 4
    assert lowest_integer_attack_power(example4) == 15
    assert lowest_integer_attack_power(example5) == 12
    assert lowest_integer_attack_power(example6) == 34
    assert lowest_integer_attack_power(input) == 19

    assert near_death_outcome(example1) == 4988
    assert near_death_outcome(example3) == 31284
    assert near_death_outcome(example4) == 3478
    assert near_death_outcome(example5) == 6474
    assert near_death_outcome(example6) == 1140
    assert near_death_outcome(input) == 55160


# between 54680 and 55160
if __name__ == '__main__':
    main()
