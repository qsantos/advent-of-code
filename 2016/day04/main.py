from collections import Counter
from string import ascii_lowercase
from typing import List, Tuple

Room = Tuple[str, int, str]
Rooms = List[Room]


def caesar(s: str, shift: int) -> str:
    return ''.join(
        ascii_lowercase[(ascii_lowercase.index(c) + shift) % len(ascii_lowercase)]
        if c in ascii_lowercase else c
        for c in s
    )


def parse_room(s: str) -> Room:
    name, rest = s.rsplit('-', 1)
    sector_id, checksum = rest[:-len(']')].split('[')
    return name, int(sector_id), checksum


def read_rooms(filename: str) -> Rooms:
    with open(filename) as f:
        return [
            parse_room(line.strip())
            for line in f
        ]


def is_real_room(room: Room) -> bool:
    name, sector_id, checksum = room
    letters = sorted((-count, c) for c, count in Counter(name.replace('-', '')).items())
    return checksum == ''.join(c for count, c in letters[:5])


def sum_of_sector_ids(rooms: Rooms) -> int:
    return sum(
        room[1]
        for room in rooms
        if is_real_room(room)
    )


def find_north_pole_object_storage(rooms: Rooms) -> int:
    for room in rooms:
        if not is_real_room(room):
            continue
        name, sector_id, checksum = room
        if caesar(name, sector_id) == 'northpole-object-storage':
            return sector_id
    assert False


def main() -> None:
    input = read_rooms('input')

    assert is_real_room(parse_room('aaaaa-bbb-z-y-x-123[abxyz]'))
    assert is_real_room(parse_room('a-b-c-d-e-f-g-h-987[abcde]'))
    assert is_real_room(parse_room('not-a-real-room-404[oarel]'))
    assert not is_real_room(parse_room('totally-real-room-200[decoy]'))

    assert sum_of_sector_ids(input) == 158835
    assert find_north_pole_object_storage(input) == 993


if __name__ == '__main__':
    main()
