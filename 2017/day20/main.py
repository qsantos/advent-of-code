from math import sqrt
from typing import Dict, List, Set, Tuple

Vec3 = Tuple[int, int, int]
Particle = Tuple[Vec3, Vec3, Vec3]


def parse_vec3(s: str) -> Vec3:
    x, y, z = s[len('X=<'):].split(',')
    return int(x), int(y), int(z)


def parse_particle(s: str) -> Particle:
    p, v, a = s[:-len('>')].split('>, ')
    return parse_vec3(p), parse_vec3(v), parse_vec3(a)


def read_particles(filename: str) -> List[Particle]:
    with open(filename) as f:
        return [
            parse_particle(line.strip())
            for line in f
        ]


def closest_to_origin(particles: List[Particle]) -> int:
    def key(i: int) -> int:
        p, v, a = particles[i]
        x, y, z = a
        return abs(x) + abs(y) + abs(z)

    return min(range(len(particles)), key=key)


def position_at_time(particle: Particle, time: int) -> Vec3:
    p, v, a = particle
    px, py, pz = tuple(
        px + vx * time + ax * time * (time + 1) // 2
        for px, vx, ax in zip(p, v, a)
    )
    return px, py, pz


def clear_collisions(particles: List[Particle]) -> int:
    incoming_collisions: Dict[int, Set[Tuple[int, int]]] = {}
    n = len(particles)
    for i in range(n):
        for j in range(i + 1, n):
            a = particles[i]
            b = particles[j]
            Δp0 = b[0][0] - a[0][0]
            Δv0 = b[1][0] - a[1][0]
            Δa0 = b[2][0] - a[2][0]
            # Δp(t) = Δp0 + Δv0 * t + Δa0 * t (t + 1) / 2
            #       = Δp0 + t * (Δv0 + Δa0 / 2) + t² * Δa0 / 2
            A = Δa0 / 2
            B = Δv0 + Δa0 / 2
            C = Δp0
            if A == 0:
                if B == 0:
                    continue
                candidate_times = [round(-C / B)]
            else:
                Δ = B * B - 4 * A * C
                if Δ < 0:
                    continue
                sΔ = sqrt(Δ)
                candidate_times = [round((-B - sΔ) / (2 * A)), round((-B + sΔ) / (2 * A))]
            for t in candidate_times:
                if position_at_time(a, t) == position_at_time(b, t):
                    if t not in incoming_collisions:
                        incoming_collisions[t] = set()
                    incoming_collisions[t].add((i, j))
    collided = set()
    for t in sorted(incoming_collisions):
        new_collisions = set()
        for i, j in incoming_collisions[t]:
            if i not in collided and j not in collided:
                new_collisions.add(i)
                new_collisions.add(j)
        collided |= new_collisions
    return len(particles) - len(collided)


def main() -> None:
    example1 = read_particles('example1')
    example2 = read_particles('example2')
    input = read_particles('input')

    assert closest_to_origin(example1) == 0
    assert closest_to_origin(input) == 125

    assert clear_collisions(example2) == 1
    assert clear_collisions(input) == 461


if __name__ == '__main__':
    main()
