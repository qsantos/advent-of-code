from typing import Dict, List

Rules = Dict[str, str]
Pattern = List[List[str]]


def rotate90(rows: List[str]) -> List[str]:
    n = len(rows)
    assert all(len(row) == n for row in rows)
    return [
        ''.join(
            rows[n - 1 - j][i]
            for j in range(n)
        )
        for i in range(n)
    ]


def read_rules(filename: str) -> Rules:
    rules = {}
    with open(filename) as f:
        for line in f:
            pattern, result = line.split(' => ')
            rows = pattern.split('/')
            rules[''.join(rows)] = result
            rules[''.join(reversed(rows))] = result
            rows = rotate90(rows)
            rules[''.join(rows)] = result
            rules[''.join(reversed(rows))] = result
            rows = rotate90(rows)
            rules[''.join(rows)] = result
            rules[''.join(reversed(rows))] = result
            rows = rotate90(rows)
            rules[''.join(rows)] = result
            rules[''.join(reversed(rows))] = result
        pass
    return rules


def apply_rules(rules: Rules, pattern: Pattern) -> Pattern:
    n = len(pattern)
    if n % 2 == 0:
        s = 2
    elif n % 3 == 0:
        s = 3
    else:
        assert False
    new_s = s + 1
    new_n = n // s * new_s
    new_pattern = [[''] * new_n for _ in range(new_n)]
    for si in range(0, n // s):
        for sj in range(0, n // s):
            part = ''.join(
                pattern[si * s + oi][sj * s + oj]
                for oi in range(s)
                for oj in range(s)
            )
            new_rows = rules[part].split('/')
            for oi in range(new_s):
                for oj in range(new_s):
                    new_pattern[si * new_s + oi][sj * new_s + oj] = new_rows[oi][oj]
    return new_pattern


def repeat_rules(rules: Rules, n_rounds: int) -> int:
    pattern = [
        list('.#.'),
        list('..#'),
        list('###'),
    ]
    for _ in range(n_rounds):
        pattern = apply_rules(rules, pattern)
        # for row in pattern:
        #     print(''.join(row))
        # print()
    return sum(row.count('#') for row in pattern)


def main() -> None:
    example = read_rules('example')
    input = read_rules('input')

    assert repeat_rules(example, 2) == 12
    assert repeat_rules(input, 5) == 150
    assert repeat_rules(input, 18) == 2606275


if __name__ == '__main__':
    main()
