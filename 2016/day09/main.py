def read_file(filename: str) -> str:
    with open(filename) as f:
        return f.read().strip()


def decompress_data1(data: str) -> str:
    output = []
    i = 0
    while True:
        prev_i = i
        try:
            i = data.index('(', prev_i)
        except ValueError:
            output.append(data[prev_i:])
            break
        output.append(data[prev_i:i])
        j = data.index('x', i)
        k = data.index(')', j)
        a = int(data[i + 1:j])
        b = int(data[j + 1:k])
        i = k + 1 + a
        segment = data[k + 1:i]
        output.append(segment * b)
    return ''.join(output)


def decompress_data2(data: str) -> int:
    def aux(i: int, j: int) -> int:
        try:
            idx_op = data.index('(', i, j)
        except ValueError:
            return j - i
        idx_x = data.index('x', idx_op)
        idx_cp = data.index(')', idx_x)
        a = int(data[idx_op + 1:idx_x])
        b = int(data[idx_x + 1:idx_cp])
        c = idx_cp + 1 + a
        before = idx_op - i
        expanded = b * aux(idx_cp + 1, c)
        after = aux(c, j)
        return before + expanded + after
    return aux(0, len(data))


def main() -> None:
    input = read_file('input')

    assert decompress_data1('ADVENT') == 'ADVENT'
    assert decompress_data1('A(1x5)BC') == 'ABBBBBC'
    assert decompress_data1('(3x3)XYZ') == 'XYZXYZXYZ'
    assert decompress_data1('A(2x2)BCD(2x2)EFG') == 'ABCBCDEFEFG'
    assert decompress_data1('(6x1)(1x3)A') == '(1x3)A'
    assert decompress_data1('X(8x2)(3x3)ABCY') == 'X(3x3)ABC(3x3)ABCY'
    assert len(decompress_data1(input)) == 110346

    assert decompress_data2('(3x3)XYZ') == len('XYZXYZXYZ')
    assert decompress_data2('X(8x2)(3x3)ABCY') == len('XABCABCABCABCABCABCY')
    assert decompress_data2('A(2x2)BCD(2x2)EFG') == len('ABCBCDEFEFG')
    assert decompress_data2('(27x12)(20x12)(13x14)(7x10)(1x12)A') == 241920
    assert decompress_data2('(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN') == 445
    assert decompress_data2(input) == 10774309173


if __name__ == '__main__':
    main()
