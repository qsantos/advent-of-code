import re
from dataclasses import dataclass
from math import gcd
from typing import Iterator, List, Tuple


@dataclass
class Moon:
    pos: List
    vel: List


example1 = [
    Moon([-1, 0, 2], [0, 0, 0]),
    Moon([2, -10, -7], [0, 0, 0]),
    Moon([4, -8, 8], [0, 0, 0]),
    Moon([3, 5, -1], [0, 0, 0]),
]

example2 = [
    Moon([-8, -10, 0], [0, 0, 0]),
    Moon([5, 5, 10], [0, 0, 0]),
    Moon([2, -7, 3], [0, 0, 0]),
    Moon([9, -8, -3], [0, 0, 0]),
]


def update_velocity(moons: List[Moon], axis: int) -> None:
    for a in moons:
        for b in moons:
            if a == b:
                continue
            if a.pos[axis] < b.pos[axis]:
                a.vel[axis] += 1
            elif a.pos[axis] > b.pos[axis]:
                a.vel[axis] -= 1


def update_positions(moons: List[Moon], axis: int) -> None:
    for a in moons:
        a.pos[axis] += a.vel[axis]


def do_step(moons: List[Moon], axis: int) -> None:
    update_velocity(moons, axis)
    update_positions(moons, axis)


def simulate(moons: List[Moon], steps: int) -> None:
    for step in range(steps):
        for axis in range(3):
            do_step(moons, axis)


def potential_energy(moon: Moon) -> int:
    return sum(abs(v) for v in moon.pos)


def kinetic_energy(moon: Moon) -> int:
    return sum(abs(v) for v in moon.vel)


def total_energy(moon: Moon) -> int:
    return potential_energy(moon) * kinetic_energy(moon)


def system_energy(moons: List[Moon]) -> int:
    return sum(total_energy(moon) for moon in moons)


def read_system(filename: str) -> Iterator[Moon]:
    pattern = re.compile(r'<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>')
    with open(filename) as f:
        for line in f:
            m = pattern.match(line)
            assert m is not None
            yield Moon(pos=[int(x) for x in m.groups()], vel=[0, 0, 0])


def state_copy(moons: List[Moon]) -> List[Moon]:
    return [
        Moon(list(moon.pos), list(moon.vel))
        for moon in moons
    ]


def state_eq(a: List[Moon], b: List[Moon], axis: int) -> bool:
    for ma, mb in zip(a, b):
        if ma.pos[axis] != mb.pos[axis]:
            return False
        if ma.vel[axis] != mb.vel[axis]:
            return False
    return True


def floyd(x0: List[Moon], axis: int) -> Tuple[int, int]:
    tortoise = state_copy(x0)
    hare = state_copy(x0)
    do_step(tortoise, axis)
    do_step(hare, axis)
    do_step(hare, axis)

    while not state_eq(tortoise, hare, axis):
        do_step(tortoise, axis)
        do_step(hare, axis)
        do_step(hare, axis)

    mu = 0
    tortoise = state_copy(x0)
    while not state_eq(tortoise, hare, axis):
        do_step(tortoise, axis)
        do_step(hare, axis)
        mu += 1

    lam = 1
    hare = state_copy(tortoise)
    do_step(hare, axis)
    while not state_eq(tortoise, hare, axis):
        do_step(hare, axis)
        lam += 1

    return lam, mu


def brent(x0: List[Moon], axis: int) -> Tuple[int, int]:
    power = lam = 1
    tortoise = state_copy(x0)
    hare = state_copy(x0)
    do_step(hare, axis)

    while not state_eq(tortoise, hare, axis):
        if power == lam:
            tortoise = state_copy(hare)
            power *= 2
            lam = 0
        do_step(hare, axis)
        lam += 1

    tortoise = state_copy(x0)
    hare = state_copy(x0)

    for i in range(lam):
        do_step(hare, axis)

    mu = 0
    while not state_eq(tortoise, hare, axis):
        do_step(tortoise, axis)
        do_step(hare, axis)
        mu += 1

    return lam, mu


def lcm(a: int, b: int) -> int:
    return a * b // gcd(a, b)


def combine(cycles: List[Tuple[int, int]]) -> Tuple[int, int]:
    lam, mu = cycles[0]
    global_lam = lam
    for lam, mu in cycles[1:]:
        global_lam = lcm(global_lam, lam)

    global_mu = 0  # TODO

    return global_lam, global_mu


def check_example1() -> None:
    moons = state_copy(example1)
    simulate(moons, 10)
    assert system_energy(moons) == 179


def check_example2() -> None:
    moons = state_copy(example2)
    simulate(moons, 100)
    assert system_energy(moons) == 1940


def main() -> None:
    check_example1()
    check_example2()

    # puzzle 1
    moons = list(read_system('input'))
    simulate(moons, 1000)
    assert system_energy(moons) == 7202

    # puzzle 2
    print(combine(list(
        floyd(example1, axis)
        for axis in range(3)
    )))

    print(combine(list(
        floyd(example2, axis)
        for axis in range(3)
    )))

    print(combine(list(
        floyd(moons, axis)
        for axis in range(3)
    )))


if __name__ == '__main__':
    main()
