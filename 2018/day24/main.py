import re
from copy import deepcopy
from dataclasses import dataclass
from typing import Dict, List, TextIO, Tuple

log = False


@dataclass
class Group:
    army_name: str
    number: int
    units: int
    hit_points: int
    weaknesses: List[str]
    immunities: List[str]
    attack_damage: int
    attack_type: str
    initiative: int

    def effective_power(self, boost: int) -> int:
        return self.units * (self.attack_damage + boost)

    def damage_to(self, other: 'Group', boost: int) -> int:
        if self.attack_type in other.immunities:
            return 0
        elif self.attack_type in other.weaknesses:
            return 2 * self.effective_power(boost)
        else:
            return self.effective_power(boost)

    def attack(self, other: 'Group', boost: int) -> None:
        damage = self.damage_to(other, boost)
        killed = min(other.units, damage // other.hit_points)
        if log:
            print(
                f'{self.army_name} group {self.number} attacks defending group '
                f'{other.number}, killing {killed} units')
        other.units -= killed


GroupId = Tuple[str, int]
Army = Dict[GroupId, Group]


group_pattern = re.compile(
    r'^(\d+) units each with (\d+) hit points(?: \(([^)]*)\))? '
    r'with an attack that does (\d+) (\S+) damage at initiative (\d+)$'
)


def parse_weaknesses_and_immunities(s: str) -> Tuple[List[str], List[str]]:
    weaknesses = []
    immunities = []
    if s:
        for s in s.split('; '):
            if s.startswith('weak to '):
                for s in s[len('weak to '):].split(', '):
                    weaknesses.append(s)
            elif s.startswith('immune to '):
                for s in s[len('immune to '):].split(', '):
                    immunities.append(s)
            else:
                assert False, s
    return weaknesses, immunities


def parse_group(army_name: str, number: int, s: str) -> Group:
    m = group_pattern.match(s)
    assert m is not None, s
    units, hp, weaknesses_and_immunities, attack_damage, attack_type, initiative = m.groups()
    weaknesses, immunities = parse_weaknesses_and_immunities(weaknesses_and_immunities)
    return Group(
        army_name=army_name,
        number=number,
        units=int(units),
        hit_points=int(hp),
        weaknesses=weaknesses,
        immunities=immunities,
        attack_damage=int(attack_damage),
        attack_type=attack_type,
        initiative=int(initiative),
    )


def read_army(army_name: str, f: TextIO) -> Army:
    assert next(f).strip() == army_name + ':'
    army = {}
    number = 1
    for line in f:
        if not line.strip():
            break
        army[army_name, number] = parse_group(army_name, number, line.strip())
        number += 1
    return army


def read_armies(filename: str) -> Tuple[Army, Army]:
    with open(filename) as f:
        army1 = read_army('Immune System', f)
        army2 = read_army('Infection', f)
    return army1, army2


def select_targets(
    army1: Army,
    army2: Army,
    army1_boost: int,
    army2_boost: int,
) -> Dict[GroupId, GroupId]:
    def order_key(group_id: GroupId) -> Tuple[int, int]:
        group = army1[group_id]
        return -group.effective_power(army1_boost), -group.initiative

    targeted = set()
    targets = {}
    for group_id in sorted(army1, key=order_key):
        group = army1[group_id]

        def preference_key(other_id: GroupId) -> Tuple[int, int, int]:
            other = army2[other_id]
            damage = group.damage_to(other, army1_boost)
            effective_power = other.effective_power(army2_boost)
            if log:
                print(
                    f'{group.army_name} group {group.number} would deal defending group '
                    f'{other.number} {damage} damage'
                )
            return damage, effective_power, other.initiative

        candidates = {
            other_id
            for other_id in army2
            if other_id not in targeted
        }
        if not candidates:
            break

        target_id = max(candidates, key=preference_key)
        target = army2[target_id]
        if group.damage_to(target, army1_boost) != 0:
            targets[group_id] = target_id
            targeted.add(target_id)
    return targets


def do_round(army1: Army, army2: Army, army1_boost: int) -> None:
    if log:
        print('Immune System:')
        for i, group in enumerate(army1.values()):
            print(f'Group {group.number} contains {group.units} units')
        print('Infection:')
        for i, group in enumerate(army2.values()):
            print(f'Group {group.number} contains {group.units} units')
        print()

    # targeting selection
    targets = {
        **select_targets(army2, army1, 0, army1_boost),
        **select_targets(army1, army2, army1_boost, 0)}
    if log:
        print()

    # attacking
    def order_key(group_id: GroupId) -> int:
        if group_id in army1:
            return -army1[group_id].initiative
        else:
            return -army2[group_id].initiative

    groups = {**army1, **army2}
    alive = set(army1.keys()) | set(army2.keys())
    for group_id in sorted(alive, key=order_key):
        if group_id not in alive:
            continue
        if group_id not in targets:
            continue
        target_id = targets[group_id]
        assert target_id in alive
        group = groups[group_id]
        target = groups[target_id]
        group.attack(target, army1_boost if group_id in army1 else 0)
        if target.units <= 0:
            alive.remove(target_id)
            if target_id in army1:
                del army1[target_id]
            else:
                del army2[target_id]

    if log:
        print()
        print()
        print()


def count_units(army: Army) -> int:
    return sum(group.units for group in army.values())


def units_after_battle(armies: Tuple[Army, Army], army1_boost: int = 0) -> Tuple[int, int]:
    army1, army2 = deepcopy(armies)
    units = (count_units(army1), count_units(army2))
    while army1 and army2:
        previous_units = units
        do_round(army1, army2, army1_boost)
        units = (count_units(army1), count_units(army2))
        if units == previous_units:
            return units
    return units


def win_battle(armies: Tuple[Army, Army], boost: int) -> bool:
    a, b = units_after_battle(armies, boost)
    return b == 0


def smallest_boost(armies: Tuple[Army, Army]) -> int:
    if win_battle(armies, 0):
        return 0
    upper = 1
    while not win_battle(armies, upper):
        upper *= 2
    lower = upper // 2
    while lower < upper - 1:
        middle = (lower + upper) // 2
        if win_battle(armies, middle):
            upper = middle
        else:
            lower = middle
    assert not win_battle(armies, upper - 1)
    assert win_battle(armies, upper)
    return units_after_battle(armies, upper)[0]


def main() -> None:
    example = read_armies('example')
    input = read_armies('input')

    assert units_after_battle(example) == (0, 5216)
    assert units_after_battle(input) == (0, 25088)

    assert units_after_battle(example, army1_boost=1570) == (51, 0)
    assert smallest_boost(input) == 2002


if __name__ == '__main__':
    main()
