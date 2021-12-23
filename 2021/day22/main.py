import re
from typing import List, NamedTuple, Optional, Set, Tuple


class Cuboid(NamedTuple):
    start_x: int
    start_y: int
    start_z: int
    stop_x: int
    stop_y: int
    stop_z: int

    def __repr__(self) -> str:
        parts = []
        if self.stop_x == self.start_x + 1:
            parts.append(f'{self.start_x}')
        else:
            parts.append(f'{self.start_x}..{self.stop_x - 1}')
        if self.stop_y == self.start_y + 1:
            parts.append(f'{self.start_y}')
        else:
            parts.append(f'{self.start_y}..{self.stop_y - 1}')
        if self.stop_z == self.start_z + 1:
            parts.append(f'{self.start_z}')
        else:
            parts.append(f'{self.start_z}..{self.stop_z - 1}')
        return '×'.join(parts)

    def volume(self) -> int:
        dx = self.stop_x - self.start_x
        dy = self.stop_y - self.start_y
        dz = self.stop_z - self.start_z
        return dx * dy * dz

    def __contains__(self, other: 'Cuboid') -> bool:  # type: ignore
        if other.start_x < self.start_x:
            return False
        if other.start_y < self.start_y:
            return False
        if other.start_z < self.start_z:
            return False
        if other.stop_x > self.stop_x:
            return False
        if other.stop_y > self.stop_y:
            return False
        if other.stop_z > self.stop_z:
            return False
        return True

    def is_outside(self, other: 'Cuboid') -> bool:
        if self.stop_x <= other.start_x:
            return True
        if self.stop_y <= other.start_y:
            return True
        if self.stop_z <= other.start_z:
            return True
        if other.stop_x <= self.start_x:
            return True
        if other.stop_y <= self.start_y:
            return True
        if other.stop_z <= self.start_z:
            return True
        return False

    def intersect(self, other: 'Cuboid') -> Optional['Cuboid']:
        if self.is_outside(other):
            return None
        return Cuboid(
            start_x=max(self.start_x, other.start_x),
            start_y=max(self.start_y, other.start_y),
            start_z=max(self.start_z, other.start_z),
            stop_x=min(self.stop_x, other.stop_x),
            stop_y=min(self.stop_y, other.stop_y),
            stop_z=min(self.stop_z, other.stop_z),
        )

    def subtract(self, b: 'Cuboid') -> List['Cuboid']:
        if self.is_outside(b):
            return [self]
        if self in b:
            return []
        cuboids = []

        if self.start_x < b.start_x:
            cuboids.append(Cuboid(
                start_x=self.start_x,
                start_y=self.start_y,
                start_z=self.start_z,
                stop_x=b.start_x,
                stop_y=self.stop_y,
                stop_z=self.stop_z,
            ))

        if b.stop_x < self.stop_x:
            cuboids.append(Cuboid(
                start_x=b.stop_x,
                start_y=self.start_y,
                start_z=self.start_z,
                stop_x=self.stop_x,
                stop_y=self.stop_y,
                stop_z=self.stop_z,
            ))

        if self.start_y < b.start_y:
            cuboids.append(Cuboid(
                start_x=max(self.start_x, b.start_x),
                start_y=self.start_y,
                start_z=self.start_z,
                stop_x=min(self.stop_x, b.stop_x),
                stop_y=b.start_y,
                stop_z=self.stop_z,
            ))

        if b.stop_y < self.stop_y:
            cuboids.append(Cuboid(
                start_x=max(self.start_x, b.start_x),
                start_y=b.stop_y,
                start_z=self.start_z,
                stop_x=min(self.stop_x, b.stop_x),
                stop_y=self.stop_y,
                stop_z=self.stop_z,
            ))

        if self.start_z < b.start_z:
            cuboids.append(Cuboid(
                start_x=max(self.start_x, b.start_x),
                start_y=max(self.start_y, b.start_y),
                start_z=self.start_z,
                stop_x=min(self.stop_x, b.stop_x),
                stop_y=min(self.stop_y, b.stop_y),
                stop_z=b.start_z,
            ))

        if b.stop_z < self.stop_z:
            cuboids.append(Cuboid(
                start_x=max(self.start_x, b.start_x),
                start_y=max(self.start_y, b.start_y),
                start_z=b.stop_z,
                stop_x=min(self.stop_x, b.stop_x),
                stop_y=min(self.stop_y, b.stop_y),
                stop_z=self.stop_z,
            ))

        return cuboids


a = Cuboid(10, 10, 10, 30, 30, 30)
b = Cuboid(15, 15, 15, 25, 25, 25)
assert a.volume() - b.volume() == sum(c.volume() for c in a.subtract(b))


Steps = List[Tuple[Cuboid, bool]]
Vec3 = Tuple[int, int, int]
Cubes = Set[Vec3]


cuboid_pattern = re.compile(
    r'^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$',
)


def read_steps(filename: str) -> Steps:
    with open(filename) as f:
        cuboids = []
        for line in f:
            m = cuboid_pattern.match(line.strip())
            assert m is not None
            on = m.group(1) == 'on'
            min_x, max_x, min_y, max_y, min_z, max_z = (int(m.group(i)) for i in range(2, 8))
            cuboids.append((Cuboid(
                start_x=min_x,
                start_y=min_y,
                start_z=min_z,
                stop_x=max_x + 1,
                stop_y=max_y + 1,
                stop_z=max_z + 1,
            ), on))
        return cuboids


def count_cubes(steps: Steps, initialization_area_only: bool = False) -> int:
    initialization_area = Cuboid(-50, -50, -50, 51, 51, 51)
    on_cuboids: Set[Cuboid] = set()
    for target, on in steps:
        if initialization_area_only:
            new_target = target.intersect(initialization_area)
            if new_target is None:
                continue
            target = new_target
        on_cuboids = {
            c
            for cuboid in on_cuboids
            for c in cuboid.subtract(target)
        }
        if on:
            on_cuboids.add(target)
    return sum(cuboid.volume() for cuboid in on_cuboids)


def main() -> None:
    example1 = read_steps('example1')
    example2 = read_steps('example2')
    example3 = read_steps('example3')
    input = read_steps('input')

    assert count_cubes(example1, True) == 39
    assert count_cubes(example2, True) == 590784
    assert count_cubes(input, True) == 620241
    assert count_cubes(example3) == 2758514936282235
    assert count_cubes(input) == 1284561759639324


if __name__ == '__main__':
    main()
