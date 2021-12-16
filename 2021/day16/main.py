from math import prod
from typing import Tuple


def read_info(filename: str) -> str:
    with open(filename) as f:
        return f.read().strip()


def read_packet(bits: str, i: int, j: int, just_sum_versions: bool = False) -> Tuple[int, int]:
    assert j >= i + 3
    version = int(bits[i:i + 3], 2)
    i += 3
    assert j >= i + 3
    type_id = int(bits[i:i + 3], 2)
    i += 3
    if type_id == 4:
        groups = []
        while i < j:
            assert j >= i + 5
            continuation_bit = bits[i]
            group = bits[i + 1: i + 5]
            i += 5
            groups.append(group)
            if continuation_bit == '0':
                break
        if just_sum_versions:
            return i, version
        else:
            value = int(''.join(groups), 2)
            return i, value
    else:
        assert j >= i + 1
        length_type_id = bits[i]
        i += 1
        if length_type_id == '0':
            assert j >= i + 15
            total_length_in_bits = int(bits[i:i + 15], 2)
            i += 15
            values = []
            j = i + total_length_in_bits
            while i < j:
                i, value = read_packet(bits, i, j, just_sum_versions=just_sum_versions)
                values.append(value)
        else:
            assert j >= i + 11
            number_of_subpackets = int(bits[i:i + 11], 2)
            i += 11
            values = []
            for _ in range(number_of_subpackets):
                i, value = read_packet(bits, i, j, just_sum_versions=just_sum_versions)
                values.append(value)
        if just_sum_versions:
            return i, version + sum(values)
        if type_id == 0:
            return i, sum(values)
        elif type_id == 1:
            return i, prod(values)
        elif type_id == 2:
            return i, min(values)
        elif type_id == 3:
            return i, max(values)
        elif type_id == 5:
            first, second, *others = values
            return i, 1 if first > second else 0
        elif type_id == 6:
            first, second, *others = values
            return i, 1 if first < second else 0
        elif type_id == 7:
            first, second, *others = values
            return i, 1 if first == second else 0
        else:
            assert False, type_id


def decode(data: str, *, just_sum_versions: bool = False) -> int:
    bits = f'{int(data, 16):0{4 * len(data)}b}'
    _, v = read_packet(bits, 0, len(bits), just_sum_versions=just_sum_versions)
    return v


def main() -> None:
    input = read_info('input')

    assert decode('D2FE28', just_sum_versions=True) == 6
    assert decode('38006F45291200', just_sum_versions=True) == 9
    assert decode('8A004A801A8002F478', just_sum_versions=True) == 16
    assert decode('620080001611562C8802118E34', just_sum_versions=True) == 12
    assert decode('C0015000016115A2E0802F182340', just_sum_versions=True) == 23
    assert decode('A0016C880162017C3686B18A3D4780', just_sum_versions=True) == 31
    assert decode(input, just_sum_versions=True) == 963

    assert decode('C200B40A82') == 3
    assert decode('04005AC33890') == 54
    assert decode('880086C3E88112') == 7
    assert decode('CE00C43D881120') == 9
    assert decode('D8005AC2A8F0') == 1
    assert decode('F600BC2D8F') == 0
    assert decode('9C005AC2F8F0') == 0
    assert decode('9C0141080250320F1802104A08') == 1
    assert decode(input) == 1549026292886


if __name__ == '__main__':
    main()
