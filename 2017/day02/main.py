from typing import List

Spreadsheet = List[List[int]]


def read_spreadsheet(filename: str) -> Spreadsheet:
    with open(filename) as f:
        return [
            [int(x) for x in line.strip().split()]
            for line in f
        ]


def sum_spreads(spreadsheet: Spreadsheet) -> int:
    return sum(
        max(row) - min(row)
        for row in spreadsheet
    )


def sum_quotients(spreadsheet: Spreadsheet) -> int:
    return sum(
        row[i] // row[j]
        for row in spreadsheet
        for i in range(len(row))
        for j in range(len(row))
        if i != j
        if row[i] % row[j] == 0
    )


def main() -> None:
    example1 = read_spreadsheet('example1')
    example2 = read_spreadsheet('example2')
    input = read_spreadsheet('input')

    assert sum_spreads(example1) == 18
    assert sum_spreads(input) == 36174

    assert sum_quotients(example2) == 9
    assert sum_quotients(input) == 244


if __name__ == '__main__':
    main()
