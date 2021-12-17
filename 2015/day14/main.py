import re
from typing import NamedTuple


class Reindeer(NamedTuple):
    name: str
    speed: int
    time: int
    rest: int

    def distance_of_time(self, t: int) -> int:
        cycles, remaining = divmod(t, self.rest + self.time)
        remaining = min(remaining, self.time)
        return cycles * self.speed * self.time + remaining * self.speed


pattern = re.compile(
    r'^(\S+) can fly (\d+) km/s for (\d+) seconds?, but then must rest for (\d+) seconds?.$',
)

reindeers = []
with open('input') as f:
    for line in f:
        m = pattern.match(line.strip())
        assert m is not None
        name, speed, time, rest = m.groups()
        reindeers.append(Reindeer(
            name=name,
            speed=int(speed),
            time=int(time),
            rest=int(rest),
        ))


Comet = Reindeer(name='Comet', speed=14, time=10, rest=127)
Dancer = Reindeer(name='Dancer', speed=16, time=11, rest=162)

assert Comet.distance_of_time(0) == 0
assert Comet.distance_of_time(1) == 14
assert Comet.distance_of_time(2) == 28
assert Comet.distance_of_time(1000) == 1120
assert Dancer.distance_of_time(1000) == 1056

assert max(reindeer.distance_of_time(2503) for reindeer in reindeers) == 2640


points = [0] * len(reindeers)
for t in range(1, 2504):
    lead = max(reindeer.distance_of_time(t) for reindeer in reindeers)
    for i, reindeer in enumerate(reindeers):
        if reindeer.distance_of_time(t) == lead:
            points[i] += 1
assert max(points) == 1102
