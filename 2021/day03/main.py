from typing import List


def gamma_rate(numbers: List[str]) -> int:
    bits = []
    n_numbers = len(numbers)
    n_bits = len(numbers[0])
    for i in range(n_bits):
        ones = sum(number[i] == '1' for number in numbers)
        assert ones != n_numbers // 2
        bits.append('1' if ones * 2 > n_numbers else '0')
    return int(''.join(bits), 2)


def epsilon_rate(numbers: List[str]) -> int:
    bits = []
    n_numbers = len(numbers)
    n_bits = len(numbers[0])
    for i in range(n_bits):
        ones = sum(number[i] == '1' for number in numbers)
        assert ones != n_numbers // 2
        bits.append('0' if ones * 2 > n_numbers else '1')
    return int(''.join(bits), 2)


def oxygen_generator_rating(numbers: List[str]) -> int:
    numbers = list(numbers)
    pos = 0
    while len(numbers) > 1:
        ones = sum(number[pos] == '1' for number in numbers)
        if 2 * ones >= len(numbers):
            most_common = '1'
        else:
            most_common = '0'
        numbers = [number for number in numbers if number[pos] == most_common]
        pos += 1
    return int(numbers[0], 2)


def co2_scrubber_rating(numbers: List[str]) -> int:
    numbers = list(numbers)
    pos = 0
    while len(numbers) > 1:
        ones = sum(number[pos] == '1' for number in numbers)
        if 2 * ones >= len(numbers):
            most_common = '1'
        else:
            most_common = '0'
        numbers = [number for number in numbers if number[pos] != most_common]
        pos += 1
    return int(numbers[0], 2)


example = [
    '00100',
    '11110',
    '10110',
    '10111',
    '10101',
    '01111',
    '00111',
    '11100',
    '10000',
    '11001',
    '00010',
    '01010',
]

with open('input') as f:
    data = [
        line.strip()
        for line in f
    ]

assert gamma_rate(example) == 22
assert epsilon_rate(example) == 9
assert gamma_rate(data) == 2663
assert epsilon_rate(data) == 1432

assert oxygen_generator_rating(example) == 23
assert co2_scrubber_rating(example) == 10
assert oxygen_generator_rating(data) == 2526
assert co2_scrubber_rating(data) == 1184
