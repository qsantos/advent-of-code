from typing import Tuple


def do_move(src: Tuple[int, int], move: str) -> Tuple[int, int]:
    x, y = src
    if move == '<':
        x -= 1
    elif move == '>':
        x += 1
    elif move == '^':
        y -= 1
    elif move == 'v':
        y += 1
    else:
        assert False, move
    return x, y


def count_houses(moves: str) -> int:
    santa = 0, 0
    houses = {santa}
    for move in moves:
        santa = do_move(santa, move)
        houses.add(santa)
    return len(houses)


def count_houses_with_robo_santa(moves: str) -> int:
    santa = 0, 0
    robo_santa = 0, 0
    it = iter(moves)
    houses = {santa}
    for move in it:
        santa = do_move(santa, move)
        houses.add(santa)
        robo_santa = do_move(robo_santa, next(it))
        houses.add(robo_santa)
    return len(houses)


def main() -> None:
    with open('input') as f:
        input = f.read().strip()

    assert count_houses('>') == 2
    assert count_houses('^>v<') == 4
    assert count_houses('^v^v^v^v^v') == 2
    assert count_houses(input) == 2572

    assert count_houses_with_robo_santa('^v') == 3
    assert count_houses_with_robo_santa('^>v<') == 3
    assert count_houses_with_robo_santa('^v^v^v^v^v') == 11
    assert count_houses_with_robo_santa(input) == 2631


if __name__ == '__main__':
    main()
