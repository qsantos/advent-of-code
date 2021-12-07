from typing import List


def read_positions(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(x) for x in f.read().strip().split(',')]


def fuel_cost_linear(positions: List[int], target: int) -> int:
    return sum(abs(position - target) for position in positions)


def fuel_cost_quadratic(positions: List[int], target: int) -> int:
    total = 0
    for position in positions:
        d = abs(position - target)
        total += d * (d + 1) // 2
    return total


def min_fuel_cost_linear(positions: List[int]) -> int:
    positions = sorted(positions)
    target = positions[len(positions) // 2]
    return fuel_cost_linear(positions, target)


def min_fuel_cost_quadratic(positions: List[int]) -> int:
    return min(
        fuel_cost_quadratic(positions, target)
        for target in range(min(positions), max(positions) + 1)
    )


example = read_positions('example')
input = read_positions('input')

assert fuel_cost_linear(example, 1) == 41
assert fuel_cost_linear(example, 2) == 37
assert fuel_cost_linear(example, 3) == 39
assert fuel_cost_linear(example, 10) == 71

assert min_fuel_cost_linear(example) == 37
assert min_fuel_cost_linear(input) == 336701

assert fuel_cost_quadratic(example, 2) == 206
assert fuel_cost_quadratic(example, 5) == 168

assert min_fuel_cost_quadratic(example) == 168
assert min_fuel_cost_quadratic(input) == 95167302
