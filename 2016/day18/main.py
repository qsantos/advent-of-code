from typing import List


def next_step(s: str) -> str:
    s = '.' + s + '.'
    return ''.join(
        '^' if s[i - 1] + s[i] + s[i + 1] in ['^^.', '.^^', '^..', '..^'] else '.'
        for i in range(1, len(s) - 1)
    )


def make_grid(first_row: str, n_rows: int) -> List[str]:
    ret = [first_row]
    row = first_row
    for _ in range(1, n_rows):
        row = next_step(row)
        ret.append(row)
    return ret


def count_safe(first_row: str, n_rows: int) -> int:
    row = first_row
    ret = row.count('.')
    for _ in range(1, n_rows):
        row = next_step(row)
        ret += row.count('.')
    return ret


def main() -> None:
    input = (
        '.^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...'
        '^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^'
    )

    assert next_step('..^^.') == '.^^^^'
    assert next_step('.^^^^') == '^^..^'

    assert make_grid('.^^.^.^^^^', 10) == [
        '.^^.^.^^^^',
        '^^^...^..^',
        '^.^^.^.^^.',
        '..^^...^^^',
        '.^^^^.^^.^',
        '^^..^.^^..',
        '^^^^..^^^.',
        '^..^^^^.^^',
        '.^^^..^.^^',
        '^^.^^^..^^',
    ]

    grid = make_grid(input, 40)
    assert sum(row.count('.') for row in grid) == 1939
    assert count_safe(input, 40) == 1939
    assert count_safe(input, 400_000) == 19999535


if __name__ == '__main__':
    main()
