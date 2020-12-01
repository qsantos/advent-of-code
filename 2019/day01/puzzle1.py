from typing import Iterable


def fuel_of_module(mass: int) -> int:
    return mass // 3 - 2


def total_fuel(masses: Iterable[int]) -> int:
    return sum(fuel_of_module(mass) for mass in masses)


def read_masses() -> Iterable[int]:
    with open('input') as f:
        for line in f:
            yield int(line.strip())


def main() -> None:
    print(total_fuel(read_masses()))


if __name__ == '__main__':
    main()
