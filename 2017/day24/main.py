from typing import Set, Tuple

Component = Tuple[int, int]


def read_components(filename: str) -> Set[Component]:
    components = set()
    with open(filename) as f:
        for line in f:
            a, b = line.strip().split('/')
            components.add((int(a), int(b)))
    return components


def strongest_bridge(components: Set[Component]) -> int:
    def aux(current_port: int) -> int:
        # always use identity component if possible
        component = (current_port, current_port)
        if component in components:
            components.remove(component)
            best = aux(current_port) + current_port + current_port
            components.add(component)
            return best
        # try other components
        best = 0
        for component in list(components):
            a, b = component
            if a == current_port:
                components.remove(component)
                best = max(best, aux(b) + a + b)
                components.add(component)
            if b == current_port:
                components.remove(component)
                best = max(best, aux(a) + a + b)
                components.add(component)
        return best
    return aux(0)


def longest_bridge(components: Set[Component]) -> Tuple[int, int]:
    def aux(current_port: int) -> Tuple[int, int]:
        # always use identity component if possible
        component = (current_port, current_port)
        if component in components:
            components.remove(component)
            length, strength = aux(current_port)
            length += 1
            strength += current_port + current_port
            components.add(component)
            return length, strength
        # try other components
        best = 0, 0
        for component in list(components):
            a, b = component
            if a == current_port:
                components.remove(component)
                length, strength = aux(b)
                length += 1
                strength += a + b
                best = max(best, (length, strength))
                components.add(component)
            if b == current_port:
                components.remove(component)
                length, strength = aux(a)
                length += 1
                strength += a + b
                best = max(best, (length, strength))
                components.add(component)
        return best
    return aux(0)


def main() -> None:
    example = read_components('example')
    input = read_components('input')

    assert strongest_bridge(example) == 31
    assert strongest_bridge(input) == 1859

    assert longest_bridge(example) == (4, 19)
    assert longest_bridge(input) == (35, 1799)


if __name__ == '__main__':
    main()
