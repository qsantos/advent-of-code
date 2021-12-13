from typing import Tuple


def read_stream(filename: str) -> str:
    with open(filename) as f:
        return f.read().strip()


def parse_stream(stream: str) -> Tuple[int, int]:
    score = 0
    depth = 0
    garbage = 0
    it = iter(stream)
    for c in it:
        if c == '!':
            next(it)
        elif c == '<':
            for c in it:
                if c == '!':
                    next(it)
                elif c == '>':
                    break
                else:
                    garbage += 1
        elif c == '{':
            depth += 1
            score += depth
        elif c == '}':
            depth -= 1
    return score, garbage


def total_score(stream: str) -> int:
    score, garbage = parse_stream(stream)
    return score


def total_garbage(stream: str) -> int:
    score, garbage = parse_stream(stream)
    return garbage


def main() -> None:
    input = read_stream('input')

    assert total_score('{}') == 1
    assert total_score('{{{}}}') == 6
    assert total_score('{{},{}}') == 5
    assert total_score('{{{},{},{{}}}}') == 16
    assert total_score('{<a>,<a>,<a>,<a>}') == 1
    assert total_score('{{<ab>},{<ab>},{<ab>},{<ab>}}') == 9
    assert total_score('{{<!!>},{<!!>},{<!!>},{<!!>}}') == 9
    assert total_score('{{<a!>},{<a!>},{<a!>},{<ab>}}') == 3
    assert total_score(input) == 14421

    assert total_garbage('<>') == 0
    assert total_garbage('<random characters>') == 17
    assert total_garbage('<<<<>') == 3
    assert total_garbage('<{!>}>') == 2
    assert total_garbage('<!!>') == 0
    assert total_garbage('<!!!>>') == 0
    assert total_garbage('<{o"i!a,<{i<a>') == 10
    assert total_garbage(input) == 6817


if __name__ == '__main__':
    main()
