from math import sqrt
from typing import Dict, Iterator, List, Tuple

Tile = List[str]
Positioned = Dict[Tuple[int, int], Tuple[int, Tile]]


def isqrt(x: int) -> int:
    r = int(round(sqrt(x)))
    assert r * r == x
    return r


def transforms(tile: Tile) -> Iterator[Tile]:
    for t in (tile, tile[::-1]):
        yield t
        t = [''.join(t[len(t) - 1 - j][i] for j in range(len(t))) for i in range(len(t[0]))]
        yield t
        t = [''.join(t[len(t) - 1 - j][i] for j in range(len(t))) for i in range(len(t[0]))]
        yield t
        t = [''.join(t[len(t) - 1 - j][i] for j in range(len(t))) for i in range(len(t[0]))]
        yield t


def read_image() -> Positioned:
    tiles: Dict[int, Tile] = {}
    with open('input') as f:
        for part in f.read().strip().split('\n\n'):
            name_str, *tile = part.split('\n')
            name = int(name_str[len('Tile '):-len(':')])
            tiles[name] = tile

    size = isqrt(len(tiles))
    positioned: Positioned = {}

    def aux(row: int, col: int) -> bool:
        if row == size:
            return True

        for name in set(tiles):
            for tile in transforms(tiles[name]):
                for (r, c), (n, t) in positioned.items():
                    if row == r and col == c - 1:
                        assert False
                    if row == r and col == c + 1:
                        if any(tile[i][0] != t[i][-1] for i in range(len(tile))):
                            break
                    if col == c and row == r - 1:
                        assert False
                    if col == c and row == r + 1:
                        if any(tile[0][j] != t[-1][j] for j in range(len(tile[0]))):
                            break
                else:
                    positioned[row, col] = name, tile
                    del tiles[name]
                    if col == size - 1:
                        ret = aux(row + 1, 0)
                    else:
                        ret = aux(row, col + 1)
                    if ret:
                        return True
                    del positioned[row, col]
                    tiles[name] = tile

        return False

    ret = aux(0, 0)
    assert ret
    return positioned


def puzzle1(positioned: Positioned) -> None:
    r = 1
    size = isqrt(len(positioned))
    for pos in [(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)]:
        n, t = positioned[pos]
        r *= n
    print(r)


def image_of_positioned(positioned: Positioned) -> List[str]:
    size = isqrt(len(positioned))
    image = [''] * (size * 8)
    for row in range(size):
        for col in range(size):
            name, tile = positioned[row, col]
            for i, r in enumerate(tile[1:-1]):
                image[row * 8 + i] += r[1:-1]
    return image


def puzzle2(positioned: Positioned) -> None:
    image = image_of_positioned(positioned)

    monster = [
        '                  # ',
        '#    ##    ##    ###',
        ' #  #  #  #  #  #   ',
    ]
    part_of_monster = set()
    for m in transforms(monster):
        for row in range(len(image) - len(m) + 1):
            for col in range(len(image[0]) - len(m[0]) + 1):
                if all(
                    m[r][c] != '#' or image[row + r][col + c] == '#'
                    for r in range(len(m))
                    for c in range(len(m[0]))
                ):
                    part_of_monster |= {
                        (row + r, col + c)
                        for r in range(len(m))
                        for c in range(len(m[0]))
                        if m[r][c] == '#'
                    }

    all_hashes = {
        (row, col)
        for row in range(len(image))
        for col in range(len(image))
        if image[row][col] == '#'
    }

    print(len(all_hashes - part_of_monster))


def main() -> None:
    positioned = read_image()
    puzzle1(positioned)
    puzzle2(positioned)


if __name__ == '__main__':
    main()
