from typing import List


def read_layers(width: int, height: int, data: str) -> List[str]:
    n_layers, rem = divmod(len(data), width * height)
    assert rem == 0
    layers = [
        data[layer * height * width:(layer + 1) * height * width]
        for layer in range(n_layers)
    ]
    return layers


def combine_layers(layers: List[str]) -> str:
    return ''.join(
        next(value for value in pixel_values if value != '2')
        for pixel_values in zip(*layers)
    )


def read_image(width: int, height: int, data: str) -> List[str]:
    pixels = combine_layers(read_layers(width, height, data))
    return [
        pixels[row * width:(row + 1) * width]
        for row in range(height)
    ]


def main() -> None:
    assert read_layers(3, 2, '123456789012') == ['123456', '789012']

    with open('input') as f:
        data = f.read().strip()

    # puzzle 1
    layer = min(read_layers(25, 6, data), key=lambda layer: layer.count('0'))
    print(layer.count('1') * layer.count('2'))

    assert read_image(2, 2, '0222112222120000') == ['01', '10']
    for row in read_image(25, 6, data):
        print(''.join(row.replace('0', ' ')))


if __name__ == '__main__':
    main()
