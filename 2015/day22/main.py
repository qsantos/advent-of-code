# 1066 < answer < 1295
from heapq import heappop, heappush
from typing import Iterator, NamedTuple


class State(NamedTuple):
    spent_mana: int
    current_mana: int
    boss_hp: int
    player_hp: int
    shield_countdown: int
    poison_countdown: int
    recharge_countdown: int


def apply_effects(state: State) -> State:
    # shield
    if state.shield_countdown > 0:
        state = state._replace(
            shield_countdown=state.shield_countdown - 1,
        )
    # poison
    if state.poison_countdown > 0:
        state = state._replace(
            boss_hp=state.boss_hp - 3,
            poison_countdown=state.poison_countdown - 1,
        )
    # recharge
    if state.recharge_countdown > 0:
        state = state._replace(
            current_mana=state.current_mana + 101,
            recharge_countdown=state.recharge_countdown - 1,
        )
    return state


def player_actions(state: State) -> Iterator[State]:
    state = apply_effects(state)
    # magic missile
    if state.current_mana >= 53:
        yield state._replace(
            spent_mana=state.spent_mana + 53,
            current_mana=state.current_mana - 53,
            boss_hp=state.boss_hp - 4,
        )
    # drain
    if state.current_mana >= 73:
        yield state._replace(
            spent_mana=state.spent_mana + 73,
            current_mana=state.current_mana - 73,
            boss_hp=state.boss_hp - 2,
            player_hp=state.player_hp + 2,
        )
    # shield
    if state.shield_countdown == 0 and state.current_mana >= 113:
        yield state._replace(
            spent_mana=state.spent_mana + 113,
            current_mana=state.current_mana - 113,
            shield_countdown=6,
        )
    # poison
    if state.poison_countdown == 0 and state.current_mana >= 173:
        yield state._replace(
            spent_mana=state.spent_mana + 173,
            current_mana=state.current_mana - 173,
            poison_countdown=6,
        )
    # recharge
    if state.recharge_countdown == 0 and state.current_mana >= 229:
        yield state._replace(
            spent_mana=state.spent_mana + 229,
            current_mana=state.current_mana - 229,
            recharge_countdown=5,
        )


def boss_action(state: State, boss_damage: int) -> State:
    state = apply_effects(state)
    if state.shield_countdown > 0:
        player_armor = 7
    else:
        player_armor = 0
    return state._replace(
        player_hp=state.player_hp - max(1, boss_damage - player_armor),
    )


def least_mana_to_win(initial_boss_hp: int, boss_damage: int, *, hard: bool) -> int:
    q = [State(
        spent_mana=0,
        current_mana=500,
        boss_hp=initial_boss_hp,
        player_hp=50,
        shield_countdown=0,
        poison_countdown=0,
        recharge_countdown=0,
    )]
    seen = set()
    while q:
        state = heappop(q)
        if state in seen:
            continue
        seen.add(state)
        if hard:
            state = state._replace(player_hp=state.player_hp - 1)
            if state.player_hp <= 0:
                continue
        for state in player_actions(state):
            if state.boss_hp <= 0:
                return state.spent_mana
            state = boss_action(state, boss_damage)
            if state.boss_hp <= 0:
                return state.spent_mana
            if state.player_hp <= 0:
                continue
            heappush(q, state)
    return -1


assert least_mana_to_win(55, 8, hard=False) == 953
assert least_mana_to_win(55, 8, hard=True) == 1289
