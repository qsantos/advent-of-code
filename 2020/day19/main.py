from typing import Dict, List, Tuple

Rules = Dict[str, List[List[str]]]


def parse_input() -> Tuple[Rules, List[str]]:
    with open('input') as f:
        rules_str, messages_str = f.read().split('\n\n')

    rules = {}
    for rule in rules_str.split('\n'):
        name, options_str = rule.split(': ', 1)
        options = []
        for option_str in options_str.split(' | '):
            options.append(option_str.split(' '))
        options.sort(key=list.__len__, reverse=True)
        rules[name] = options

    messages = messages_str.split('\n')
    return rules, messages


def match1(rules: Rules, message: str) -> bool:
    def match_option(option: List[str], offset: int) -> Tuple[bool, int]:
        for clause in option:
            if clause[0] == '"':
                if offset >= len(message) or message[offset] != clause[1]:
                    return False, 0
                offset += 1
            else:
                ok, offset = match_rule(clause, offset)
                if not ok:
                    return False, 0
        return True, offset

    def match_rule(name: str, offset: int) -> Tuple[bool, int]:
        for option in rules[name]:
            ok, new_offset = match_option(option, offset)
            if ok:
                return True, new_offset
        return False, 0

    ok, offset = match_rule('0', 0)
    return ok and offset == len(message)


def puzzle1(rules: Rules, messages: List[str]) -> None:
    print(sum(
        match1(rules, message)
        for message in messages
    ))


def match2(rules: Rules, message: str) -> bool:
    def match_at(clauses: List[str], offset: int) -> bool:
        if not clauses:
            return offset == len(message)
        if offset >= len(message):
            return False
        clauses = list(clauses)
        clause = clauses.pop()
        if clause[0] == '"':
            if message[offset] != clause[1]:
                return False
            return match_at(clauses, offset + 1)

        for option in rules[clause]:
            if match_at(clauses + option[::-1], offset):
                return True
        return False

    return match_at(['0'], 0)


def puzzle2(rules: Rules, messages: List[str]) -> None:
    rules['8'] = [
        ['42'],
        ['42', '8'],
    ]
    rules['11'] = [
        ['42', '31'],
        ['42', '11', '31'],
    ]

    print(sum(
        match2(rules, message)
        for message in messages
    ))


def main() -> None:
    rules, messages = parse_input()
    puzzle1(rules, messages)
    puzzle2(rules, messages)


if __name__ == '__main__':
    main()
