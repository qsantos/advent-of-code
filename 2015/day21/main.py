from typing import Iterator, NamedTuple, Tuple


class Fighter(NamedTuple):
    hit_points: int
    damage: int
    armor: int


class Item(NamedTuple):
    cost: int
    damage: int
    armor: int


weapons = (
    Item(8, 4, 0),  # Dagger
    Item(10, 5, 0),  # Shortsword
    Item(25, 6, 0),  # Warhammer
    Item(40, 7, 0),  # Longsword
    Item(74, 8, 0),  # Greataxe
)
armors = (
    Item(0, 0, 0),  # no armor
    Item(13, 0, 1),  # Leather
    Item(31, 0, 2),  # Chainmail
    Item(53, 0, 3),  # Splintmail
    Item(75, 0, 4),  # Bandedmail
    Item(102, 0, 5),  # Platemail
)
rings = (
    Item(25, 1, 0),  # Damage +1
    Item(50, 2, 0),  # Damage +2
    Item(100, 3, 0),  # Damage +3
    Item(20, 0, 1),  # Defense +1
    Item(40, 0, 2),  # Defense +2
    Item(80, 0, 3),  # Defense +3
)


def player_wins(player: Fighter, boss: Fighter) -> bool:
    player_rounds_to_win = (boss.hit_points - 1) // max(1, player.damage - boss.armor) + 1
    boss_rounds_to_win = (player.hit_points - 1) // max(1, boss.damage - player.armor) + 1
    return player_rounds_to_win <= boss_rounds_to_win


def iter_equipements() -> Iterator[Tuple[int, int, int]]:
    cost = 0
    damage = 0
    armor = 0
    for weapon in weapons:
        cost += weapon.cost
        damage += weapon.damage
        armor += weapon.armor
        for armor_item in armors:
            cost += armor_item.cost
            damage += armor_item.damage
            armor += armor_item.armor
            yield cost, damage, armor
            for ring1 in rings:
                cost += ring1.cost
                damage += ring1.damage
                armor += ring1.armor
                yield cost, damage, armor
                for ring2 in rings:
                    if ring2 > ring1:
                        cost += ring2.cost
                        damage += ring2.damage
                        armor += ring2.armor
                        yield cost, damage, armor
                        cost -= ring2.cost
                        damage -= ring2.damage
                        armor -= ring2.armor
                cost -= ring1.cost
                damage -= ring1.damage
                armor -= ring1.armor
            cost -= armor_item.cost
            damage -= armor_item.damage
            armor -= armor_item.armor
        cost -= weapon.cost
        damage -= weapon.damage
        armor -= weapon.armor
    assert cost == 0
    assert damage == 0
    assert armor == 0


def min_cost_to_win(player_hit_points: int, boss: Fighter) -> int:
    return min(
        cost
        for cost, damage, armor in iter_equipements()
        if player_wins(Fighter(player_hit_points, damage, armor), boss)
    )


def max_cost_to_lose(player_hit_points: int, boss: Fighter) -> int:
    return max(
        cost
        for cost, damage, armor in iter_equipements()
        if not player_wins(Fighter(player_hit_points, damage, armor), boss)
    )


assert player_wins(Fighter(8, 5, 5), Fighter(12, 7, 2))

assert min_cost_to_win(100, Fighter(109, 8, 2)) == 111
assert max_cost_to_lose(100, Fighter(109, 8, 2)) == 188
