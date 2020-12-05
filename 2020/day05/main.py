from typing import Iterator, Tuple


def decode_boarding_pass(boarding_pass: str) -> Tuple[int, int]:
    row = int(boarding_pass[:7].replace('F', '0').replace('B', '1'), 2)
    col = int(boarding_pass[7:].replace('L', '0').replace('R', '1'), 2)
    return row, col


def iterate_seat_ids() -> Iterator[int]:
    with open('input') as f:
        for line in f:
            row, col = decode_boarding_pass(line)
            yield row * 8 + col

def puzzle1() -> None:
    print(max(iterate_seat_ids()))


def puzzle2() -> None:
    seat_ids = set(iterate_seat_ids())
    for missing_seat_id in set(range(2**10)) - seat_ids:
        if {missing_seat_id - 1, missing_seat_id + 1} <= seat_ids:
            print(missing_seat_id)


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
