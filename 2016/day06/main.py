from collections import Counter
from typing import List


def read_messages(filename: str) -> List[str]:
    with open(filename) as f:
        return [
            line.strip()
            for line in f
        ]


def most_common(messages: List[str]) -> str:
    n = len(messages[0])
    assert all(len(message) == n for message in messages)
    ret = []
    for i in range(n):
        (c, _), = Counter(message[i] for message in messages).most_common(1)
        ret.append(c)
    return ''.join(ret)


def least_common(messages: List[str]) -> str:
    n = len(messages[0])
    assert all(len(message) == n for message in messages)
    ret = []
    for i in range(n):
        c, _ = list(Counter(message[i] for message in messages).most_common())[-1]
        ret.append(c)
    return ''.join(ret)


def main() -> None:
    example = read_messages('example')
    input = read_messages('input')

    assert most_common(example) == 'easter'
    assert most_common(input) == 'tzstqsua'

    assert least_common(example) == 'advent'
    assert least_common(input) == 'myregdnr'


if __name__ == '__main__':
    main()
