#!/usr/bin/env python3
import re
from typing import Iterator, Tuple
from urllib.request import urlopen


def load_stats() -> bytes:
    with urlopen('https://adventofcode.com/2019/stats') as f:
        return f.read()  # type: ignore


def iter_day_stats(data: bytes) -> Iterator[Tuple[int, int, int]]:
    pattern = (
        rb'<a href="/\d+/day/(\d+)">\s*\d+  '
        rb'<span class="stats-both">\s*(\d+)</span>\s*'
        rb'<span class="stats-firstonly">\s*(\d+)</span>'
    )
    for match in re.findall(pattern, data):
        day, both_puzzle, one_puzzle = (int(x) for x in match)
        yield day, both_puzzle, one_puzzle


def main() -> None:
    prev_both_puzzle = None
    print('Day  Both puzzle  One puzzle  Total  Rel. puzzle 1/2  Rel. day before')
    for day, both_puzzle, one_puzzle in sorted(iter_day_stats(load_stats())):
        total = both_puzzle + one_puzzle
        if prev_both_puzzle is None:
            prev_both_puzzle = both_puzzle
        print((
            f'{day:2d}'
            f'         {both_puzzle:5d}'
            f'      {one_puzzle:5d}'
            f'   {total:5d}'
            f'             {both_puzzle / total * 100:2.0f} %'
            f'            {both_puzzle / prev_both_puzzle * 100:3.0f} %'
        ))
        prev_both_puzzle = both_puzzle


if __name__ == '__main__':
    main()
