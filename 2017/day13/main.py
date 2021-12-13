from typing import List, Tuple

Layers = List[Tuple[int, int]]


def read_layers(filename: str) -> Layers:
    layers = []
    with open(filename) as f:
        for line in f:
            left, right = line.strip().split(': ')
            layers.append((int(left), int(right)))
    return layers


def severity(layers: Layers) -> int:
    total = 0
    for depth, length in layers:
        cycle = 2 * length - 2
        if depth % cycle == 0:
            total += depth * length
    return total


def min_delay(layers: Layers) -> int:
    depth_cycles = [
        (depth, 2 * length - 2)
        for depth, length in layers
    ]
    delay = 10
    while True:
        if all((depth + delay) % cycle != 0 for depth, cycle in depth_cycles):
            return delay
        delay += 1
    assert False


def main() -> None:
    example = read_layers('example')
    input = read_layers('input')

    assert severity(example) == 24
    assert severity(input) == 2160

    assert min_delay(input) == 3907470


if __name__ == '__main__':
    main()
