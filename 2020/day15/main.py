from typing import List


def nth_number(numbers: List[int], last_turn: int) -> int:
    numbers = list(numbers)
    last = numbers.pop()
    turn = len(numbers)
    last_seen = {
        number: turn
        for turn, number in enumerate(numbers)
    }
    while turn < last_turn - 1:
        if last in last_seen:
            number = turn - last_seen[last]
        else:
            number = 0
        last_seen[last] = turn
        last = number
        turn += 1
    return last


def main() -> None:
    with open('input') as f:
        numbers = [int(x) for x in next(f).strip().split(',')]

    print(nth_number(numbers, 2020))  # puzzle 1
    print(nth_number(numbers, 30000000))  # puzzle 2


if __name__ == '__main__':
    main()
