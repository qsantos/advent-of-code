import re
from typing import List, NamedTuple, Set, Tuple


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

    def subtract(self, b: 'Cuboid') -> List['Cuboid']:
        if self.is_outside(b):
            return [self]
        if self in b:
            return []
        parts_x = [
            (self.start_x, b.start_x),
            (max(self.start_x, b.start_x), min(b.stop_x, self.stop_x)),
            (b.stop_x, self.stop_x),
        ]
        parts_y = [
            (self.start_y, b.start_y),
            (max(self.start_y, b.start_y), min(b.stop_y, self.stop_y)),
            (b.stop_y, self.stop_y),
        ]
        parts_z = [
            (self.start_z, b.start_z),
            (max(self.start_z, b.start_z), min(b.stop_z, self.stop_z)),
            (b.stop_z, self.stop_z),
        ]
        cuboids = []
        for xi in range(3):
            start_x, stop_x = parts_x[xi]
            if not start_x < stop_x:
                continue
            for yi in range(3):
                start_y, stop_y = parts_y[yi]
                if not start_y < stop_y:
                    continue
                for zi in range(3):
                    if (xi, yi, zi) == (1, 1, 1):
                        continue
                    start_z, stop_z = parts_z[zi]
                    if not start_z < stop_z:
                        continue
                    cuboids.append(Cuboid(
                        start_x=start_x,
                        start_y=start_y,
                        start_z=start_z,
                        stop_x=stop_x,
                        stop_y=stop_y,
                        stop_z=stop_z,
                    ))
        return cuboids


a = Cuboid(10, 10, 10, 30, 30, 30)
b = Cuboid(15, 15, 15, 25, 25, 25)
d = [
    Cuboid(start_x=10, start_y=10, start_z=10, stop_x=15, stop_y=15, stop_z=15),
    Cuboid(start_x=10, start_y=10, start_z=15, stop_x=15, stop_y=15, stop_z=25),
    Cuboid(start_x=10, start_y=10, start_z=25, stop_x=15, stop_y=15, stop_z=30),
    Cuboid(start_x=10, start_y=15, start_z=10, stop_x=15, stop_y=25, stop_z=15),
    Cuboid(start_x=10, start_y=15, start_z=15, stop_x=15, stop_y=25, stop_z=25),
    Cuboid(start_x=10, start_y=15, start_z=25, stop_x=15, stop_y=25, stop_z=30),
    Cuboid(start_x=10, start_y=25, start_z=10, stop_x=15, stop_y=30, stop_z=15),
    Cuboid(start_x=10, start_y=25, start_z=15, stop_x=15, stop_y=30, stop_z=25),
    Cuboid(start_x=10, start_y=25, start_z=25, stop_x=15, stop_y=30, stop_z=30),
    Cuboid(start_x=15, start_y=10, start_z=10, stop_x=25, stop_y=15, stop_z=15),
    Cuboid(start_x=15, start_y=10, start_z=15, stop_x=25, stop_y=15, stop_z=25),
    Cuboid(start_x=15, start_y=10, start_z=25, stop_x=25, stop_y=15, stop_z=30),
    Cuboid(start_x=15, start_y=15, start_z=10, stop_x=25, stop_y=25, stop_z=15),
    Cuboid(start_x=15, start_y=15, start_z=25, stop_x=25, stop_y=25, stop_z=30),
    Cuboid(start_x=15, start_y=25, start_z=10, stop_x=25, stop_y=30, stop_z=15),
    Cuboid(start_x=15, start_y=25, start_z=15, stop_x=25, stop_y=30, stop_z=25),
    Cuboid(start_x=15, start_y=25, start_z=25, stop_x=25, stop_y=30, stop_z=30),
    Cuboid(start_x=25, start_y=10, start_z=10, stop_x=30, stop_y=15, stop_z=15),
    Cuboid(start_x=25, start_y=10, start_z=15, stop_x=30, stop_y=15, stop_z=25),
    Cuboid(start_x=25, start_y=10, start_z=25, stop_x=30, stop_y=15, stop_z=30),
    Cuboid(start_x=25, start_y=15, start_z=10, stop_x=30, stop_y=25, stop_z=15),
    Cuboid(start_x=25, start_y=15, start_z=15, stop_x=30, stop_y=25, stop_z=25),
    Cuboid(start_x=25, start_y=15, start_z=25, stop_x=30, stop_y=25, stop_z=30),
    Cuboid(start_x=25, start_y=25, start_z=10, stop_x=30, stop_y=30, stop_z=15),
    Cuboid(start_x=25, start_y=25, start_z=15, stop_x=30, stop_y=30, stop_z=25),
    Cuboid(start_x=25, start_y=25, start_z=25, stop_x=30, stop_y=30, stop_z=30),
]
assert a.subtract(b) == d
assert a.volume() - b.volume() == sum(c.volume() for c in d)


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


def count_cubes_in_initialization_area(steps: Steps) -> int:
    cubes = set()
    for cuboid, on in steps:
        for x in range(max(-50, cuboid.start_x), min(+51, cuboid.stop_x)):
            for y in range(max(-50, cuboid.start_y), min(+51, cuboid.stop_y)):
                for z in range(max(-50, cuboid.start_z), min(+51, cuboid.stop_z)):
                    if on:
                        cubes.add((x, y, z))
                    else:
                        cubes.discard((x, y, z))
    return len(cubes)


def count_cubes(steps: Steps) -> int:
    on_cuboids: Set[Cuboid] = set()
    for target, on in steps:
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

    assert count_cubes_in_initialization_area(example1) == 39
    assert count_cubes_in_initialization_area(example2) == 590784
    assert count_cubes_in_initialization_area(input) == 620241
    assert count_cubes(example3) == 2758514936282235
    assert count_cubes(input) == 1284561759639324


if __name__ == '__main__':
    main()
