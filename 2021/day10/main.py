from typing import List, Tuple


def read_lines(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


closing_of_opening = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}

score_of_corrupted_closing = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

score_of_missing_closing = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}


def line_errors(line: str) -> Tuple[List[str], int]:
    closings = []
    for c in line:
        if c in closing_of_opening:
            closings.append(closing_of_opening[c])
        elif c == closings.pop():
            pass
        else:
            return [], score_of_corrupted_closing[c]
    return closings, 0


def score_corrupted_line(line: str) -> int:
    _, corrupted_score = line_errors(line)
    return corrupted_score


def score_corrupted_lines(lines: List[str]) -> int:
    return sum(
        score_corrupted_line(line)
        for line in lines
    )


def score_incomplete_line(line: str) -> int:
    missing, _ = line_errors(line)
    total = 0
    for c in reversed(missing):
        total *= 5
        total += score_of_missing_closing[c]
    return total


def score_incomplete_lines(lines: List[str]) -> int:
    scores = [
        score_incomplete_line(line)
        for line in lines
    ]
    scores = sorted(
        score
        for score in scores
        if score
    )
    return scores[len(scores) // 2]


example = read_lines('example')
input = read_lines('input')


assert score_corrupted_lines(example) == 26397
assert score_corrupted_lines(input) == 366027

assert score_incomplete_line('[({(<(())[]>[[{[]{<()<>>') == 288957
assert score_incomplete_line('[(()[<>])]({[<{<<[]>>(') == 5566
assert score_incomplete_line('(((({<>}<{<{<>}{[]{[]{}') == 1480781
assert score_incomplete_line('{<[[]]>}<{[{[{[]{()[[[]') == 995444
assert score_incomplete_line('<{([{{}}[<[[[<>{}]]]>[]]') == 294

assert score_incomplete_lines(example) == 288957
assert score_incomplete_lines(input) == 1118645287
